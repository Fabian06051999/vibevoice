# <img src="src/assets/logo.svg" width="32" height="32" alt="logo" /> Vibe Voice

### Talk to your code. Literally.

**Vibe Voice** is a push-to-talk voice-to-text tool built for the age of vibe coding. Hold a hotkey, speak naturally, and watch your words appear exactly where your cursor is — whether you're writing code, running terminal commands, or prompting an AI agent.

It knows the difference.

![Windows](https://img.shields.io/badge/platform-Windows_10%2F11-0078D4?logo=windows&logoColor=white)
![Tauri 2](https://img.shields.io/badge/built_with-Tauri_2.0-7C3AED?logo=tauri&logoColor=white)
![Groq](https://img.shields.io/badge/powered_by-Groq_Whisper-F55036)
![License](https://img.shields.io/badge/license-MIT-green)

---

## Why Vibe Voice?

Every vibe coder knows the pain: you're in the zone, your AI agent is building your app, and you need to type the next instruction — but typing breaks your flow. Voice input tools exist, but they don't understand code. They don't know that "use effect open paren" should become `useEffect(`.

**Vibe Voice does.**

- You say *"arrow"* → it types `=>`
- You say *"geschweifte klammer auf"* → it types `{`
- You say *"git status pipe grep main"* → it types `git status | grep main`
- You say *"Refactor this component to use a custom hook"* → it types exactly that

No training. No configuration. Just hold `Ctrl+Win` and speak.

---

## Features

| | Feature | Description |
|---|---|---|
| **🎙** | **Push-to-talk** | Hold `Ctrl+Win` to record, release to transcribe and paste |
| **🔒** | **Locked mode** | Double-tap `Ctrl+Win` for hands-free recording, tap again to stop |
| **🌍** | **Auto language** | Whisper auto-detects your spoken language — or pin one manually |
| **🎯** | **Context-aware** | Detects code editor, terminal, AI prompt, or prose — formats accordingly |
| **⚡** | **Symbol dictation** | Say "open brace", "arrow", "use effect" → get `{`, `=>`, `useEffect` |
| **🧹** | **Hallucination filter** | Suppresses phantom transcriptions on silence |
| **💊** | **Minimal overlay** | Tiny listening pill at the bottom of your screen — never in the way |
| **🔇** | **System tray** | Lives silently in the tray, zero distractions |
| **🚀** | **Instant** | Powered by [Groq](https://groq.com/) — transcription in under a second |

---

## How it works

```
Ctrl+Win (hold) → microphone records → release → Groq Whisper API → text injected
```

Under the hood:

1. A low-level keyboard hook captures `Ctrl+Win` globally (works in any app)
2. Your focused window is saved before recording starts
3. Audio is captured from your default microphone
4. The context is detected — are you in a code editor? terminal? AI chat?
5. Audio is sent to Groq's Whisper API for near-instant transcription
6. The transcript is formatted based on context (symbol replacement in code mode, raw text in prose mode)
7. Focus is restored to your original window and the text is injected

The entire pipeline typically completes in **under 2 seconds**.

---

## Context detection

Vibe Voice reads your active window and adjusts its behavior automatically:

| Context | Detected when | What happens |
|---|---|---|
| **Code** | Cursor / VS Code with a source file open | Symbols are replaced, camelCase presets applied |
| **Terminal** | Windows Terminal, integrated terminal | Pipe, slash, and command symbols recognized |
| **Prompt** | Cursor Composer / AI Chat | Natural language preserved as-is |
| **Prose** | Any other text field | Natural language preserved as-is |

---

## Symbol dictation

In Code and Terminal mode, spoken words are intelligently replaced:

| You say | You get |
|---|---|
| "open paren" / "klammer auf" | `(` |
| "open brace" / "geschweifte klammer auf" | `{` |
| "arrow" / "pfeil" | `=>` |
| "triple equals" | `===` |
| "double colon" / "doppelpunkt doppelpunkt" | `::` |
| "pipe" | `\|` |
| "use effect" | `useEffect` |
| "use state" | `useState` |
| "console log" | `console.log` |
| "new line" / "neue zeile" | ↵ newline |

Works in both **English** and **German** — more languages coming.

---

## Quick start

### Prerequisites

- **Windows 10 or 11**
- **Rust** — install via [rustup.rs](https://rustup.rs/)
- **Node.js 18+** — for the Tauri CLI
- **Groq API key** — free at [console.groq.com/keys](https://console.groq.com/keys)

### Build & run

```bash
git clone https://github.com/Fabian06051999/vibevoice.git
cd vibevoice
npm install
npm run build
```

Then run `src-tauri/target/release/vibe-voice-tool.exe`.

### First launch

1. The app starts in the **system tray** (bottom-right)
2. Right-click the tray icon → **Settings**
3. Paste your **Groq API key** (or click the link to get one free)
4. Leave language on **Auto-detect** or pick one
5. **Save settings** — you'll see a confirmation toast
6. Hold `Ctrl+Win` and start talking

---

## Recording modes

| Action | Behavior |
|---|---|
| **Hold** `Ctrl+Win` | Record while held — release to transcribe and paste |
| **Double-tap** `Ctrl+Win` | **Locked mode** — recording continues hands-free |
| **Tap** `Ctrl+Win` (while locked) | Stop recording, transcribe and paste |

Locked mode shows a `🔒 Locked` indicator in the overlay so you know it's running.

---

## Tech stack

| Layer | Technology |
|---|---|
| App framework | [Tauri 2.0](https://v2.tauri.app/) |
| Backend | Rust |
| Frontend | Vanilla HTML / CSS / JS |
| Audio capture | [cpal](https://crates.io/crates/cpal) + [hound](https://crates.io/crates/hound) |
| Transcription | [Groq Whisper](https://groq.com/) (whisper-large-v3-turbo) |
| Keyboard hook | Win32 `WH_KEYBOARD_LL` |
| Text injection | Win32 `SendInput` / Clipboard |
| DPI support | Per-monitor DPI aware v2 |

---

## Project structure

```
vibevoice/
├── src/                          # Frontend
│   ├── index.html / index.css    # Settings window
│   ├── overlay.html / .css / .js # Recording overlay pill
│   ├── main.js                   # Settings logic + toast
│   └── assets/logo.svg           # App logo
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs                # Orchestration, tray, pipeline
│   │   ├── audio.rs              # Microphone recording
│   │   ├── transcription.rs      # Groq Whisper client
│   │   ├── hotkey.rs             # Global hotkey + locked mode
│   │   ├── focus.rs              # Focus capture/restore
│   │   ├── clipboard.rs          # Text injection methods
│   │   ├── context.rs            # Vibe mode detection
│   │   ├── format.rs             # Symbol map + code presets
│   │   └── config.rs             # Config persistence
│   ├── icons/                    # App, tray & window icons
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── README.md
```

---

## Configuration

Settings are stored in `%APPDATA%\vibe-voice-tool\config.json` — never in the repo:

```json
{
  "api_key": "gsk_...",
  "language": "auto",
  "hotkey": "Ctrl+Win"
}
```

---

## Roadmap

- [ ] Release binary downloads (no build required)
- [ ] Custom hotkey configuration
- [ ] Per-project voice profiles (`.vibe-voice.json`)
- [ ] LLM post-processing for smarter prompt formatting
- [ ] macOS & Linux support
- [ ] More language symbol maps

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

## License

[MIT](LICENSE)

---

<p align="center">
  <sub>Built with vibes, Rust, and way too much coffee.</sub>
</p>
