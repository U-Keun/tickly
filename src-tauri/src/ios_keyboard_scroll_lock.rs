use std::{cell::RefCell, ptr::NonNull, sync::Arc};

use objc2::{
    define_class, msg_send, rc::Retained, runtime::ProtocolObject,
    DefinedClass, MainThreadMarker, MainThreadOnly,
};
use objc2_core_foundation::{CGPoint, CGRect};
use objc2_foundation::{
    NSNotification, NSNotificationCenter, NSNotificationName, NSObject, NSObjectProtocol,
};
use objc2_ui_kit::{
    UIKeyboardDidShowNotification, UIKeyboardWillHideNotification, UIKeyboardWillShowNotification,
    UIScrollView, UIScrollViewContentInsetAdjustmentBehavior, UIScrollViewDelegate, UIView,
};
use tauri::WebviewWindow;

thread_local! {
    // delegate가 살아있어야 하므로(ARC) thread_local로 보관
    static LOCK_DELEGATE: RefCell<Option<Retained<KeyboardScrollLockDelegate>>> = RefCell::new(None);
    // WKWebView의 원래 frame 저장
    static ORIGINAL_WEBVIEW_FRAME: RefCell<Option<CGRect>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct KeyboardScrollLockState {
    pub scroll_view: Arc<Retained<UIScrollView>>,
    pub offset: CGPoint,
}

define_class! {
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "KeyboardScrollLockDelegate"]
    #[ivars = KeyboardScrollLockState]
    pub(crate) struct KeyboardScrollLockDelegate;

    unsafe impl NSObjectProtocol for KeyboardScrollLockDelegate {}

    unsafe impl UIScrollViewDelegate for KeyboardScrollLockDelegate {
        // 바깥 스크롤이 움직이면 즉시 원래 위치로 되돌림
        #[unsafe(method(scrollViewDidScroll:))]
        unsafe fn scroll_view_did_scroll(&self, _scroll_view: &UIScrollView) {
            self.ivars().scroll_view.setContentOffset(self.ivars().offset);
        }
    }
}

impl KeyboardScrollLockDelegate {
    fn new(
        mtm: MainThreadMarker,
        scroll_view: Arc<Retained<UIScrollView>>,
        offset: CGPoint,
    ) -> Retained<Self> {
        let obj = Self::alloc(mtm).set_ivars(KeyboardScrollLockState { scroll_view, offset });
        unsafe { msg_send![super(obj), init] }
    }
}

pub fn lock_outer_scroll_while_keyboard(webview_window: &WebviewWindow) {
    #[cfg(target_os = "ios")]
    let _ = webview_window.with_webview(|webview| unsafe {
        // iOS에서 WKWebView의 scrollView에 msg_send로 접근
        // WKWebView는 NSObject이므로 포인터로 직접 메시지 전송
        let wk_webview = webview.inner().cast::<NSObject>();
        let scroll_view: Retained<UIScrollView> = msg_send![wk_webview, scrollView];
        let scroll_view_arc = Arc::new(scroll_view);

        // WKWebView를 UIView로 캐스팅하여 frame 접근
        let webview_as_view: &UIView = &*webview.inner().cast();
        // WKWebView 포인터를 raw pointer로 저장 (lifetime 보장은 webview_window가 관리)
        let wk_webview_ptr = wk_webview as *const NSObject;

        // 원복용 저장소
        let old_delegate_arc = Arc::new(std::sync::Mutex::new(None::<Retained<ProtocolObject<dyn UIScrollViewDelegate>>>));
        let old_bounces_arc = Arc::new(std::sync::Mutex::new(None::<bool>));
        let old_always_bounce_v_arc = Arc::new(std::sync::Mutex::new(None::<bool>));
        let old_adjust_arc = Arc::new(std::sync::Mutex::new(None::<UIScrollViewContentInsetAdjustmentBehavior>));

        // WKWebView의 원래 frame 저장
        let original_frame = webview_as_view.frame();
        ORIGINAL_WEBVIEW_FRAME.with(|cell| *cell.borrow_mut() = Some(original_frame));

        let center = NSNotificationCenter::defaultCenter();

        // 키보드 올라오기 직전: 바깥 스크롤 "잠금"
        {
            let sv = scroll_view_arc.clone();
            let old_delegate_arc = old_delegate_arc.clone();
            let old_bounces_arc = old_bounces_arc.clone();
            let old_always_bounce_v_arc = old_always_bounce_v_arc.clone();
            let old_adjust_arc = old_adjust_arc.clone();

            create_observer(&center, &UIKeyboardWillShowNotification, move |_n| {
                // 1) 원래 값 저장
                *old_delegate_arc.lock().unwrap() = sv.delegate();
                *old_bounces_arc.lock().unwrap() = Some(sv.bounces());
                *old_always_bounce_v_arc.lock().unwrap() = Some(sv.alwaysBounceVertical());
                *old_adjust_arc.lock().unwrap() = Some(sv.contentInsetAdjustmentBehavior());

                // 2) iOS 자동 inset 조정 끔
                sv.setContentInsetAdjustmentBehavior(UIScrollViewContentInsetAdjustmentBehavior::Never);

                // 3) "배경이 당겨지는 느낌" 완화
                sv.setBounces(false);
                sv.setAlwaysBounceVertical(false);

                // 4) 현재 위치를 고정하도록 delegate 교체
                let mtm = MainThreadMarker::new().expect("must be on the main thread");
                let lock_delegate = KeyboardScrollLockDelegate::new(
                    mtm,
                    sv.clone(),
                    sv.contentOffset(),
                );

                LOCK_DELEGATE.with(|cell| *cell.borrow_mut() = Some(lock_delegate));

                LOCK_DELEGATE.with(|cell| {
                    if let Some(d) = cell.borrow().as_ref() {
                        let obj = ProtocolObject::from_ref(&**d);
                        sv.setDelegate(Some(obj));
                    }
                });
            });
        }

        // 키보드가 완전히 올라온 후: WKWebView frame 고정
        {
            create_observer(&center, &UIKeyboardDidShowNotification, move |_n| {
                ORIGINAL_WEBVIEW_FRAME.with(|cell| {
                    if let Some(frame) = *cell.borrow() {
                        // raw pointer에서 다시 참조 생성 (webview는 계속 살아있음)
                        let view: &UIView = &*wk_webview_ptr.cast();
                        view.setFrame(frame);
                    }
                });
            });
        }

        // 키보드 내려가기 직전: 원래대로 복구
        {
            let sv = scroll_view_arc.clone();
            let old_delegate_arc = old_delegate_arc.clone();
            let old_bounces_arc = old_bounces_arc.clone();
            let old_always_bounce_v_arc = old_always_bounce_v_arc.clone();
            let old_adjust_arc = old_adjust_arc.clone();

            create_observer(&center, &UIKeyboardWillHideNotification, move |_n| {
                // delegate 원복
                if let Some(d) = old_delegate_arc.lock().unwrap().take() {
                    sv.setDelegate(Some(d.as_ref()));
                } else {
                    sv.setDelegate(None);
                }

                // 스크롤뷰 속성 원복
                if let Some(b) = old_bounces_arc.lock().unwrap().take() {
                    sv.setBounces(b);
                }
                if let Some(bv) = old_always_bounce_v_arc.lock().unwrap().take() {
                    sv.setAlwaysBounceVertical(bv);
                }
                if let Some(adj) = old_adjust_arc.lock().unwrap().take() {
                    sv.setContentInsetAdjustmentBehavior(adj);
                }

                // 보관하던 delegate 해제
                LOCK_DELEGATE.with(|cell| *cell.borrow_mut() = None);

                // 저장된 frame 정보 해제
                ORIGINAL_WEBVIEW_FRAME.with(|cell| *cell.borrow_mut() = None);
            });
        }
    });
}

fn create_observer(
    center: &NSNotificationCenter,
    name: &NSNotificationName,
    handler: impl Fn(&NSNotification) + 'static,
) -> Retained<ProtocolObject<dyn NSObjectProtocol>> {
    let block = block2::RcBlock::new(move |notification: NonNull<NSNotification>| {
        handler(unsafe { notification.as_ref() });
    });

    unsafe { center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block) }
}
