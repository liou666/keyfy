use std::error::Error;

use crate::keycodes::key_from_code;
use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::VK_ESCAPE,
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
            UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
            WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

use crate::keycodes::{self, Key};

#[derive(Debug)]
pub enum EventType {
    KeyPress(Key),
    KeyRelease(Key),
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
}

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event)>> = None;
static mut GLOBAL_CONFIG: Option<Config> = None;

unsafe extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;

        let is_virtual_key = kb_struct.flags
            & windows::Win32::UI::WindowsAndMessaging::KBDLLHOOKSTRUCT_FLAGS(0x10)
            != windows::Win32::UI::WindowsAndMessaging::KBDLLHOOKSTRUCT_FLAGS(0);

        if let Some(config) = &GLOBAL_CONFIG {
            if !config.is_listen_virtual_key && is_virtual_key {
                println!("忽略模拟按键");
                return CallNextHookEx(None, code, w_param, l_param);
            }
        } else if is_virtual_key {
            println!("忽略模拟按键");
            return CallNextHookEx(None, code, w_param, l_param);
        }

        let event_type = match w_param.0 as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => Some(EventType::KeyPress(key_from_code(vk_code))),
            WM_KEYUP | WM_SYSKEYUP => Some(EventType::KeyRelease(key_from_code(vk_code))),
            _ => None,
        };

        if let Some(event_type) = event_type {
            if let Some(callback) = GLOBAL_CALLBACK.as_mut() {
                callback(Event { event_type });
            }
        }
    }
    CallNextHookEx(None, code, w_param, l_param)
}

pub struct Config {
    pub is_listen_virtual_key: bool,
}

pub fn listen<F>(callback: F, config: Option<Config>) -> Result<(), Box<dyn Error>>
where
    F: FnMut(Event) + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        GLOBAL_CONFIG = config;

        let h_instance: HINSTANCE = GetModuleHandleW(None).unwrap().into();
        let _hooks = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), h_instance, 0)?;
        let _msg: *mut MSG = &mut MSG::default();
        let _ = GetMessageW(_msg, None, 0, 0);

        //  UnhookWindowsHookEx(_hooks)
    };

    Ok(())
}
