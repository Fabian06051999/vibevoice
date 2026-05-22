use crate::focus::capture_target_window;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_LCONTROL, VK_LWIN, VK_RCONTROL, VK_RWIN};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

const STOP_DEBOUNCE_MS: u64 = 400;

static CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static WIN_DOWN: AtomicBool = AtomicBool::new(false);
static RECORDING: AtomicBool = AtomicBool::new(false);
static LOCKED: AtomicBool = AtomicBool::new(false);
static LAST_COMBO_RELEASE: AtomicU64 = AtomicU64::new(0);
static STOP_GENERATION: AtomicU64 = AtomicU64::new(0);

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
static ACTION_TX: OnceLock<Sender<HotkeyAction>> = OnceLock::new();
static HOOK_HANDLE: AtomicIsize = AtomicIsize::new(0);

pub enum HotkeyAction {
    StartRecording,
    StopRecording,
}

#[derive(Serialize, Clone)]
struct RecordingStartPayload {
    locked: bool,
}

pub fn start_hotkey_listener(app: AppHandle, tx: Sender<HotkeyAction>) {
    let _ = APP_HANDLE.set(app);

    if ACTION_TX.set(tx).is_err() {
        return;
    }

    thread::spawn(|| unsafe {
        let hook =
            match SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), HINSTANCE::default(), 0) {
                Ok(hook) => hook,
                Err(error) => {
                    eprintln!("Failed to install keyboard hook: {error}");
                    return;
                }
            };

        HOOK_HANDLE.store(hook.0 as isize, Ordering::SeqCst);

        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        let _ = UnhookWindowsHookEx(hook);
    });
}

pub fn stop_hotkey_listener() {
    let raw = HOOK_HANDLE.load(Ordering::SeqCst);
    if raw != 0 {
        unsafe {
            let _ = UnhookWindowsHookEx(HHOOK(raw as *mut _));
        }
        HOOK_HANDLE.store(0, Ordering::SeqCst);
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code != HC_ACTION as i32 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let keyboard = *(lparam.0 as *const KBDLLHOOKSTRUCT);
    let virtual_key = keyboard.vkCode;
    let is_key_down = wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;
    let is_key_up = wparam.0 == WM_KEYUP as usize || wparam.0 == WM_SYSKEYUP as usize;

    let is_ctrl = virtual_key == VK_LCONTROL.0 as u32 || virtual_key == VK_RCONTROL.0 as u32;
    let is_win = virtual_key == VK_LWIN.0 as u32 || virtual_key == VK_RWIN.0 as u32;

    if is_ctrl {
        if is_key_down {
            CTRL_DOWN.store(true, Ordering::SeqCst);
            try_start_recording();
        } else if is_key_up {
            CTRL_DOWN.store(false, Ordering::SeqCst);
            try_stop_recording();
        }
    }

    if is_win {
        if is_key_down {
            WIN_DOWN.store(true, Ordering::SeqCst);
            try_start_recording();
        } else if is_key_up {
            WIN_DOWN.store(false, Ordering::SeqCst);
            try_stop_recording();
        }

        if CTRL_DOWN.load(Ordering::SeqCst) {
            return LRESULT(1);
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn cancel_pending_stop() {
    STOP_GENERATION.fetch_add(1, Ordering::SeqCst);
}

fn schedule_stop() {
    let generation = STOP_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(STOP_DEBOUNCE_MS));
        if STOP_GENERATION.load(Ordering::SeqCst) != generation {
            return;
        }
        if LOCKED.load(Ordering::SeqCst) || !RECORDING.load(Ordering::SeqCst) {
            return;
        }
        finish_recording();
    });
}

fn emit_recording_start(locked: bool) {
    if let Some(app) = APP_HANDLE.get() {
        let payload = RecordingStartPayload { locked };
        let _ = app.emit("recording-start", payload);
    }
}

fn emit_recording_locked() {
    if let Some(app) = APP_HANDLE.get() {
        let _ = app.emit("recording-locked", ());
    }
}

fn begin_recording(locked: bool) {
    if RECORDING.swap(true, Ordering::SeqCst) {
        return;
    }

    capture_target_window();
    emit_recording_start(locked);

    if let Some(tx) = ACTION_TX.get() {
        let _ = tx.send(HotkeyAction::StartRecording);
    }
}

fn finish_recording() {
    if !RECORDING.swap(false, Ordering::SeqCst) {
        return;
    }

    LOCKED.store(false, Ordering::SeqCst);
    capture_target_window();

    if let Some(app) = APP_HANDLE.get() {
        let _ = app.emit("recording-stop", ());
        let _ = app.emit("transcribing", ());
    }

    if let Some(tx) = ACTION_TX.get() {
        let _ = tx.send(HotkeyAction::StopRecording);
    }
}

fn try_start_recording() {
    if !CTRL_DOWN.load(Ordering::SeqCst) || !WIN_DOWN.load(Ordering::SeqCst) {
        return;
    }

    cancel_pending_stop();

    if LOCKED.load(Ordering::SeqCst) {
        finish_recording();
        return;
    }

    let last_release = LAST_COMBO_RELEASE.load(Ordering::SeqCst);
    let now = now_ms();
    if last_release > 0 && now.saturating_sub(last_release) < STOP_DEBOUNCE_MS {
        LOCKED.store(true, Ordering::SeqCst);
        if RECORDING.load(Ordering::SeqCst) {
            emit_recording_locked();
        } else {
            begin_recording(true);
        }
        return;
    }

    begin_recording(false);
}

fn try_stop_recording() {
    if !RECORDING.load(Ordering::SeqCst) {
        return;
    }

    if CTRL_DOWN.load(Ordering::SeqCst) && WIN_DOWN.load(Ordering::SeqCst) {
        return;
    }

    if LOCKED.load(Ordering::SeqCst) {
        return;
    }

    LAST_COMBO_RELEASE.store(now_ms(), Ordering::SeqCst);
    schedule_stop();
}
