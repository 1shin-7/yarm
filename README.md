# Yarm (Yet Another Resolution Manager)

![Socialify](https://socialify.git.ci/1shin-7/yarm/image?description=1&descriptionEditable=A%20modern%2C%20minimalist%20screen%20resolution%20%26%20orientation%20manager%20for%20Windows.&font=Inter&language=1&name=1&owner=1&pattern=Plus&theme=Light)

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/1shin-7/yarm/rust.yml?branch=main)](https://github.com/1shin-7/yarm/actions)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue.svg)](https://github.com/rust-lang/rustfmt)

</div>

> "I frequently use a 4K monitor for browsing the web and coding, but my laptop's performance isn't sufficient to run games at 4K resolution. Since adjusting the resolution through Windows settings every time is too cumbersome, the idea for this tool was born."

**Yarm** is a lightweight, Rust-based utility designed to make managing display settings on Windows effortless. Whether you need to quickly switch resolutions for gaming or rotate your screen for coding, Yarm provides both a modern GUI and a scriptable CLI to get the job done.

## ✨ Features

*   **Modern GUI**: A clean, "Sea Salt Blue" themed interface built with [Iced](https://github.com/iced-rs/iced), featuring a floating profile sidebar and rounded UI elements.
*   **Resolution Management**: Quickly list and apply supported resolutions for all connected monitors.
*   **Orientation Switcher**: An intuitive, 4-way segmented control to rotate your display (0°, 90°, 180°, 270°) instantly.
*   **Profiles**: Save your favorite multi-monitor setups (resolution + orientation) as named profiles.
*   **CLI Support**: Use the command line to list or switch profiles, making it easy to integrate with scripts or stream decks.
    *   `yarm switch <profile_name>`
    *   `yarm list`
*   **Detailed Info**: Displays real monitor names (e.g., "Dell U2415" instead of "Generic PnP Monitor") and primary status.

## 🚀 Installation

### Prerequisites
*   Windows 10/11
*   [Rust](https://www.rust-lang.org/tools/install) installed (for building from source)

### Building from Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/1shin-7/yarm.git
    cd yarm
    ```

2.  Build the project:
    ```bash
    cargo build --release
    ```

3.  Run the executable:
    ```bash
    ./target/release/yarm.exe
    ```

## 📖 Usage

### GUI Mode
Simply run `yarm.exe` without arguments to open the graphical interface.
1.  **Adjust Settings**: Select resolutions and orientations for each monitor.
2.  **Apply**: Click "Apply Changes" to test them immediately.
3.  **Save Profile**: Click "+ Save Profile", enter a name, and confirm to save the current snapshot.
4.  **Load Profile**: Click any profile name in the sidebar to load its settings into the staging area (click Apply to set them).

### CLI Mode
Yarm is automation-friendly.

*   **List all saved profiles:**
    ```powershell
    yarm list
    ```

*   **Switch to a specific profile:**
    ```powershell
    yarm switch "Gaming Mode"
    ```

*   **Debug mode:**
    ```powershell
    yarm --debug
    ```

## 📝 TODO

*   [ ] **Subcommand `run`**: Implement a watcher or launcher that accepts an application path. It would automatically apply a specific display profile when the application starts (based on rules in `config.toml`) and revert when it closes.
    *   Example: `yarm run --app "C:\Games\Cyberpunk.exe" --profile "1080p-Gaming"`

## 🤝 Credits

This project was crafted with the assistance of **Google Gemini**, helping to generate code, refactor structure, and polish the UI design.

**Built with:**
*   [**Rust**](https://www.rust-lang.org/): For performance and safety.
*   [**Iced**](https://github.com/iced-rs/iced): For the cross-platform GUI.
*   [**Clap**](https://github.com/clap-rs/clap): For the command-line interface.
*   [**Windows-rs**](https://github.com/microsoft/windows-rs): For interacting with Windows Display APIs.
*   **Clippy**: For keeping the code idiomatic and clean.

---

<div align="center">
Made with ❤️ by <a href="https://github.com/1shin-7">1shin-7</a>
</div>
