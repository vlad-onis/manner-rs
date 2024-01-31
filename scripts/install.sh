#!/bin/bash

# Check for argument
if [ "$1" == "--zsh" ]; then
    config_file=~/.zshrc
elif [ "$1" == "--bash" ]; then
    config_file=~/.bashrc
else
    echo "Invalid or no argument provided. Please use --zsh or --bash."
    exit 1
fi

# Change to your Rust project directory
cd /path/to/your/rust/project

# Install your project using Cargo
cargo install --path .

# Add a custom command or environment variable to the appropriate shell config file
echo 'alias manners="manner-rs"' >> $config_file

# Run the command on terminal opening
echo 'manners' >> $config_file


# Inform the user to manually source the config file
echo "Installation complete. Please open a new $shell_name terminal or run 'source $config_file' to apply the changes."
