# Clock
<div align="center">
  <img src="assets/clock.gif" alt="Clock Image" />
  
  <br/>
  
  ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Version](https://img.shields.io/github/v/release/evilenx/clock?style=for-the-badge)](https://github.com/evilenx/clock/releases)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/evilenx/clock/ci.yml?style=for-the-badge)](https://github.com/evilenx/clock/actions)
</div>
## Description
  **Clock** is a Rust project that provides a digital clock implementation with nanosecond precision.

## Features
  - High-precision time display with nanoseconds
  - Built in Rust for performance and reliability
  - Simple command-line interface

## Requirements
  - **Rust** (latest stable version recommended)
- **Cargo** (included with Rust)

## Installation

  1. **Clone the repository:**
  ```bash
  git clone https://github.com/evilenx/clock.git
  cd clock
  ```

  2. **Build the project:**
  ```bash
  cargo build --release
  ```

  3. **Run the clock:**
  ```bash
  cargo run --release
  ```

  4. **Using curl**
  ```bash
  curl -sSf https://raw.githubusercontent.com/evilenx/clock/main/install.sh | bash

  ```

## Configuration
  The application requires a configuration file located at `~/.config/clock/config.yml` to set the font size.

  **Create the configuration directory and file:**
  ```bash
  mkdir -p ~/.config/clock
  ```

  **Example `config.yml`:**
  ```yaml
  font_size: 80
  ```

## Usage
  After running the application, the digital clock will display the current time with nanosecond precision in your terminal. Make sure to create the configuration file before running the clock.

## Contributing
  Contributions are welcome! Please feel free to submit a Pull Request.

## License
  This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
