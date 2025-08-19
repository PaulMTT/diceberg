# 🐭 AI Chat TUI (ratatui + mistralrs)

A **terminal-based AI chat application** built with [`ratatui`](https://github.com/ratatui-org/ratatui) and powered by [`mistralrs`](https://github.com/EricLBuehler/mistral.rs).

It provides a **fast, interactive, keyboard-driven interface** for chatting with large language models (LLMs), featuring **streaming responses**, **markdown rendering**, **think-mode reasoning traces**, and **chat session management**.

---

## ✨ Features

* **TUI built with ratatui** — smooth, responsive terminal UI
* **Streaming responses** — see model output token by token
* **Markdown rendering** — supports headers, lists, blockquotes, and code
* **Special `<think>...</think>` styling** — highlights internal reasoning traces in dim blue
* **Queueing system** — multiple prompts can be queued while one is running
* **Status bar** — real-time token usage, speed, and state
* **Legend bar** — dynamic keyboard hints that adapt to context
* **Cancelable requests** — abort mid-stream, undo last exchange, or clear input
* **Scroll model** — supports line/page navigation with auto-follow
* **Configurable personality** — default system prompt makes the model decisive

---

## 🖥️ UI Layout

The screen is divided into **four sections**:

1. **Chat Window**

    * Scrollable conversation history
    * `[you]` for user, `[ai]` for assistant, `[system]` for system prompt

2. **Input Box**

    * Where you type your next message
    * Title changes depending on **think mode**

3. **Legend Bar**

    * Shows keybindings & current state dynamically
    * Example: `Enter send • Ctrl+C cancel current • Pending: 2`

4. **Status Bar**

    * Displays model status, errors, or performance metrics

---

## ⌨️ Key Bindings

| Key             | Action                                          |
| --------------- | ----------------------------------------------- |
| **Enter**       | Send message (queue if busy)                    |
| **Ctrl+C**      | Contextual: clear input / cancel request / undo |
| **Esc**         | Quit                                            |
| **Ctrl+N**      | Start new chat (clear history & input)          |
| **Ctrl+T**      | Toggle *Think Mode*                             |
| **↑ / ↓**       | Scroll line up/down                             |
| **PgUp / PgDn** | Scroll page up/down                             |
| **Home / End**  | Jump to top / bottom                            |

The **legend bar** updates automatically to reflect the current meaning of each key.

---

## 🧠 Think Mode

When enabled (`Ctrl+T`):

* Model is asked to **"think" internally** in `<think>...</think>` blocks
* Sampler configuration shifts to encourage **reasoning exploration**:

    * `temperature = 0.6` (vs 0.7)
    * `top_p = 0.95` (vs 0.8)
    * Repetition penalties applied more strongly
* `<think>` blocks styled in **dim blue**, visually separated from final answers

---

## 🏗️ Architecture

The app is organized into **modules**, with a strict **State + View** separation and a **duplex communication layer** for LLM interaction.

### Modules

* **`term/ui/chat`** — Chat history

    * `ChatState`: stores turns, responses, scroll offsets
    * `ChatView`: renders conversation and scrollbar

* **`term/ui/input`** — User input

    * `InputState`: buffer + think flag
    * `InputView`: renders input box

* **`term/ui/legend`** — Keybindings legend

    * `LegendState`: busy, pending, undo, think flags
    * `LegendView`: renders keybinding hints

* **`term/ui/status`** — Status bar

    * `StatusState`: current status text
    * `StatusView`: renders single line

* **`term/ui/traits`** — Shared traits

    * `Render`, `RenderArea`, `Clearable`

* **`term/duplex`** — Message passing abstraction

    * `DuplexSource` / `DuplexSink` traits
    * `SourceHandle` / `SinkHandle` implementations
    * `bounded` and `unbounded` duplex channels

* **`term/llm_chat_sink`** — Backend connector

    * `MistralDuplexSink` streams LLM responses
    * Handles cancel, shutdown, and graceful termination

* **`term/llm_chat_ui_source`** — Frontend controller

    * `MistralDuplexSourceUi`: main event loop and TUI manager
    * Runs user input handling, queueing, and rendering

---

## 🔄 Flow of a Turn

Here’s how a single interaction works:

1. **User Input**

    * User types → presses **Enter**
    * Input trimmed → `PendingReq` created

2. **Dispatch**

    * If idle → request sent immediately
    * If busy → request queued

3. **Streaming**

    * `MistralDuplexSink` streams `Response::Chunk` messages
    * Appends into current `Turn` → rendered live in chat

4. **Completion**

    * On `ChatEvent::Complete`:

        * Mark turn as complete
        * Show token usage & speed in status bar

5. **Next Request**

    * If queue non-empty → dequeue and start automatically

6. **Cancellation / Undo**

    * **Ctrl+C** cancels current turn or removes last completed one

---

## 🎨 Rendering Markdown

The app uses [`tui-markdown`](https://crates.io/crates/tui-markdown) with extra logic:

* Detects Markdown blocks (headers, lists, code, quotes)
* Joins inline text gracefully between chunks
* Applies special styling for `<think>` regions

This makes answers visually structured, not raw plaintext.
