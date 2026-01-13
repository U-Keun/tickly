use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{msg_send, MainThreadMarker};
use objc2_foundation::NSString;
use objc2_ui_kit::{
    UIApplication, UIButton, UIButtonType, UIColor, UIControlEvents, UIControlState,
    UIScrollView, UITextField, UITextBorderStyle, UIView, UIWindow,
};
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

use crate::AppState;

// Thread-local storage to keep UI elements alive
thread_local! {
    static NATIVE_CONTAINER: RefCell<Option<Retained<UIView>>> = RefCell::new(None);
}

// Shared state between components
struct SharedState {
    app_handle: AppHandle,
}

/// Entry point called from lib.rs to setup native iOS UI
pub fn setup_native_ui(webview_window: &WebviewWindow, app_handle: AppHandle) {
    #[cfg(target_os = "ios")]
    {
        let _ = webview_window.with_webview(move |webview| unsafe {
            let mtm = MainThreadMarker::new().expect("Must be called on main thread");

            // Get the webview
            let wk_webview: &NSObject = &*webview.inner().cast::<NSObject>();

            // Get the window
            let app = UIApplication::sharedApplication(mtm);
            let windows: Retained<NSObject> = msg_send![&app, windows];
            let window: Retained<UIWindow> = msg_send![&windows, firstObject];

            // Create shared state
            let shared_state = Arc::new(SharedState {
                app_handle: app_handle.clone(),
            });

            // Create native UI container (only for AddItemInput)
            let native_container = UIView::new(mtm);
            native_container.setTranslatesAutoresizingMaskIntoConstraints(false);
            native_container.setBackgroundColor(Some(&UIColor::colorWithRed_green_blue_alpha(0.9725, 0.9647, 0.9412, 1.0)));

            // Add container to window (above webview)
            window.addSubview(&native_container);

            // Create AddItemInput view (60pt height)
            let add_input_view = create_add_item_input_view(mtm, shared_state.clone());
            native_container.addSubview(&add_input_view);

            // Setup layout constraints
            let total_height = 60.0; // Only AddItemInput

            native_container.topAnchor().constraintEqualToAnchor(&window.safeAreaLayoutGuide().topAnchor()).setActive(true);
            native_container.leadingAnchor().constraintEqualToAnchor(&window.leadingAnchor()).setActive(true);
            native_container.trailingAnchor().constraintEqualToAnchor(&window.trailingAnchor()).setActive(true);
            native_container.heightAnchor().constraintEqualToConstant(total_height).setActive(true);

            add_input_view.topAnchor().constraintEqualToAnchor(&native_container.topAnchor()).setActive(true);
            add_input_view.leadingAnchor().constraintEqualToAnchor(&native_container.leadingAnchor()).setActive(true);
            add_input_view.trailingAnchor().constraintEqualToAnchor(&native_container.trailingAnchor()).setActive(true);
            add_input_view.heightAnchor().constraintEqualToConstant(60.0).setActive(true);

            // Disable webview scroll
            let scroll_view: Retained<UIScrollView> = msg_send![wk_webview, scrollView];
            scroll_view.setScrollEnabled(false);
            scroll_view.setBounces(false);

            // Store container
            NATIVE_CONTAINER.with(|cell| {
                *cell.borrow_mut() = Some(native_container);
            });
        });
    }

    #[cfg(not(target_os = "ios"))]
    {
        let _ = (webview_window, app_handle);
    }
}

fn create_add_item_input_view(mtm: MainThreadMarker, shared_state: Arc<SharedState>) -> Retained<UIView> {
    unsafe {
        let container = UIView::new(mtm);
        container.setTranslatesAutoresizingMaskIntoConstraints(false);
        container.setBackgroundColor(Some(&UIColor::colorWithRed_green_blue_alpha(0.9725, 0.9647, 0.9412, 1.0)));

        let text_field = UITextField::new(mtm);
        text_field.setTranslatesAutoresizingMaskIntoConstraints(false);
        text_field.setPlaceholder(Some(&NSString::from_str("항목을 입력하세요...")));
        text_field.setBorderStyle(UITextBorderStyle::RoundedRect);
        text_field.setBackgroundColor(Some(&UIColor::whiteColor()));

        let add_button = UIButton::buttonWithType(UIButtonType::System, mtm);
        add_button.setTranslatesAutoresizingMaskIntoConstraints(false);
        add_button.setTitle_forState(Some(&NSString::from_str("추가")), UIControlState::Normal);
        add_button.setTitleColor_forState(Some(&UIColor::colorWithRed_green_blue_alpha(0.1686, 0.1686, 0.1686, 1.0)), UIControlState::Normal);
        add_button.setBackgroundColor(Some(&UIColor::colorWithRed_green_blue_alpha(0.6549, 0.7804, 0.9059, 1.0)));
        add_button.layer().setCornerRadius(8.0);

        container.addSubview(&text_field);
        container.addSubview(&add_button);

        // Button action
        let app_handle = shared_state.app_handle.clone();
        let tf = text_field.clone();

        let block = block2::RcBlock::new(move |_: *mut NSObject| {
            println!("🔵 Add button clicked!");
            let text_opt = tf.text();
            println!("🔵 Got text option: {:?}", text_opt.is_some());

            if let Some(text_ns) = text_opt {
                let text_str = text_ns.to_string();
                println!("🔵 Text string: '{}'", text_str);
                let trimmed = text_str.trim();

                if !trimmed.is_empty() {
                    println!("🔵 Adding item: '{}'", trimmed);
                    let text = trimmed.to_string();

                    // Get selected category from global state
                    let cat_id = *crate::SELECTED_CATEGORY_FOR_NATIVE_INPUT.lock().unwrap();
                    println!("🔵 Category ID: {:?}", cat_id);
                    let app = app_handle.clone();

                    std::thread::spawn(move || {
                        println!("🔵 Calling add_item...");
                        let result = crate::add_item(text.clone(), cat_id, app.state::<AppState>());
                        println!("🔵 add_item result: {:?}", result.is_ok());

                        if result.is_ok() {
                            println!("🔵 Emitting native-item-added event");
                            let emit_result = app.emit("native-item-added", ());
                            println!("🔵 Emit result: {:?}", emit_result.is_ok());
                        } else {
                            println!("🔴 add_item error: {:?}", result.err());
                        }
                    });

                    tf.setText(Some(&NSString::from_str("")));
                    tf.resignFirstResponder();
                    println!("🔵 Cleared text field");
                } else {
                    println!("⚠️ Text is empty after trim");
                }
            } else {
                println!("⚠️ Text is None");
            }
        });

        // Keep block alive by storing it
        let block_copy = block.clone();
        let _: () = msg_send![&*add_button, addTarget:&*block, action:objc2::sel!(invoke), forControlEvents:UIControlEvents::TouchUpInside];

        // Store block to prevent it from being dropped
        std::mem::forget(block_copy);

        // Constraints
        text_field.leadingAnchor().constraintEqualToAnchor_constant(&container.leadingAnchor(), 16.0).setActive(true);
        text_field.centerYAnchor().constraintEqualToAnchor(&container.centerYAnchor()).setActive(true);
        text_field.heightAnchor().constraintEqualToConstant(44.0).setActive(true);

        add_button.leadingAnchor().constraintEqualToAnchor_constant(&text_field.trailingAnchor(), 8.0).setActive(true);
        add_button.trailingAnchor().constraintEqualToAnchor_constant(&container.trailingAnchor(), -16.0).setActive(true);
        add_button.centerYAnchor().constraintEqualToAnchor(&container.centerYAnchor()).setActive(true);
        add_button.widthAnchor().constraintEqualToConstant(60.0).setActive(true);
        add_button.heightAnchor().constraintEqualToConstant(44.0).setActive(true);

        container
    }
}

fn load_categories_sync(app_handle: &AppHandle) -> Vec<crate::Category> {
    let state = app_handle.state::<AppState>();
    match crate::get_categories(state) {
        Ok(categories) => categories,
        Err(e) => {
            eprintln!("Failed to load categories: {}", e);
            vec![]
        }
    }
}
