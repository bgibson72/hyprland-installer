# Arch Linux Hyprland Installer

A comprehensive, interactive Rust-based installer for setting up Hyprland on Arch Linux with automatic configuration generation.

## Features

✨ **Interactive TUI** - Beautiful terminal-based user interface
🧪 **Dry Run Mode** - Test the installer without making any changes to your system
📦 **Smart Package Management** - Handles both official repos and AUR packages  
🛠️ **AUR Helper Installation** - Automatically builds and installs yay or paru from source
⚙️ **Auto-Configuration** - Automatically updates `hyprland.conf` with exec-once statements  
🔧 **19 Configuration Steps** - Comprehensive coverage of all Hyprland components  
🎯 **Skip Options** - Skip any step to configure manually later  
💾 **Config Backup** - Automatically backs up existing configuration files  
🔐 **Root Detection** - Properly handles permissions and file ownership  
🗂️ **XDG User Directories** - Optionally creates standard user folders  

## What Gets Configured

The installer handles:

1. **AUR Helper** (yay, paru) - Automatically built and installed
2. **Display Manager** (SDDM, GDM, LightDM, greetd)
3. **GPU Drivers** (NVIDIA, AMD, Intel, Mesa)
4. **Hyprland Version** (stable, git, meta)
5. **XDG User Directories** (Documents, Downloads, Pictures, etc.)
6. **UWSM** (Universal Wayland Session Manager)
7. **Terminal & Shell** (Kitty, Alacritty, Foot, Ghostty + Bash/Zsh/Fish)
8. **Notification Daemon** (Dunst, Mako, Fnott, SwayNC)
9. **Audio System** (PipeWire/WirePlumber, PulseAudio)
10. **XDG Desktop Portal** (Hyprland portal)
11. **Authentication Agent** (Polkit agents)
12. **Qt Support** (Qt5/Qt6 Wayland)
13. **Status Bar** (Waybar, Polybar, Eww, Ironbar)
14. **Wallpaper Utilities** (Multiple selections possible)
15. **App Launcher** (Rofi, Wofi, Tofi, Fuzzel, etc.)
16. **Color Picker** (Hyprpicker, etc.)
17. **Clipboard Manager** (Cliphist, Clipman, etc.)
18. **File Managers** (GUI and/or TUI - can select both!)
19. **Web Browser** (Firefox, Chromium, Brave, LibreWolf, Edge, Opera)

## Prerequisites

- Arch Linux minimal installation
- Internet connection
- Root/sudo privileges
- **No Rust installation required** (if using pre-compiled binary)

## Installation

### Option 1: Download Pre-compiled Binary (Recommended)

**No Rust installation needed!** Just download and run:
```bash
# Download the latest release
wget https://github.com/YOUR_USERNAME/YOUR_REPO/releases/latest/download/hyprland-installer-x86_64-linux

# Make it executable
chmod +x hyprland-installer-x86_64-linux

# Run the installer
sudo ./hyprland-installer-x86_64-linux
