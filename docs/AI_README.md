# DICI AI TUI Client

A modular, asynchronous, and extensible terminal interface for interacting with AI models via the **Data and Insights Cloud Integration (DICI)** Model Context Protocol (MCP).
It is built with **ratatui** for rendering, **tokio** for concurrency, and **mistralrs** for model integration.

---

## 🧠 Design Philosophy

The project is designed with **clear separation of concerns** and **state-driven rendering**:

* **UI components are self-contained** — each view owns its own `state` and knows how to render itself.
* **Asynchronous AI communication** is isolated in an `AiBus` + worker task model.
* **User actions** are mapped directly to key handling, without ambiguous state transitions.
* **State mutation** and **UI rendering** are decoupled, making it easier to extend.
* **Telemetry** provides insight into model performance and usage without interfering with UI flow.

---

## 📐 Core Abstractions

### **Traits**

* **`Render`** — Implemented by the main `DiciAi` struct for full-UI rendering.
* **`RenderArea`** — Implemented by modular view components (e.g., `ChatView`, `InputView`) for drawing into specific rectangular areas.
* **`MessagesExt`** — Extension trait to render chat history into styled terminal lines.

These traits allow **flexible composition** of UI elements without coupling them to global state.

---

### **Main Application Struct**

#### `DiciAi`

* Central orchestrator of application state and rendering.
* Owns:

    * `AiBus` — async communication channels.
    * `Telemetry` — performance metrics.
    * `RequestState` — tracks current/pending requests.
    * `ChatView`, `InputView`, `LegendView`, `StatusView` — UI modules.
    * Optional worker task handle.
* Responsible for:

    * The main event loop (`run_loop`).
    * Handling keyboard input.
    * Dispatching AI requests.
    * Updating views with new data.

---

### **UI State and Views**

#### **State Structs**

* **`ChatState`** — Stores chat history (`Messages`), current partial AI output, scroll position, and height measurements.
* **`InputState`** — Manages the user’s input buffer and `ThinkMode`.
* **`LegendState`** — Tracks conditions for displaying keyboard hints (busy state, pending prompts, undo availability).
* **`StatusState`** — Displays short, high-priority messages (status updates, errors).

#### **View Structs**

Each view wraps its state and implements `RenderArea`:

* **`ChatView`** — Displays conversation history and partial AI output.
* **`InputView`** — Displays the current input buffer.
* **`LegendView`** — Displays dynamic keyboard hints based on app state.
* **`StatusView`** — Displays current status/telemetry message.

---

### **AI Communication**

#### `AiBus`

* Holds channels for:

    * Sending commands to the worker (`cmd_tx`).
    * Receiving events from the worker (`evt_rx`).
    * Canceling in-progress work (`cancel_tx`).

#### `spawn_ai_worker`

* Spawns a dedicated async task for handling AI requests.
* Maintains streaming connection to the model, sending incremental output (`Token` events), usage stats (`Usage`), and completion (`Done`).

---

### **Enums**

#### **`ThinkMode`**

* `Think` — Display `<think>` reasoning segments in the output.
* `NoThink` — Hide reasoning segments.
* Toggles via `Ctrl+T`.

#### **`AiCmd`**

* Represents commands sent to the AI worker, e.g.:

    * `Ask { id, history, thinking }`

#### **`AiEvt`**

* Represents events sent from the AI worker to the UI:

    * `Token(id, delta)` — incremental text output.
    * `Usage(id, Usage)` — telemetry update.
    * `Done(id, String)` — final AI output.
    * `Error(id, Error)` — error during processing.

---

### **Telemetry**

* Tracks:

    * Token counts and completion time.
    * Whether the model is in "thinking" phase.
    * End-to-end elapsed time.
* Produces a **status line** for `StatusView`:

    * `"Thinking..."`, `"Answering..."`, or `"Ready • {tok} tok • {time} • {tok/s}"`.

---

## 📊 Flow Overview

1. **User Input** — Typed into `InputView` buffer.
2. **Submit (Enter)** — Creates a `PendingReq` in `RequestState`.
3. **Dispatch to Worker** — If idle, sends an `AiCmd::Ask` over `AiBus.cmd_tx`.
4. **Streaming Response** — Worker sends `AiEvt::Token` events for each chunk.
5. **Finalization** — Worker sends `AiEvt::Done` or `AiEvt::Error`.
6. **UI Update** — Views re-render with new state.