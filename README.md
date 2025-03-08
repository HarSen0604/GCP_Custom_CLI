# GCLI
GCLI is an advanced command-line interface (CLI) tool designed to simplify and enhance the way users interact with Google Cloud services. By leveraging the power of both the command line and a graphical user interface (GUI)-like experience, GCLI provides a flexible, user-friendly method for managing Google Cloud resources. This hybrid approach combines the speed and power of a CLI with the intuitive usability of a graphical interface, making it suitable for developers of all levels.


## Tech Stack and features
Built with Rust for core CLI functionality, GCLI offers high performance, reliability, and speed, ensuring a robust and scalable experience. Python is also utilized to provide auxiliary capabilities, such as interfacing with external APIs and enabling advanced functionality. This combination ensures the tool is fast, extensible, and capable of seamlessly managing complex Google Cloud operations.

A standout feature of GCLI is its integration with the Gemini UI, which enhances the user experience by allowing users to interact with the custom CLI through an intuitive graphical interface. This integration enables users to ask questions about the available commands, view detailed information, and receive real-time feedback on their queries, creating a more accessible and efficient workflow. The Gemini UI not only offers a GUI-like experience but also interacts directly with the CLI commands, enabling the best of both worlds for users—whether you're an experienced developer or a beginner. This makes it easier for users to navigate and operate the tool without memorizing every command, ensuring an interactive and seamless experience while managing Google Cloud resources.

## Installation Instructions for macOS
1. Installing Rust
To run the Rust-based CLI tool on macOS, follow these steps:

### Step 1: Install Homebrew
Homebrew is a package manager for macOS, which you can use to install Rust.

1. Open a terminal on your macOS.
2. Install Homebrew by running:

`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

This will install Homebrew. After the installation is complete, ensure that Homebrew is available in your terminal by running:

`brew --version`

### Step 2: Install Rust
Once Homebrew is installed, you can easily install Rust:
Run the following command to install Rust:

`brew install rust`

After the installation is complete, confirm that Rust is installed by checking the version:

`rustc --version`

This should print the installed version of rustc, confirming that Rust is installed correctly.

