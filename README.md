# GCLI
GCLI is an advanced command-line interface (CLI) tool designed to simplify and enhance the way users interact with Google Cloud services. 

By leveraging the power of both the command line 


and a graphical user interface (GUI)-like experience, 
<img width="479" alt="image" src="https://github.com/user-attachments/assets/919d07c8-e8cc-475b-85ad-15d67924ac01" />

GCLI provides a flexible, user-friendly method for managing Google Cloud resources. This hybrid approach combines the speed and power of a CLI with the intuitive usability of a graphical interface, making it suitable for developers of all levels.


## Tech Stack and features
Built with Rust for core CLI functionality, GCLI offers high performance, reliability, and speed, ensuring a robust and scalable experience. Python is also utilized to provide auxiliary capabilities, such as interfacing with external APIs and enabling advanced functionality. This combination ensures the tool is fast, extensible, and capable of seamlessly managing complex Google Cloud operations.

In the below image is a quick glance on our custome made cdommands along with the help and also the error handling that we have incorporated in our codebase:

<img width="774" alt="image" src="https://github.com/user-attachments/assets/ae4bb95a-a72a-4b10-98c0-5a7b0ff7d8fd" />


A standout feature of GCLI is its integration with the Gemini UI, which enhances the user experience by allowing users to interact with the custom CLI through an intuitive graphical interface. This integration enables users to ask questions about the available commands, view detailed information, and receive real-time feedback on their queries, creating a more accessible and efficient workflow. The Gemini UI not only offers a GUI-like experience but also interacts directly with the CLI commands, enabling the best of both worlds for users—whether you're an experienced developer or a beginner. This makes it easier for users to navigate and operate the tool without memorizing every command, ensuring an interactive and seamless experience while managing Google Cloud resources.


We have also implemented short hand notations and grouping of similar commands for improved agility and felexibility for seasoned developers.

<img width="670" alt="image" src="https://github.com/user-attachments/assets/38d867f5-f16d-4d83-860d-40e35bdc855b" />
<img width="596" alt="image" src="https://github.com/user-attachments/assets/e3458232-2d0e-4449-853f-e11619027f2b" />


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

## Execution Instructions for Different Modes

### 1. For newDev and experiencedDev
#### Steps for newDev:
1. Copy the code to a directory:
- Create a directory where you want to place your project files.
- Copy the source code (e.g., the Rust project) into the directory.
2. Build the project:
- Open a terminal and navigate to the project directory.
- Run the following command to build the project:
  
`cargo build`

3. Run the project:
- After the build is complete, execute the project using the following command:

`cargo run`

#### Steps for experiencedDev:
1. Copy the code to a directory:
- Copy the source code to your preferred directory.
  
2. Build the project:
- Open a terminal and navigate to the directory containing your project.
- Run the following command to build the project:
  
`cargo build`

3. Run the project:
- Execute the project with the following command:

`cargo run`

### 2.  For Equators[Gemini UI]:
#### Steps for Equators setup:
1. Copy the required JSON files:
- From the directory Prompt, copy the two JSON files to the directory Equators.
2. Install dependencies:
- Make sure to have pip installed. Then, navigate to the Equators directory and install the required Python dependencies by running:

`pip install -r requirements.txt`

3. Run the nlp_gcommand.py script:
- Running in experiencedDev mode:
a. Open a terminal in the Equators directory and run the following command:

`python nlp_gcommand.py --type="experienced"`

- Running in newDev mode:
a. For running in newDev mode, use:

`python nlp_gcommand.py --type="new"`

- Add Gemini API Key:
a. In the nlp_gcommand.py file, locate the section where the Gemini API key is required, and add your API key there to allow the script to communicate with the Gemini API.

## Notes:
**Gemini API Key**: Make sure you have the Gemini API key and place it in the appropriate section of the nlp_gcommand.py file as described in the script's documentation or comments.

**Directory Structure**: Ensure that the directory structure is set up as per the instructions to avoid issues with file paths.

**Python and Rust Versions**: Ensure you have the required versions of Python and Rust installed on your machine.

