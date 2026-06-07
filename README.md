# <img src="src/assets/logo.svg" width="32" height="32" alt="Vibe Voice logo" /> Vibe Voice

### Free Windows voice-to-text for vibe coding — talk to Cursor, Claude & ChatGPT

**Vibe Voice** is a free, open-source **push-to-talk voice-to-text** desktop app for **Windows 10/11**, built for **vibe coding** and AI-assisted development. Hold a hotkey, speak your next prompt, and the transcript is pasted exactly where your cursor is — in **Cursor**, **Composer**, **ChatGPT**, **Claude**, VS Code, or any text field.

No modes. No context guessing. Just fast, accurate **speech-to-text for coding prompts**.

[![Vibe Voice — free push-to-talk voice-to-text for vibe coding on Windows](src/assets/promo-banner.png)](https://github.com/Fabian06051999/vibevoice/releases)

![Release](https://img.shields.io/github/v/release/Fabian06051999/vibevoice?label=release)
![Windows](https://img.shields.io/badge/platform-Windows_10%2F11-0078D4?logo=windows&logoColor=white)
![Tauri 2](https://img.shields.io/badge/built_with-Tauri_2.0-7C3AED?logo=tauri&logoColor=white)
![Groq Whisper](https://img.shields.io/badge/powered_by-Groq_Whisper-F55036)
![License](https://img.shields.io/badge/license-MIT-green)

**Keywords:** voice to text Windows · speech to text for coding · push-to-talk dictation · Whisper Groq · Cursor voice input · AI coding prompts · Tauri Rust desktop app

---

## Table of contents

- [Why Vibe Voice?](#why-vibe-voice)
- [Features](#features)
- [How it works](#how-it-works)
- [Prompt-first by design](#prompt-first-by-design)
- [Install (Windows)](#install-windows)
- [Build from source](#build-from-source)
- [Recording modes](#recording-modes)
- [Tech stack](#tech-stack)
- [Project structure](#project-structure)
- [Configuration](#configuration)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)

---

## Why Vibe Voice?

Every vibe coder knows the pain: you're in the zone, your AI agent is building your app, and you need to type the next instruction — but typing breaks your flow. Most **Windows dictation tools** are built for emails and documents, not for **AI coding workflows**. Vibe Voice fills that gap.

**Perfect if you want:**

- **Voice input for Cursor / Claude / ChatGPT** without switching apps
- A lightweight **Windows speech-to-text** tool that stays out of the way
- **Push-to-talk** control instead of always-on microphone listening
- Fast transcription via **Groq Whisper** (typically under 2 seconds)

**Vibe Voice keeps the loop moving.**

- You think of the next change
- You hold `Ctrl+Win`
- You say the prompt out loud
- Vibe Voice inserts clean text at your cursor
- Your AI agent keeps working

It is intentionally simple: **speech in, prompt out**.

---

## Features

| | Feature | Description |
|---|---|---|
| **🎙** | **Push-to-talk** | Hold `Ctrl+Win` to record, release to transcribe and paste |
| **🔒** | **Locked mode** | Double-tap `Ctrl+Win` for hands-free recording, tap again to stop |
| **🌍** | **Auto language** | Whisper auto-detects your spoken language — or pin one manually |
| **🎯** | **Prompt-first** | Optimized for AI-agent instructions instead of brittle app detection |
| **⚡** | **Vibe coding vocabulary** | Whisper is primed with Cursor, Composer, agents, refactors, tests and common dev terms |
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
4. Audio is sent to Groq's Whisper API with a vibe-coding prompt vocabulary
5. The transcript is cleaned up as natural prompt text
6. Focus is restored to your original window and the text is injected

The entire pipeline typically completes in **under 2 seconds**.

---

## Prompt-first by design

Vibe Voice deliberately does **not** try to convert your speech into code syntax. That sounds clever, but for vibe coding it often gets in the way.

Instead, it optimizes for the thing you actually do all day: giving precise instructions to an AI coding agent.

Examples:

| You say | Vibe Voice inserts |
|---|---|
| “Refactor this component so the state logic lives in a custom hook and add tests.” | Refactor this component so the state logic lives in a custom hook and add tests. |
| “Explain why this TypeScript error happens and suggest the smallest safe fix.” | Explain why this TypeScript error happens and suggest the smallest safe fix. |
| “Update the README so the setup instructions are clearer for Windows users.” | Update the README so the setup instructions are clearer for Windows users. |
| “Find the bug in the save flow and fix it without changing unrelated behavior.” | Find the bug in the save flow and fix it without changing unrelated behavior. |

---

## Install (Windows)

### What you need

- **Windows 10 or 11**
- **Your own Groq API key** — Vibe Voice does not include an API key. Create a free key at [console.groq.com/keys](https://console.groq.com/keys) and paste it into Settings.

### Download & install

1. Go to [**Releases**](https://github.com/Fabian06051999/vibevoice/releases) and download the MSI for your language:
   - English: `Vibe Voice Tool_*_x64_en-US.msi`
   - German: `Vibe Voice Tool_*_x64_de-DE.msi`
2. Double-click the `.msi` file — Windows opens the familiar installer wizard:
   - Welcome screen
   - Choose install folder
   - Install progress (WebView2 is downloaded automatically if needed)
   - Finish — optional launch from the wizard
3. Vibe Voice appears in the **Start menu** and runs in the **system tray**

To uninstall later: **Settings → Apps → Installed apps → Vibe Voice Tool → Uninstall**.

### First launch

1. The app starts in the **system tray** (bottom-right)
2. Right-click the tray icon → **Settings**
3. Paste your **own Groq API key** (or click the link to get one free)
4. Leave language on **Auto-detect** or pick one
5. **Save settings** — you'll see a confirmation toast
6. Hold `Ctrl+Win` and start talking

---

## Build from source

For developers who want to run or build the app locally:

### Prerequisites

- **Windows 10 or 11**
- **Rust** — [rustup.rs](https://rustup.rs/)
- **Node.js 18+**

### Build & run

```bash
git clone https://github.com/Fabian06051999/vibevoice.git
cd vibevoice
npm install
npm run dev
```

### Build the Windows MSI locally

```bash
npm run build:installer
```

The MSI file is created at:

`src-tauri/target/release/bundle/msi/Vibe Voice Tool_*_x64_en-US.msi`

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
│   │   ├── format.rs             # Prompt cleanup
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

You must bring your own Groq API key. The app never ships with one, and your saved key stays local on your machine.

---

## FAQ

### Is Vibe Voice free?

Yes. Vibe Voice is **MIT-licensed** and free to download. You only need your own **Groq API key** ([free tier available](https://console.groq.com/keys)) for transcription.

### Does it work with Cursor, Claude, and ChatGPT?

Yes. Vibe Voice pastes text at your **current cursor position** in any Windows app — including Cursor, VS Code, browsers, terminals, and chat UIs.

### How is this different from Windows Voice Typing?

Windows Voice Typing is system-wide dictation. Vibe Voice is **prompt-first**: optimized for short, precise **AI agent instructions**, with push-to-talk, a minimal overlay, and a vocabulary tuned for dev terms (refactors, tests, Composer, agents, etc.).

### Does it run locally or in the cloud?

Audio is sent to the **Groq Whisper API** for transcription. The app itself is a local **Tauri + Rust** desktop program; your API key stays in `%APPDATA%` on your machine.

### Which languages are supported?

Whisper **auto-detects** your spoken language. You can also pin a language manually in Settings. Installers are available in **English** and **German**.

### Can I use it without Cursor?

Absolutely. Any app with a text cursor works — IDEs, browsers, Slack, Notion, terminal prompts, and more.

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
