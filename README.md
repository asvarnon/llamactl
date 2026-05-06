# LLM Manager

A lightweight Rust-based management utility for handling local LLM inference servers (specifically `llama.cpp`).

## 🚀 Architecture Overview

This project is designed with a **Launcher-Wrapper** pattern to keep the core Rust logic simple, robust, and easy to maintain.

### The "Layered" Execution Model
Instead of managing complex CLI arguments and environment variables directly within Rust, the project follows this chain of command:

1.  **`llm-manager` (Rust)**: Acts as the high-level orchestrator. It handles configuration, state management (tracking what is running), and provides a clean CLI interface.
2.  **`cmd.exe` (Windows Shell)**: The Rust application spawns a command shell to execute a designated script.
3.  **`.bat` Wrapper (The Workload)**: A batch script (defined in `llm-config.toml`) contains the actual "heavy lifting"—the specific paths to `llama-server.exe`, quantization parameters, GPU layer counts, and context window settings.
4.  **`llama-server.exe` (The Engine)**: The actual inference engine running the model.

## 🛠 Design Decisions & Nuances

### 1. Why use `.bat` files instead of direct execution?
* **Simplicity**: The Rust codebase remains clean and focused on process management rather than being cluttered with hundreds of lines of model-specific CLI flags.
* **Flexibility**: You can tune model parameters (like `--n-gpu-layers` or `--ctx-size`) by simply editing a text file in the `scripts/` directory, without needing to recompile the Rust application.
* **Portability**: It decouples the *management* of the model from the *configuration* of the model.

### 2. PID Tracking vs. Sledgehammer Termination
A critical nuance in this project is how processes are identified and stopped.

* **The PID Limitation**: When the manager starts a model, it captures the `PID` of the process it spawned. However, because we launch a `.bat` file, that `PID` belongs to `cmd.exe` (the shell runner), **not** the `llama-server.exe` itself.
* **The "Sledgehammer" Approach**: To ensure a clean shutdown, the `stop` command uses `taskkill /IM llama-server.exe /T`. 
    * **Why?** If we only killed the PID of the batch runner, the actual `llama-server.exe` might continue running in the background as an "orphan," hogging GPU memory.
    * **The Result**: By targeting the image name (`/IM`) and its entire tree (`/T`), we guarantee that the actual inference engine is terminated, regardless of the shell that launched it.

## 📋 Configuration

Models are registered in your configuration file (e.g., `llm-config.toml`). 

Example entry:
```toml
[[models]]
name = "my-model"
aliases = ["my-alias"]
script = "path/to/your/script.bat"
```

## 📦 Development

### Prerequisites
* Rust (latest stable)
* Windows (designed for Windows `cmd.exe` and `taskkill`)

### Building
```bash
cargo build --release
```
