# iOS 키보드 관련 이슈 트러블슈팅

## 개요

iOS에서 input 포커스 시 키보드가 올라오면서 발생하는 UI 문제들에 대한 해결 과정과 미해결 이슈를 기록합니다.

---

## ✅ 해결된 이슈

### 1. UIWebView → WKWebView 전환

**문제:**
- 초기 구현이 deprecated된 `UIWebView`를 사용하여 scrollView에 접근
- 향후 iOS 버전에서 호환성 문제 가능성

**해결:**
```rust
// 기존 (deprecated)
let webview: &objc2_ui_kit::UIWebView = &*webview.inner().cast();
let scroll_view_arc = Arc::new(webview.scrollView());

// 현재 (WKWebView)
let wk_webview = webview.inner().cast::<NSObject>();
let scroll_view: Retained<UIScrollView> = msg_send![wk_webview, scrollView];
let scroll_view_arc = Arc::new(scroll_view);
```

**결과:** objc2-web-kit 의존성 없이 `msg_send!` 매크로로 WKWebView의 scrollView에 안전하게 접근

**파일:** `src-tauri/src/ios_keyboard_scroll_lock.rs:62-66`

---

### 2. 키보드 올라올 때 스크롤 잠금

**문제:**
- iOS에서 키보드가 올라오면 WKWebView의 scrollView가 자동으로 스크롤되어 입력창 위치로 이동
- 사용자가 의도하지 않은 스크롤 발생

**해결 방법:**

#### a) contentInsetAdjustmentBehavior 비활성화
```rust
sv.setContentInsetAdjustmentBehavior(UIScrollViewContentInsetAdjustmentBehavior::Never);
```
iOS의 자동 inset 조정 기능을 차단 (Tauri issue #9368 참고)

#### b) Bounce 효과 제거
```rust
sv.setBounces(false);
sv.setAlwaysBounceVertical(false);
```
오버스크롤 "튕김" 효과 비활성화로 배경이 당겨지는 느낌 완화

#### c) Custom UIScrollViewDelegate 설치
```rust
unsafe impl UIScrollViewDelegate for KeyboardScrollLockDelegate {
    #[unsafe(method(scrollViewDidScroll:))]
    unsafe fn scroll_view_did_scroll(&self, _scroll_view: &UIScrollView) {
        self.ivars().scroll_view.setContentOffset(self.ivars().offset);
    }
}
```
스크롤이 발생하면 즉시 원래 위치로 강제 복귀

#### d) 키보드 알림 구독
- `UIKeyboardWillShowNotification`: 잠금 활성화
- `UIKeyboardWillHideNotification`: 원상 복구

**결과:** 키보드가 올라와도 스크롤 위치 고정됨 ✅

**파일:** `src-tauri/src/ios_keyboard_scroll_lock.rs:82-145`

---

### 3. 프론트엔드 Focus Fix

**문제:**
- iOS Safari/WebView의 버그로 input focus 시 키보드 인식 지연

**해결:**
```typescript
// src/lib/iosFocusFix.ts
export function iosFocusFix(node: HTMLElement) {
  const onFocus = () => {
    node.classList.remove('ios-focus-fix');
    requestAnimationFrame(() => node.classList.add('ios-focus-fix'));
  };
  // ...
}
```

```css
/* src/app.css */
@keyframes ios_focus_opacity_blink {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

.ios-focus-fix {
  animation: ios_focus_opacity_blink 0.01s;
}
```

**결과:** 0.01초 opacity blink 애니메이션으로 iOS의 focus 인식 문제 우회 ✅

**파일:** `src/lib/iosFocusFix.ts`, `src/app.css:97-104`

---

## ⏸️ 미해결 이슈

### 키보드 올라올 때 전체 화면이 위로 이동

**문제 설명:**
- 키보드가 올라오면 WKWebView 전체가 위로 밀려 올라가는 현상
- 스크롤은 잠겨있지만, View 자체의 frame이 변경되는 것으로 추정

**시도한 방법 1: Frame 복원**

```rust
// 1. 원래 frame 저장
let original_frame = webview_as_view.frame();
ORIGINAL_WEBVIEW_FRAME.with(|cell| *cell.borrow_mut() = Some(original_frame));

// 2. 키보드 완전히 올라온 후 복원 시도
create_observer(&center, &UIKeyboardDidShowNotification, move |_n| {
    ORIGINAL_WEBVIEW_FRAME.with(|cell| {
        if let Some(frame) = *cell.borrow() {
            let view: &UIView = &*wk_webview_ptr.cast();
            view.setFrame(frame);  // ❌ 효과 없음
        }
    });
});
```

**결과:** ❌ 실패 - frame을 복원해도 여전히 화면이 올라간 상태로 유지됨

**파일:** `src-tauri/src/ios_keyboard_scroll_lock.rs:79-137`

---

## 💡 향후 시도해볼 방법

### 1. Auto Layout Constraints 조정
WKWebView가 Auto Layout으로 배치된 경우, constraint를 직접 조작하여 위치 고정:
```swift
// 참고용 Swift 코드
webView.translatesAutoresizingMaskIntoConstraints = false
// Top constraint를 SafeArea가 아닌 SuperView.top에 고정
```

Rust에서는 `NSLayoutConstraint`를 objc2로 접근 필요

---

### 2. WKWebView의 scrollView.contentInset 조정
키보드 높이만큼 bottom inset만 조정하고, frame은 그대로 유지:
```rust
// UIKeyboardWillShowNotification에서 키보드 높이 추출
let keyboard_frame: CGRect = notification
    .userInfo()
    .objectForKey(&NSString::from_str("UIKeyboardFrameEndUserInfoKey"));
let keyboard_height = keyboard_frame.size.height;

// contentInset만 조정 (frame은 건드리지 않음)
let mut insets = scroll_view.contentInset();
insets.bottom = keyboard_height;
scroll_view.setContentInset(insets);
```

---

### 3. Tauri Window 설정 조정
`tauri.conf.json`에서 iOS 전용 설정 추가:
```json
{
  "identifier": "com.yourname.tickly",
  "ios": {
    "minimumSystemVersion": "14.0",
    "contentInsetAdjustmentBehavior": "never"  // 시도 필요
  }
}
```

---

### 4. CSS로 viewport 고정
프론트엔드에서 viewport meta 태그와 CSS 조합:
```html
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
```

```css
body, html {
  position: fixed;
  overflow: hidden;
  height: 100vh;
  height: -webkit-fill-available;  /* iOS Safari */
}
```

현재 이미 `app-container`가 `position: fixed`이지만, 추가 조정 가능

---

### 5. UIWindow의 rootViewController 조정
더 근본적인 접근: Tauri의 window 생성 로직에서 rootViewController의 자동 조정 비활성화

```rust
// with_webview 대신 window level에서 접근
let ui_window: &UIWindow = ...; // Tauri window에서 추출
ui_window.rootViewController()?.setAutomaticallyAdjustsScrollViewInsets(false);
```

---

## 참고 자료

- [Tauri Issue #9368](https://github.com/tauri-apps/tauri/issues/9368) - iOS keyboard contentInsetAdjustmentBehavior
- [objc2-ui-kit docs](https://docs.rs/objc2-ui-kit/latest/objc2_ui_kit/)
- [Apple UIScrollView Documentation](https://developer.apple.com/documentation/uikit/uiscrollview)
- [WKWebView Keyboard Handling Guide](https://developer.apple.com/documentation/webkit/wkwebview)

---

## 버전 정보

- **Tauri:** 2.9.5
- **objc2:** 0.6.3
- **iOS Target:** 14.0+
- **마지막 업데이트:** 2026-01-12
