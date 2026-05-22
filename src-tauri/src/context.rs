use std::sync::Mutex;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetGUIThreadInfo, GetWindowRect, GetWindowTextW,
    GetWindowThreadProcessId, GUITHREADINFO,
};

static RECORDING_CONTEXT: Mutex<Option<RecordingContext>> = Mutex::new(None);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VibeMode {
    Code,
    Terminal,
    Prompt,
    Prose,
}

#[derive(Debug, Clone)]
pub struct RecordingContext {
    pub mode: VibeMode,
    pub language_hint: Option<String>,
}

impl Default for RecordingContext {
    fn default() -> Self {
        Self {
            mode: VibeMode::Prose,
            language_hint: None,
        }
    }
}

pub fn capture_recording_context() {
    let context = detect_context(unsafe { GetForegroundWindow() });
    if let Ok(mut slot) = RECORDING_CONTEXT.lock() {
        *slot = Some(context);
    }
}

pub fn peek_recording_context() -> RecordingContext {
    RECORDING_CONTEXT
        .lock()
        .ok()
        .and_then(|slot| slot.clone())
        .unwrap_or_default()
}

pub fn mode_display(context: &RecordingContext) -> String {
    let base = match context.mode {
        VibeMode::Code => "Code",
        VibeMode::Terminal => "Terminal",
        VibeMode::Prompt => "Prompt",
        VibeMode::Prose => "Prosa",
    };

    if let Some(language) = &context.language_hint {
        format!("{base} · {language}")
    } else {
        base.to_string()
    }
}

pub fn take_recording_context() -> RecordingContext {
    RECORDING_CONTEXT
        .lock()
        .ok()
        .and_then(|mut slot| slot.take())
        .unwrap_or_default()
}

fn detect_context(hwnd: HWND) -> RecordingContext {
    if hwnd.0.is_null() {
        return RecordingContext::default();
    }

    let class = window_class_name(hwnd);
    let title = window_title(hwnd);
    let title_lower = title.to_lowercase();

    if class == "CASCADIA_HOSTING_WINDOW_CLASS" || class == "ConsoleWindowClass" {
        return RecordingContext {
            mode: VibeMode::Terminal,
            language_hint: None,
        };
    }

    if is_prompt_surface(&title_lower) {
        return RecordingContext {
            mode: VibeMode::Prompt,
            language_hint: None,
        };
    }

    if let Some(language) = language_from_title(&title) {
        return RecordingContext {
            mode: VibeMode::Code,
            language_hint: Some(language),
        };
    }

    if is_electron_editor(&class, &title_lower) {
        if focused_in_terminal_panel(hwnd) {
            return RecordingContext {
                mode: VibeMode::Terminal,
                language_hint: None,
            };
        }

        return RecordingContext {
            mode: VibeMode::Code,
            language_hint: guess_language_from_title(&title_lower),
        };
    }

    RecordingContext {
        mode: VibeMode::Prose,
        language_hint: None,
    }
}

fn is_prompt_surface(title_lower: &str) -> bool {
    title_lower.contains("composer")
        || title_lower.contains("chat")
        || title_lower.contains("copilot")
        || title_lower.contains("agent")
}

fn is_electron_editor(class: &str, title_lower: &str) -> bool {
    class == "Chrome_WidgetWin_1"
        && (title_lower.contains("cursor")
            || title_lower.contains("visual studio code")
            || title_lower.ends_with(" - code"))
}

fn language_from_title(title: &str) -> Option<String> {
    let file = title.split(" - ").next()?.trim();
    let extension = file.rsplit('.').next()?;
    if extension == file {
        return None;
    }

    Some(match extension.to_lowercase().as_str() {
        "ts" | "tsx" => "typescript".to_string(),
        "js" | "jsx" => "javascript".to_string(),
        "rs" => "rust".to_string(),
        "py" => "python".to_string(),
        "go" => "go".to_string(),
        "java" => "java".to_string(),
        "cs" => "csharp".to_string(),
        "cpp" | "cc" | "cxx" | "h" | "hpp" => "cpp".to_string(),
        "vue" => "vue".to_string(),
        "svelte" => "svelte".to_string(),
        "json" => "json".to_string(),
        "toml" => "toml".to_string(),
        "yaml" | "yml" => "yaml".to_string(),
        "md" => "markdown".to_string(),
        "css" | "scss" => "css".to_string(),
        "html" => "html".to_string(),
        "sql" => "sql".to_string(),
        "sh" | "bash" | "zsh" => "shell".to_string(),
        "ps1" => "powershell".to_string(),
        _ => extension.to_string(),
    })
}

fn guess_language_from_title(title_lower: &str) -> Option<String> {
    if title_lower.contains("cursor") || title_lower.contains("code") {
        Some("typescript".to_string())
    } else {
        None
    }
}

fn focused_in_terminal_panel(hwnd: HWND) -> bool {
    let focus_class = focused_control_class(hwnd).to_lowercase();
    let focus_title = focused_control_title(hwnd).to_lowercase();

    focus_title.contains("terminal")
        || focus_class.contains("terminal")
        || focus_class.contains("xterm")
}

fn focused_control_class(root: HWND) -> String {
    let Some(focus) = focused_hwnd(root) else {
        return String::new();
    };
    window_class_name(focus)
}

fn focused_control_title(root: HWND) -> String {
    let Some(focus) = focused_hwnd(root) else {
        return String::new();
    };
    window_title(focus)
}

fn focused_hwnd(root: HWND) -> Option<HWND> {
    unsafe {
        let thread_id = GetWindowThreadProcessId(root, None);
        let mut info = GUITHREADINFO {
            cbSize: std::mem::size_of::<GUITHREADINFO>() as u32,
            ..Default::default()
        };

        if GetGUIThreadInfo(thread_id, &mut info).is_err() {
            return None;
        }

        if info.hwndFocus.0.is_null() {
            return None;
        }

        Some(info.hwndFocus)
    }
}

pub fn window_class_name(hwnd: HWND) -> String {
    let mut buffer = [0u16; 256];
    unsafe {
        let length = GetClassNameW(hwnd, &mut buffer);
        if length == 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buffer[..length as usize])
    }
}

pub fn window_title(hwnd: HWND) -> String {
    let mut buffer = [0u16; 512];
    unsafe {
        let length = GetWindowTextW(hwnd, &mut buffer);
        if length == 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buffer[..length as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_typescript_from_title() {
        assert_eq!(
            language_from_title("App.tsx - vibe-voice - Cursor").as_deref(),
            Some("typescript")
        );
    }

    #[test]
    fn detects_prompt_surface() {
        assert!(is_prompt_surface("composer - cursor"));
        assert!(!is_prompt_surface("app.tsx - cursor"));
    }
}
