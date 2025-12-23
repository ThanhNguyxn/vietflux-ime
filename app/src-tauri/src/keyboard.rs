//! Windows keyboard hook module for VietFlux IME
//! Uses WH_KEYBOARD_LL + SendInput approach (like gonhanh.org)

#[cfg(windows)]
mod windows_impl {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Mutex;
    use vietflux_core::Engine;
    use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, GetKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
        KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_BACK, VK_CAPITAL,
        VK_CONTROL, VK_MENU, VK_SHIFT,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
        UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, LLKHF_INJECTED, MSG, WH_KEYBOARD_LL,
        WM_KEYDOWN, WM_SYSKEYDOWN,
    };

    /// Marker to identify our injected keys (prevent recursion)
    const INJECTED_KEY_MARKER: usize = 0x56464C58; // "VFLX" in hex

    /// Global hook handle stored as raw pointer value
    static HOOK_HANDLE: AtomicUsize = AtomicUsize::new(0);

    /// Global engine instance
    static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

    /// Processing flag to prevent recursion
    static IS_PROCESSING: AtomicBool = AtomicBool::new(false);

    /// Hook running state
    static HOOK_RUNNING: AtomicBool = AtomicBool::new(false);

    /// Initialize the IME engine
    pub fn init_engine() {
        let mut engine = ENGINE.lock().unwrap();
        if engine.is_none() {
            *engine = Some(Engine::new());
        }
    }

    /// Start the keyboard hook (call from main thread)
    pub fn start_hook() {
        if HOOK_RUNNING.load(Ordering::SeqCst) {
            return;
        }

        init_engine();
        HOOK_RUNNING.store(true, Ordering::SeqCst);

        std::thread::spawn(|| {
            unsafe {
                // For WH_KEYBOARD_LL, hMod can be None as hook runs in current process
                let hook_result: windows::core::Result<HHOOK> = SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(keyboard_hook_callback),
                    None,
                    0,
                );

                if let Ok(hook) = hook_result {
                    HOOK_HANDLE.store(hook.0 as usize, Ordering::SeqCst);

                    // Message loop to keep hook alive
                    let mut msg: MSG = std::mem::zeroed();
                    while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                        let _ = TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        });
    }

    /// Stop the keyboard hook
    pub fn stop_hook() {
        HOOK_RUNNING.store(false, Ordering::SeqCst);

        let hook_ptr = HOOK_HANDLE.swap(0, Ordering::SeqCst);
        if hook_ptr != 0 {
            unsafe {
                let hook = HHOOK(hook_ptr as *mut std::ffi::c_void);
                let _ = UnhookWindowsHookEx(hook);
            }
        }
    }

    /// Toggle IME enabled state
    pub fn toggle_ime() -> bool {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.toggle();
            e.is_enabled()
        } else {
            false
        }
    }

    /// Check if IME is enabled
    pub fn is_enabled() -> bool {
        let engine = ENGINE.lock().unwrap();
        if let Some(ref e) = *engine {
            e.is_enabled()
        } else {
            true // Default enabled
        }
    }

    /// Set input method
    pub fn set_method(method: &str) {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.set_method(method);
        }
    }

    /// Get current method
    pub fn get_method() -> String {
        let engine = ENGINE.lock().unwrap();
        if let Some(ref e) = *engine {
            e.get_method().to_string()
        } else {
            "telex".to_string()
        }
    }

    /// Set engine options
    pub fn set_options(auto_capitalize: bool, smart_quotes: bool, spell_check: bool) {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.set_options(auto_capitalize, smart_quotes, spell_check);
        }
    }

    /// Get engine options
    pub fn get_options() -> (bool, bool, bool) {
        let engine = ENGINE.lock().unwrap();
        if let Some(ref e) = *engine {
            e.get_options()
        } else {
            (true, false, true) // Default values
        }
    }

    /// Get shortcuts
    pub fn get_shortcuts() -> Vec<vietflux_core::shortcut::Shortcut> {
        let engine = ENGINE.lock().unwrap();
        if let Some(ref e) = *engine {
            e.get_shortcuts()
        } else {
            Vec::new()
        }
    }

    /// Add shortcut
    pub fn add_shortcut(trigger: &str, replacement: &str) {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.add_shortcut(trigger, replacement);
        }
    }

    /// Remove shortcut
    pub fn remove_shortcut(trigger: &str) {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.remove_shortcut(trigger);
        }
    }

    /// Toggle shortcut
    pub fn toggle_shortcut(trigger: &str) {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.toggle_shortcut(trigger);
        }
    }

    /// Clear engine
    pub fn clear() {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.clear();
        }
    }

    /// Keyboard hook callback
    unsafe extern "system" fn keyboard_hook_callback(
        code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        // Skip if already processing
        if IS_PROCESSING.load(Ordering::SeqCst) {
            return call_next_hook(code, w_param, l_param);
        }

        if code >= 0 {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);

            // Skip our own injected keys
            if kb_struct.dwExtraInfo == INJECTED_KEY_MARKER {
                return call_next_hook(code, w_param, l_param);
            }

            // Skip injected keys from other sources
            if (kb_struct.flags.0 & LLKHF_INJECTED.0) != 0 {
                return call_next_hook(code, w_param, l_param);
            }

            // Only process key down events
            let msg = w_param.0 as u32;
            if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                let vk_code = VIRTUAL_KEY(kb_struct.vkCode as u16);

                // Skip if Ctrl or Alt pressed
                if is_key_down(VK_CONTROL) || is_key_down(VK_MENU) {
                    // Clear buffer on Ctrl+key
                    if is_key_down(VK_CONTROL) {
                        clear_buffer();
                    }
                    return call_next_hook(code, w_param, l_param);
                }

                // Convert virtual key to char
                if let Some(ch) = vk_to_char(vk_code) {
                    if let Some(handled) = process_key(ch) {
                        if handled {
                            return LRESULT(1); // Block original key
                        }
                    }
                }
            }
        }

        call_next_hook(code, w_param, l_param)
    }

    /// Call next hook in chain
    unsafe fn call_next_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let hook_ptr = HOOK_HANDLE.load(Ordering::SeqCst);
        let hook = if hook_ptr != 0 {
            Some(HHOOK(hook_ptr as *mut std::ffi::c_void))
        } else {
            None
        };
        CallNextHookEx(hook, code, w_param, l_param)
    }

    /// Check if a key is currently pressed
    unsafe fn is_key_down(vk: VIRTUAL_KEY) -> bool {
        (GetAsyncKeyState(vk.0 as i32) & 0x8000u16 as i16) != 0
    }

    /// Check if CapsLock is on
    unsafe fn is_caps_lock_on() -> bool {
        (GetKeyState(VK_CAPITAL.0 as i32) & 0x0001) != 0
    }

    /// Convert virtual key to character
    fn vk_to_char(vk: VIRTUAL_KEY) -> Option<char> {
        let shift = unsafe { is_key_down(VK_SHIFT) };
        let caps = unsafe { is_caps_lock_on() };
        let uppercase = shift ^ caps;

        let base = match vk.0 {
            0x41..=0x5A => Some(((vk.0 - 0x41) as u8 + b'a') as char), // A-Z
            0x20 => Some(' '),                                  // Space
            _ => None,
        };

        base.map(|c| if uppercase { c.to_ascii_uppercase() } else { c })
    }

    /// Clear the engine buffer
    fn clear_buffer() {
        let mut engine = ENGINE.lock().unwrap();
        if let Some(ref mut e) = *engine {
            e.clear();
        }
    }

    /// Process a key through the IME engine
    fn process_key(ch: char) -> Option<bool> {
        IS_PROCESSING.store(true, Ordering::SeqCst);
        println!("Processing key: '{}'", ch);

        let result = {
            let mut engine = ENGINE.lock().unwrap();
            if let Some(ref mut e) = *engine {
                if !e.is_enabled() {
                    println!("IME disabled, passing through");
                    IS_PROCESSING.store(false, Ordering::SeqCst);
                    return Some(false);
                }
                Some(e.process_key(ch, false))
            } else {
                None
            }
        };

        if let Some(result) = result {
            use vietflux_core::engine::Action;

            println!("Engine result: action={:?}, backspace={}, output='{}'", 
                     result.action, result.backspace, result.output);

            match result.action {
                Action::Update => {
                    // ONLY block and replace if there's actual transformation
                    // i.e., we need to send backspaces OR output differs from just adding the char
                    if result.backspace > 0 {
                        println!("Transformation: sending {} backspaces + '{}'", result.backspace, result.output);
                        send_backspaces(result.backspace);
                        send_unicode_text(&result.output);
                        IS_PROCESSING.store(false, Ordering::SeqCst);
                        return Some(true); // Block original key
                    }
                    // No backspaces needed - let the key pass through normally
                    // The engine tracks it in buffer but we don't need to resend
                    println!("No transformation, passing through");
                }
                Action::Commit => {
                    // Word boundary - pass through
                    println!("Commit, passing through");
                }
                Action::Passthrough => {
                    println!("Passthrough");
                }
                Action::Restore => {
                    // Restore ASCII - need to replace
                    if result.backspace > 0 {
                        println!("Restore: sending {} backspaces + '{}'", result.backspace, result.output);
                        send_backspaces(result.backspace);
                        send_unicode_text(&result.output);
                        IS_PROCESSING.store(false, Ordering::SeqCst);
                        return Some(true);
                    }
                }
            }
        }

        IS_PROCESSING.store(false, Ordering::SeqCst);
        Some(false) // Don't block - let key pass through
    }

    /// Send backspace keys
    fn send_backspaces(count: usize) {
        if count == 0 {
            return;
        }

        let mut inputs: Vec<INPUT> = Vec::with_capacity(count * 2);

        for _ in 0..count {
            // Key down
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: INJECTED_KEY_MARKER,
                    },
                },
            });

            // Key up
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: INJECTED_KEY_MARKER,
                    },
                },
            });
        }

        unsafe {
            let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }

    /// Send Unicode text using SendInput
    fn send_unicode_text(text: &str) {
        if text.is_empty() {
            return;
        }

        let mut inputs: Vec<INPUT> = Vec::with_capacity(text.len() * 2);

        for ch in text.encode_utf16() {
            // Key down
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: INJECTED_KEY_MARKER,
                    },
                },
            });

            // Key up
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_UNICODE.0 | KEYEVENTF_KEYUP.0),
                        time: 0,
                        dwExtraInfo: INJECTED_KEY_MARKER,
                    },
                },
            });
        }

        unsafe {
            let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

// Re-export for Windows
#[cfg(windows)]
pub use windows_impl::*;

// Stub for non-Windows platforms
#[cfg(not(windows))]
pub fn start_hook() {
    eprintln!("Keyboard hook not implemented for this platform");
}

#[cfg(not(windows))]
pub fn stop_hook() {}

#[cfg(not(windows))]
pub fn toggle_ime() -> bool {
    false
}

#[cfg(not(windows))]
pub fn is_enabled() -> bool {
    false
}

#[cfg(not(windows))]
pub fn set_method(_method: &str) {}

#[cfg(not(windows))]
pub fn get_method() -> String {
    "telex".to_string()
}

#[cfg(not(windows))]
pub fn set_options(_auto_capitalize: bool, _smart_quotes: bool, _spell_check: bool) {}

#[cfg(not(windows))]
pub fn get_options() -> (bool, bool, bool) {
    (true, false, true)
}

#[cfg(not(windows))]
pub fn get_shortcuts() -> Vec<vietflux_core::shortcut::Shortcut> {
    Vec::new()
}

#[cfg(not(windows))]
pub fn add_shortcut(_trigger: &str, _replacement: &str) {}

#[cfg(not(windows))]
pub fn remove_shortcut(_trigger: &str) {}

#[cfg(not(windows))]
pub fn toggle_shortcut(_trigger: &str) {}

#[cfg(not(windows))]
pub fn clear() {}
