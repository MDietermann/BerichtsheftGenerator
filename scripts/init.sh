#!/bin/bash

clear

# Check OS
OS=$(uname -s)

case "$OS" in
"Linux")
    # Check for specific Linux distribution's package manager
    if command -v apt-get &>/dev/null; then
        echo "Detected Debian/Ubuntu. Using apt-get for installation..."
        sudo apt-get update && sudo apt-get install -y git-all
    elif command -v dnf &>/dev/null; then
        echo "Detected Fedora. Using dnf for installation..."
        sudo dnf install -y git-all
    elif command -v pacman &>/dev/null; then
        echo "Detected Arch-Linux. Using pacman for installation..."
        sudo pacman -Syu --noconfirm git
    else
        echo "Unsupported Linux distribution. Please install git manually!"
        exit 1
    fi
    ;;
"Darvin")
    echo "Detected macOS. Using homebrew for installation..."
    # Check if Homebrew is installed, if not prompt for install
    if ! command -v brew &>/dev/null; then
        echo "Homebrew is not installed."
        read -p "Do you want to install it now? [y/N] " -n 1 -r
        echo

        if [[$REPLY =~ ^[Yy]$ ]]; then
            echo "Installing Homebrew..."
            echo /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        else
            echo "Homebrew installation cancelled. Exiting..."
            exit 1
        fi
    fi
    brew install git
    ;;
"MINGW" | "MSYS" | "CYGWIN")
    echo "Detected Git Bash/WSL on Windows. You may already have git. Checking..."
    if ! command -v git &>/dev/null; then
        echo "Git is not installed. Please install it via the official installer or your WSL distribution's package manager."
    else
        echo "Git is already installed."
    fi
    ;;
*)
    echo "Unknown OS: $OS"
    exit 1
    ;;
esac

git --version
