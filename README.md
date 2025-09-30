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
```

### Option 2: Build from Source
If you prefer to build from source or want to modify the installer:

```bash
# Prerequisites: Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/YOUR_USERNAME/YOUR_REPO.git
cd hyprland-installer

# Build the installer
cargo build --release

# Run the installer
sudo ./target/release/hyprland-installer
```

### Auto-Configuration Features
Hyprland.conf Updates

The installer will:

1. Backup your existing config to hyprland.conf.backup
2. Add exec-once statements for installed components
3. Include environment variables for NVIDIA GPUs
4. Organize statements with helpful comments
5. Mark auto-generated sections for easy updates
6. Fix file ownership if running as root

#### In Dry Run Mode:
The installer will show you a preview of what would be added without modifying any files.

### Example Generated Config
```bash
# === AUTO-GENERATED EXEC-ONCE START ===

# NVIDIA-specific environment variables
env = LIBVA_DRIVER_NAME,nvidia
env = XDG_SESSION_TYPE,wayland
env = GBM_BACKEND,nvidia-drm
env = __GLX_VENDOR_LIBRARY_NAME,nvidia

# Notification daemon
exec-once = dunst

# Audio system (PipeWire)
exec-once = /usr/bin/pipewire
exec-once = /usr/bin/pipewire-pulse
exec-once = /usr/bin/wireplumber

# Authentication agent
exec-once = hyprpolkitagent

# Status bar
exec-once = waybar

# Wallpaper utilities
exec-once = hyprpaper

# Clipboard manager
exec-once = wl-paste --type text --watch cliphist store

# === AUTO-GENERATED EXEC-ONCE END ===
```

### Updating Configuration Later

If you run the installer again, it will:

- Replace the auto-generated section with new settings
- Preserve any manual changes outside the marked section
- Create a new backup before making changes

### Post-Installation

After installation:

1. Review your ~/.config/hypr/hyprland.conf
2. Check your XDG user directories (if installed): Documents, Downloads, Pictures, Videos, Music, Desktop, Templates, Public
3. Adjust any paths (wallpapers, videos, etc.)
4. Configure keybindings for your installed tools
5. Set your preferred theme and colors
6. Reboot and select Hyprland from your display manager

### Useful Configuration Snippets

#### Add Keybindings

Add these to your hyprland.conf:

```bash
# Terminal
bind = SUPER, Return, exec, kitty

# App Launcher
bind = SUPER, D, exec, rofi -show drun

# Clipboard Manager
bind = SUPER, V, exec, cliphist list | rofi -dmenu | cliphist decode | wl-copy

# Color Picker
bind = SUPER, C, exec, hyprpicker -a
```

Configure Wallpaper

For hyprpaper, create ~/.config/hypr/hyprpaper.conf:
```bash
preload = /path/to/wallpaper.png
wallpaper = ,/path/to/wallpaper.png
```

For swww:
```bash
swww img /path/to/wallpaper.png
```

### Troubleshooting

#### Testing Before Installing

Always test with Dry Run mode first! This lets you see exactly what will happen without risking your current setup:
```bash
./hyprland-installer-x86_64-linux
# Answer "Yes" to dry run mode
```

##### AUR Helper Installation Failed
If the automatic AUR helper installation fails, you can install it manually:
```bash
# For yay
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si

# For paru
git clone https://aur.archlinux.org/paru.git
cd paru
makepkg -si
```
Then re-run the installer and select "SKIP" for the AUR helper step.

#### Permission Issues
Make sure to run with sudo:
```bash
sudo ./hyprland-installer-x86_64-linux
```

#### Config Not Updated
The installer will prompt you to update the config. Make sure you answer "Yes" when asked.

#### Services Not Starting
Enable and start services manually:
```bash
systemctl enable sddm.service
systemctl start sddm.service
```

### Resources
[Hyprland Wiki](https://wiki.hypr.land)

[Arch Wiki](https://wiki.archlinux.org)

[Hyprland GitHub](https://github.com/hyprwm/Hyprland)


### Contributing
Feel free to submit issues or pull requests to improve the installer!

### License
MIT License - feel free to modify and distribute


