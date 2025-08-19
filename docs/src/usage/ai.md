# 🐭 AI Chat TUI (ratatui + mistralrs)

A **terminal-based AI chat application** built with [`ratatui`](https://github.com/ratatui-org/ratatui) and powered by [`mistralrs`](https://github.com/EricLBuehler/mistral.rs).

It provides a **fast, interactive, keyboard-driven interface** for chatting with large language models (LLMs), featuring **streaming responses**, **markdown rendering**, **think-mode reasoning traces**, and **chat session management**.

---

## 🚀 Installation

The AI TUI requires one of the **Mistral backends** to be enabled via Cargo features.  

- On **macOS**, use the `metal` feature.  
- On **Linux**, you’ll typically use `cuda` (or another supported backend).  
- If you also want to enable the **MCP server**, add the `mcp` feature.  

Examples:

```bash
# macOS (Metal backend only)
cargo install --path . --features metal

# Linux (CUDA backend only)
cargo install --path . --features cuda

# macOS with both AI + MCP server
cargo install --path . --features "metal mcp"
````

⚠️ Only one backend should be chosen at a time (e.g. don’t enable both `metal` and `cuda`).

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
* **`term/ui/input`** — User input
* **`term/ui/legend`** — Keybindings legend
* **`term/ui/status`** — Status bar
* **`term/ui/traits`** — Shared rendering traits
* **`term/duplex`** — Message passing abstraction
* **`term/llm_chat_sink`** — Backend connector (Mistral streaming)
* **`term/llm_chat_ui_source`** — Frontend controller / main TUI manager

---

## 🔄 Flow of a Turn

1. **User Input** → create request
2. **Dispatch** → send immediately if idle, otherwise queue
3. **Streaming** → chunks stream from Mistral, appended live
4. **Completion** → mark turn complete, show stats
5. **Next Request** → auto-start next queued prompt
6. **Cancel/Undo** → `Ctrl+C` cancels or reverts

---

## 🎨 Rendering Markdown

The app uses [`tui-markdown`](https://crates.io/crates/tui-markdown) with extra logic:

* Detects Markdown blocks (headers, lists, code, quotes)
* Joins inline text gracefully between chunks
* Styles `<think>` regions in **dim blue**

This makes answers visually structured, not raw plaintext.
