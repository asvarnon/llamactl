# LLM Manager

A lightweight Rust-based management utility for handling local LLM inference servers (specifically `llama.cpp`).

## 🚀 Overview

`llm-manager` provides a clean CLI interface to start, stop, and monitor local LLM inference processes. It uses a **Launcher-Wrapper** architecture, delegating the complex execution logic to lightweight batch scripts.

## 🛠 Prerequisites

* **Windows OS** (Designed for `cmd.exe` and `taskkill`).
* **Rust Toolchain** (latest stable).
* **llama.cpp binaries**: You must have `llama-server.exe` and its dependencies available on your system.

## 📦 Installation & Setup

### Option 1: Build from Source (Recommended for Developers)
1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd llm-manager
   ```
2. **Build the project:**
   ```bash
   cargo build --release
   ```

### Option 2: Download Pre-built Release
1. Go to the [GitHub Releases page](https://github.com/asvarnon/llamactl/releases).
2. Download the latest `.zip` or `.exe` for Windows.
3. Extract the contents to your desired directory.

---

### Post-Installation Steps (Required for both options)

Regardless of how you install, you must complete the following:

1. **Prepare your configuration:**
   Create a file named `llm-config.toml` in the application directory.
2. **Create your model scripts:**
   Create a folder (e.g., `scripts/`) and add `.bat` files that contain the specific `llama-server.exe` commands for your models.

### Recommended Directory Structure
```text
llm-manager/
├── Cargo.toml
├── llm-config.toml       <-- Your model registry
├── src/
├── scripts/              <-- Your model wrappers
│   └── run-model-a.bat   <-- Calls llama-server.exe with specific flags
└── target/
```

## ⚙️ Configuration

Models are defined in a configuration file. You can specify the location of this file using the `--config` flag or by setting the `LLM_CONFIG_PATH` environment variable.

### Configuration Hierarchy
The application resolves the configuration path in this order:
1.  **CLI Flag**: `--config <path>`
2.  **Environment Variable**: `LLM_CONFIG_PATH`
3.  **Default**: `llm-config.toml` in the current directory.

### Example entry:
```toml
[[models]]
name = "my-model"
aliases = ["my-alias"]
script = "path/to/your/script.bat"
```

## ⌨️ Commands

`llamactl` (or `cargo run --`) supports the following commands:

| Command | Description |
| :--- | :--- |
| `<name>` | **Shorthand**: Starts the model matching the name or alias. |
| `start <name>` | Starts a model by name or alias. |
| `stop` | Stops all running `llama-server.exe` processes. |
| `status` | Shows the status of the currently running model (if any). |
| `list` | Lists all configured models and their aliases. |

### 🛠 Global Options

| Option | Description |
| :--- | :--- |
| `--config <path>` | Specify a custom path to the configuration file. |

### Examples

**Start a model (shorthand):**
```bash
llamactl my-model
```

**List all models:**
```bash
llamactl list
```

**Start a model with a custom config:**
```bash
llamactl --config ./custom-config.toml start my-model
```

## 🧠 Architecture & Design Decisions

### The "Launcher-Wrapper" Pattern
To keep the Rust codebase maintainable and avoid "flag bloat," we do not pass complex arguments directly through Rust. Instead:
* **Rust** manages the *lifecycle* (Start/Stop/Monitor).
* **Batch Scripts** manage the *configuration* (Quantization, Layers, Context).

### PID vs. Sledgehammer Termination
A critical design choice was made regarding how processes are stopped:

* **The Problem**: When Rust spawns a `.bat` file, the captured `PID` belongs to the `cmd.exe` shell runner, not the actual `llama-server.exe` process. Killing the shell PID often leaves the inference engine running as an "orphan" process, hogging GPU memory.
* **The Solution (Sledgehammer)**: The `stop` command uses `taskkill /IM llama-server.exe /T`. This targets the actual executable by name and kills its entire process tree, ensuring a clean release of system resources.

## 🛠 Development

### Building
```bash
cargo build --release
```
