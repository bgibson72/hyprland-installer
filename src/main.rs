use inquire::{Select, MultiSelect, Confirm, Text};
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;

#[derive(Debug, Clone)]
struct InstallConfig {
    dry_run: bool,
    aur_helper: Option<String>,
    greeter: Option<String>,
    gpu_driver: Option<String>,
    hyprland_version: Option<String>,
    uwsm: bool,
    xdg_user_dirs: bool,
    terminal: Option<String>,
    shell: Option<String>,
    notification_daemon: Option<String>,
    audio: Option<String>,
    xdg_portal: bool,
    auth_agent: Option<String>,
    qt_support: bool,
    status_bar: Option<String>,
    wallpaper_utils: Vec<String>,
    app_launcher: Option<String>,
    color_picker: Option<String>,
    clipboard_manager: Option<String>,
    gui_file_manager: Option<String>,
    tui_file_manager: Option<String>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            dry_run: false,
            aur_helper: None,
            greeter: None,
            gpu_driver: None,
            hyprland_version: None,
            uwsm: false,
            xdg_user_dirs: false,
            terminal: None,
            shell: None,
            notification_daemon: None,
            audio: None,
            xdg_portal: false,
            auth_agent: None,
            qt_support: false,
            status_bar: None,
            wallpaper_utils: Vec::new(),
            app_launcher: None,
            color_picker: None,
            clipboard_manager: None,
            gui_file_manager: None,
            tui_file_manager: None,
        }
    }
}

fn main() {
    println!("\n╔═════════════════════════════════════════════╗");
    println!("║        Arch Linux Hyprland Installer        ║");
    println!("╚═════════════════════════════════════════════╝\n");
    
    let mut config = InstallConfig::default();
    
    // Ask about dry run mode
    match Confirm::new("Run in DRY RUN mode? (No actual installation or file changes)")
        .with_default(false)
        .prompt()
    {
        Ok(true) => {
            config.dry_run = true;
            println!("\n DRY RUN MODE ENABLED - No changes will be made to your system\n");
        }
        Ok(false) => {
            if !check_root() {
                println!("⚠️  This installer should be run as root or with sudo privileges.");
                println!("Some package installations may fail without proper permissions.\n");
            }
        }
        Err(_) => {
            if !check_root() {
                println!("⚠️  This installer should be run as root or with sudo privileges.");
                println!("Some package installations may fail without proper permissions.\n");
            }
        }
    }

    // Step through each configuration option
    select_aur_helper(&mut config);
    select_greeter(&mut config);
    select_gpu_driver(&mut config);
    select_hyprland_version(&mut config);
    select_xdg_user_dirs(&mut config);
    select_uwsm(&mut config);
    select_terminal_shell(&mut config);
    select_notification_daemon(&mut config);
    select_audio(&mut config);
    select_xdg_portal(&mut config);
    select_auth_agent(&mut config);
    select_qt_support(&mut config);
    select_status_bar(&mut config);
    select_wallpaper_utils(&mut config);
    select_app_launcher(&mut config);
    select_color_picker(&mut config);
    select_clipboard_manager(&mut config);
    select_file_manager(&mut config);

    // Summary and confirmation
    display_summary(&config);
    
    if confirm_installation(&config) {
        perform_installation(&config);
        
        // Update hyprland.conf with exec-once statements
        if config.dry_run {
            println!("\n🧪 DRY RUN: Showing what would be added to hyprland.conf...");
            show_config_preview(&config);
        } else if Confirm::new("Would you like to update your hyprland.conf with exec-once statements?")
            .with_default(true)
            .prompt()
            .unwrap_or(false)
        {
            update_hyprland_config(&config);
        }
        
        // Prompt to start Hyprland
        if !config.dry_run {
            println!();
            match Confirm::new("Would you like to start Hyprland now?")
                .with_default(false)
                .prompt()
            {
                Ok(true) => {
                    println!("\n🚀 Starting Hyprland...\n");
                    let _ = Command::new("Hyprland").exec();
                    // exec() replaces the current process, so this line won't be reached
                }
                Ok(false) => {
                    println!("\n✓ Installation complete. Start Hyprland later by running: Hyprland");
                }
                Err(_) => {
                    println!("\n✓ Installation complete. Start Hyprland later by running: Hyprland");
                }
            }
        }
    } else {
        println!("\n❌ Installation cancelled.");
    }
}

fn check_root() -> bool {
    let output = Command::new("id")
        .arg("-u")
        .output()
        .expect("Failed to check user ID");
    
    let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    uid == "0"
}

fn select_aur_helper(config: &mut InstallConfig) {
    println!("\n═══ Step 1: AUR Helper ═══");
    
    // Check if yay or paru is already installed
    let yay_installed = Command::new("which").arg("yay").output().map(|o| o.status.success()).unwrap_or(false);
    let paru_installed = Command::new("which").arg("paru").output().map(|o| o.status.success()).unwrap_or(false);
    
    if yay_installed || paru_installed {
        let installed = if yay_installed { "yay" } else { "paru" };
        println!("✓ AUR helper already installed: {}", installed);
        
        match Confirm::new(&format!("Keep using {}?", installed))
            .with_default(true)
            .prompt()
        {
            Ok(true) => {
                config.aur_helper = Some(installed.to_string());
                return;
            }
            _ => {}
        }
    }
    
    let options = vec!["yay (recommended)", "paru", "SKIP (install manually later)"];
    
    match Select::new("Select an AUR helper to install:", options).prompt() {
        Ok(choice) => {
            config.aur_helper = match choice {
                "SKIP (install manually later)" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.aur_helper = None,
    }
}

fn select_greeter(config: &mut InstallConfig) {
    println!("\n═══ Step 2: Display Manager ═══");
    
    let options = vec!["sddm (default)", "gdm", "lightdm", "greetd", "SKIP"];
    
    match Select::new("Select a display manager:", options).prompt() {
        Ok(choice) => {
            config.greeter = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.greeter = None,
    }
}

fn select_gpu_driver(config: &mut InstallConfig) {
    println!("\n═══ Step 3: GPU Driver Selection ═══");
    
    let options = vec!["nvidia", "amd", "intel", "open-source (mesa)", "SKIP"];
    
    match Select::new("Select your GPU driver:", options).prompt() {
        Ok(choice) => {
            config.gpu_driver = match choice {
                "SKIP" => None,
                option => Some(option.to_string()),
            };
        }
        Err(_) => config.gpu_driver = None,
    }
}

fn select_hyprland_version(config: &mut InstallConfig) {
    println!("\n═══ Step 4: Hyprland Installation ═══");
    
    let options = vec!["hyprland (default)", "hyprland-git", "hyprland-meta", "SKIP"];
    
    match Select::new("Select Hyprland package:", options).prompt() {
        Ok(choice) => {
            config.hyprland_version = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.hyprland_version = None,
    }
}

fn select_xdg_user_dirs(config: &mut InstallConfig) {
    println!("\n═══ Step 5: XDG User Directories ═══");
    println!("Creates standard directories like Documents, Downloads, Pictures, etc.");
    
    match Confirm::new("Install xdg-user-dirs?")
        .with_default(true)
        .prompt()
    {
        Ok(answer) => config.xdg_user_dirs = answer,
        Err(_) => config.xdg_user_dirs = false,
    }
}

fn select_uwsm(config: &mut InstallConfig) {
    println!("\n═══ Step 6: UWSM (Universal Wayland Session Manager) ═══");
    
    match Confirm::new("Install UWSM?")
        .with_default(false)
        .prompt()
    {
        Ok(answer) => config.uwsm = answer,
        Err(_) => config.uwsm = false,
    }
}

fn select_terminal_shell(config: &mut InstallConfig) {
    println!("\n═══ Step 7: Terminal & Shell Selection ═══");
    
    // Terminal selection
    let term_options = vec!["kitty (default)", "foot", "alacritty", "ghostty", "SKIP"];
    
    match Select::new("Select a terminal emulator:", term_options).prompt() {
        Ok(choice) => {
            config.terminal = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.terminal = None,
    }
    
    // Shell selection
    let shell_options = vec!["bash (default)", "zsh", "fish", "SKIP"];
    
    match Select::new("Select a shell:", shell_options).prompt() {
        Ok(choice) => {
            config.shell = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.shell = None,
    }
}

fn select_notification_daemon(config: &mut InstallConfig) {
    println!("\n═══ Step 8: Notification Daemon ═══");
    
    let options = vec!["dunst", "mako", "fnott", "swaync", "SKIP"];
    
    match Select::new("Select a notification daemon:", options).prompt() {
        Ok(choice) => {
            config.notification_daemon = match choice {
                "SKIP" => None,
                option => Some(option.to_string()),
            };
        }
        Err(_) => config.notification_daemon = None,
    }
}

fn select_audio(config: &mut InstallConfig) {
    println!("\n═══ Step 9: Audio System ═══");
    
    let options = vec!["pipewire (recommended)", "pulseaudio", "SKIP"];
    
    match Select::new("Select audio system:", options).prompt() {
        Ok(choice) => {
            config.audio = match choice {
                "SKIP" => None,
                "pipewire (recommended)" => Some("pipewire".to_string()),
                option => Some(option.to_string()),
            };
        }
        Err(_) => config.audio = None,
    }
}

fn select_xdg_portal(config: &mut InstallConfig) {
    println!("\n═══ Step 10: XDG Desktop Portal ═══");
    
    match Confirm::new("Install XDG Desktop Portal (xdg-desktop-portal-hyprland)?")
        .with_default(true)
        .prompt()
    {
        Ok(answer) => config.xdg_portal = answer,
        Err(_) => config.xdg_portal = false,
    }
}

fn select_auth_agent(config: &mut InstallConfig) {
    println!("\n═══ Step 11: Authentication Agent ═══");
    
    let options = vec!["hyprpolkitagent (default)", "polkit-kde-agent", "polkit-gnome", "SKIP"];
    
    match Select::new("Select authentication agent:", options).prompt() {
        Ok(choice) => {
            config.auth_agent = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.auth_agent = None,
    }
}

fn select_qt_support(config: &mut InstallConfig) {
    println!("\n═══ Step 12: Qt5/6 Support ═══");
    
    match Confirm::new("Install Qt5/Qt6 Wayland support?")
        .with_default(true)
        .prompt()
    {
        Ok(answer) => config.qt_support = answer,
        Err(_) => config.qt_support = false,
    }
}

fn select_status_bar(config: &mut InstallConfig) {
    println!("\n═══ Step 13: Status Bar ═══");
    
    let options = vec!["waybar (default)", "polybar", "eww", "ironbar", "SKIP"];
    
    match Select::new("Select a status bar:", options).prompt() {
        Ok(choice) => {
            config.status_bar = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.status_bar = None,
    }
}

fn select_wallpaper_utils(config: &mut InstallConfig) {
    println!("\n═══ Step 14: Wallpaper Utility (Multiple Selection) ═══");
    
    let options = vec![
        "hyprpaper",
        "waypaper",
        "swww",
        "swaybg",
        "mpvpaper",
        "wpaperd",
    ];
    
    match MultiSelect::new("Select wallpaper utilities (Space to select, Enter to confirm):", options).prompt() {
        Ok(choices) => {
            config.wallpaper_utils = choices.iter().map(|s| s.to_string()).collect();
        }
        Err(_) => config.wallpaper_utils = Vec::new(),
    }
}

fn select_app_launcher(config: &mut InstallConfig) {
    println!("\n═══ Step 15: Application Launcher ═══");
    
    let options = vec!["rofi (default)", "wofi", "tofi", "fuzzel", "bemenu", "anyrun", "walker", "SKIP"];
    
    match Select::new("Select an application launcher:", options).prompt() {
        Ok(choice) => {
            config.app_launcher = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.app_launcher = None,
    }
}

fn select_color_picker(config: &mut InstallConfig) {
    println!("\n═══ Step 16: Color Picker ═══");
    
    let options = vec!["hyprpicker (default)", "wl-color-picker", "SKIP"];
    
    match Select::new("Select a color picker:", options).prompt() {
        Ok(choice) => {
            config.color_picker = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.color_picker = None,
    }
}

fn select_clipboard_manager(config: &mut InstallConfig) {
    println!("\n═══ Step 17: Clipboard Manager ═══");
    
    let options = vec!["cliphist (default)", "clipman", "clipse", "copyq", "wl-clip-persist", "SKIP"];
    
    match Select::new("Select a clipboard manager:", options).prompt() {
        Ok(choice) => {
            config.clipboard_manager = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.clipboard_manager = None,
    }
}

fn select_file_manager(config: &mut InstallConfig) {
    println!("\n═══ Step 18: File Manager ═══");
    
    // GUI File Manager
    let gui_options = vec!["dolphin (default)", "nautilus", "nemo", "thunar", "SKIP"];
    
    match Select::new("Select GUI file manager:", gui_options).prompt() {
        Ok(choice) => {
            config.gui_file_manager = match choice {
                "SKIP" => None,
                option => Some(option.split_whitespace().next().unwrap().to_string()),
            };
        }
        Err(_) => config.gui_file_manager = None,
    }
    
    // TUI File Manager
    let tui_options = vec!["lf", "nnn", "ranger", "yazi", "SKIP"];
    
    match Select::new("Select TUI file manager:", tui_options).prompt() {
        Ok(choice) => {
            config.tui_file_manager = match choice {
                "SKIP" => None,
                option => Some(option.to_string()),
            };
        }
        Err(_) => config.tui_file_manager = None,
    }
}

fn display_summary(config: &InstallConfig) {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║           Installation Summary               ║");
    println!("╚══════════════════════════════════════════════╝\n");
    
    if config.dry_run {
        println!("MODE:                 DRY RUN (No changes will be made)\n");
    }
    
    println!("AUR Helper:            {:?}", config.aur_helper.as_ref().unwrap_or(&"None".to_string()));
    println!("Display Manager:       {:?}", config.greeter.as_ref().unwrap_or(&"None".to_string()));
    println!("GPU Driver:            {:?}", config.gpu_driver.as_ref().unwrap_or(&"None".to_string()));
    println!("Hyprland Version:      {:?}", config.hyprland_version.as_ref().unwrap_or(&"None".to_string()));
    println!("XDG User Directories:  {}", if config.xdg_user_dirs { "Yes" } else { "No" });
    println!("UWSM:                  {}", if config.uwsm { "Yes" } else { "No" });
    println!("Terminal:              {:?}", config.terminal.as_ref().unwrap_or(&"None".to_string()));
    println!("Shell:                 {:?}", config.shell.as_ref().unwrap_or(&"None".to_string()));
    println!("Notification Daemon:   {:?}", config.notification_daemon.as_ref().unwrap_or(&"None".to_string()));
    println!("Audio System:          {:?}", config.audio.as_ref().unwrap_or(&"None".to_string()));
    println!("XDG Portal:            {}", if config.xdg_portal { "Yes" } else { "No" });
    println!("Auth Agent:            {:?}", config.auth_agent.as_ref().unwrap_or(&"None".to_string()));
    println!("Qt Support:            {}", if config.qt_support { "Yes" } else { "No" });
    println!("Status Bar:            {:?}", config.status_bar.as_ref().unwrap_or(&"None".to_string()));
    println!("Wallpaper Utils:       {:?}", if config.wallpaper_utils.is_empty() { "None".to_string() } else { config.wallpaper_utils.join(", ") });
    println!("App Launcher:          {:?}", config.app_launcher.as_ref().unwrap_or(&"None".to_string()));
    println!("Color Picker:          {:?}", config.color_picker.as_ref().unwrap_or(&"None".to_string()));
    println!("Clipboard Manager:     {:?}", config.clipboard_manager.as_ref().unwrap_or(&"None".to_string()));
    println!("GUI File Manager:      {:?}", config.gui_file_manager.as_ref().unwrap_or(&"None".to_string()));
    println!("TUI File Manager:      {:?}", config.tui_file_manager.as_ref().unwrap_or(&"None".to_string()));
    println!();
}

fn confirm_installation(config: &InstallConfig) -> bool {
    let prompt = if config.dry_run {
        "Continue with dry run?"
    } else {
        "Proceed with installation?"
    };
    
    match Confirm::new(prompt)
        .with_default(true)
        .prompt()
    {
        Ok(answer) => answer,
        Err(_) => false,
    }
}

fn perform_installation(config: &InstallConfig) {
    if config.dry_run {
        println!("\n DRY RUN: Showing what would be installed...\n");
    } else {
        println!("\n Starting installation...\n");
    }
    
    // Install AUR helper first if needed
    if let Some(ref aur_helper) = config.aur_helper {
        if config.dry_run {
            println!(" Would install AUR helper: {}", aur_helper);
        } else {
            install_aur_helper(aur_helper);
        }
    }
    
    let mut packages = Vec::new();
    let mut aur_packages = Vec::new();
    let mut services_to_enable = Vec::new();
    
    // Collect packages based on configuration
    if let Some(ref greeter) = config.greeter {
        packages.push(greeter.as_str());
        services_to_enable.push(format!("{}.service", greeter));
    }
    
    if let Some(ref gpu) = config.gpu_driver {
        match gpu.as_str() {
            "nvidia" => {
                packages.extend_from_slice(&["nvidia", "nvidia-utils", "nvidia-settings"]);
            }
            "amd" => {
                packages.extend_from_slice(&["mesa", "vulkan-radeon", "libva-mesa-driver"]);
            }
            "intel" => {
                packages.extend_from_slice(&["mesa", "vulkan-intel", "intel-media-driver"]);
            }
            "open-source (mesa)" => {
                packages.push("mesa");
            }
            _ => {}
        }
    }
    
    if let Some(ref hypr) = config.hyprland_version {
        if hypr == "hyprland" {
            packages.push("hyprland");
        } else {
            aur_packages.push(hypr.as_str());
        }
    }
    
    if config.xdg_user_dirs {
        packages.push("xdg-user-dirs");
    }
    
    if config.uwsm {
        aur_packages.push("uwsm");
    }
    
    if let Some(ref term) = config.terminal {
        packages.push(term.as_str());
    }
    
    if let Some(ref shell) = config.shell {
        packages.push(shell.as_str());
    }
    
    if let Some(ref notif) = config.notification_daemon {
        packages.push(notif.as_str());
    }
    
    if let Some(ref audio) = config.audio {
        match audio.as_str() {
            "pipewire" => {
                packages.extend_from_slice(&["pipewire", "pipewire-pulse", "pipewire-alsa", "pipewire-jack", "wireplumber"]);
            }
            "pulseaudio" => {
                packages.extend_from_slice(&["pulseaudio", "pulseaudio-alsa"]);
            }
            _ => {}
        }
    }
    
    if config.xdg_portal {
        packages.extend_from_slice(&["xdg-desktop-portal-hyprland", "xdg-desktop-portal"]);
    }
    
    if let Some(ref auth) = config.auth_agent {
        if auth == "hyprpolkitagent" {
            packages.push("hyprpolkitagent");
        } else {
            packages.push(auth.as_str());
        }
    }
    
    if config.qt_support {
        packages.extend_from_slice(&["qt5-wayland", "qt6-wayland"]);
    }
    
    if let Some(ref bar) = config.status_bar {
        packages.push(bar.as_str());
    }
    
    for wallpaper in &config.wallpaper_utils {
        if wallpaper == "hyprpaper" || wallpaper == "swww" {
            packages.push(wallpaper.as_str());
        } else {
            aur_packages.push(wallpaper.as_str());
        }
    }
    
    if let Some(ref launcher) = config.app_launcher {
        packages.push(launcher.as_str());
    }
    
    if let Some(ref picker) = config.color_picker {
        packages.push(picker.as_str());
    }
    
    if let Some(ref clip) = config.clipboard_manager {
        packages.push(clip.as_str());
        // Most clipboard managers need wl-clipboard
        if clip != "copyq" {
            packages.push("wl-clipboard");
        }
    }
    
    if let Some(ref fm) = config.gui_file_manager {
        packages.push(fm.as_str());
    }
    
    if let Some(ref fm) = config.tui_file_manager {
        packages.push(fm.as_str());
    }
    
    // Install official repo packages
    if !packages.is_empty() {
        if config.dry_run {
            println!(" Would install from official repositories:");
            for pkg in &packages {
                println!("   - {}", pkg);
            }
        } else {
            println!(" Installing packages from official repositories...");
            install_pacman_packages(&packages);
        }
    }
    
    // Install AUR packages
    if !aur_packages.is_empty() {
        if config.dry_run {
            println!("\n Would install from AUR:");
            for pkg in &aur_packages {
                println!("   - {}", pkg);
            }
        } else {
            println!("\n Installing AUR packages...");
            install_aur_packages(&aur_packages);
        }
    }
    
    // Enable services
    if !services_to_enable.is_empty() {
        if config.dry_run {
            println!("\n Would enable services:");
            for service in &services_to_enable {
                println!("   - {}", service);
            }
        } else {
            for service in services_to_enable {
                enable_service(&service);
            }
        }
    }
    
    // Initialize xdg-user-dirs if installed
    if config.xdg_user_dirs {
        if config.dry_run {
            println!("\n Would initialize XDG user directories (Documents, Downloads, Pictures, etc.)");
        } else {
            println!("\n Initializing XDG user directories...");
            if let Some(username) = get_username() {
                let status = Command::new("sudo")
                    .arg("-u")
                    .arg(&username)
                    .arg("xdg-user-dirs-update")
                    .status();
                
                match status {
                    Ok(s) if s.success() => println!("✓ XDG user directories created"),
                    _ => eprintln!("⚠️  Failed to create XDG user directories"),
                }
            }
        }
    }
    
    if config.dry_run {
        println!("\n DRY RUN complete! No changes were made to your system.");
    } else {
        println!("\n Installation complete!");
    }
    
    if !config.dry_run {
        println!("\n Next steps:");
        println!("   1. Review your hyprland.conf at ~/.config/hypr/hyprland.conf");
        println!("   2. Adjust any exec-once paths or parameters as needed");
        println!("   3. Configure wallpaper paths and other personal preferences");
        println!("   4. Reboot your system");
        println!("   5. Select Hyprland from your display manager");
        println!("\n Documentation: https://wiki.hyprland.org/");
    } else {
        println!("\n To perform actual installation:");
        println!("   Run the installer again and answer 'No' to dry run mode");
    }
}


fn install_pacman_packages(packages: &[&str]) {
    let status = Command::new("pacman")
        .arg("-S")
        .arg("--needed")
        .arg("--noconfirm")
        .args(packages)
        .status();
    
    match status {
        Ok(status) if status.success() => {
            println!("✓ Packages installed successfully");
        }
        _ => {
            eprintln!("⚠️  Some packages may have failed to install");
        }
    }
}

fn install_aur_packages(packages: &[&str]) {
    // Try yay first, then paru
    let aur_helper = if Command::new("yay").arg("--version").output().is_ok() {
        "yay"
    } else if Command::new("paru").arg("--version").output().is_ok() {
        "paru"
    } else {
        eprintln!("⚠️  No AUR helper found (yay/paru). Please install AUR packages manually:");
        for pkg in packages {
            eprintln!("   - {}", pkg);
        }
        return;
    };
    
    let status = Command::new(aur_helper)
        .arg("-S")
        .arg("--needed")
        .arg("--noconfirm")
        .args(packages)
        .status();
    
    match status {
        Ok(status) if status.success() => {
            println!("✓ AUR packages installed successfully");
        }
        _ => {
            eprintln!("⚠️  Some AUR packages may have failed to install");
        }
    }
}

fn install_aur_helper(helper: &str) {
    println!(" Installing AUR helper: {}", helper);
    
    // Check if already installed
    if Command::new("which").arg(helper).output().map(|o| o.status.success()).unwrap_or(false) {
        println!("✓ {} is already installed", helper);
        return;
    }
    
    // Get the actual username (not root)
    let username = match get_username() {
        Some(user) => user,
        None => {
            eprintln!("⚠️  Could not determine username. Please install {} manually.", helper);
            return;
        }
    };
    
    if username == "root" {
        eprintln!("⚠️  Cannot build AUR packages as root. Please install {} manually as a regular user.", helper);
        return;
    }
    
    // Install base-devel and git if not present
    println!(" Installing build dependencies...");
    let _ = Command::new("pacman")
        .arg("-S")
        .arg("--needed")
        .arg("--noconfirm")
        .arg("base-devel")
        .arg("git")
        .status();
    
    // Create temporary directory
    let temp_dir = format!("/tmp/{}-install", helper);
    let repo_url = format!("https://aur.archlinux.org/{}.git", helper);
    
    println!(" Cloning {} repository...", helper);
    
    // Clean up any existing directory
    let _ = Command::new("rm").arg("-rf").arg(&temp_dir).status();
    
    // Clone as the regular user
    let clone_status = Command::new("sudo")
        .arg("-u")
        .arg(&username)
        .arg("git")
        .arg("clone")
        .arg(&repo_url)
        .arg(&temp_dir)
        .status();
    
    if clone_status.is_err() || !clone_status.unwrap().success() {
        eprintln!("⚠️  Failed to clone {} repository", helper);
        return;
    }
    
    println!(" Building and installing {}...", helper);
    
    // Build and install as the regular user
    let build_status = Command::new("sudo")
        .arg("-u")
        .arg(&username)
        .arg("bash")
        .arg("-c")
        .arg(format!("cd {} && makepkg -si --noconfirm", temp_dir))
        .status();
    
    match build_status {
        Ok(status) if status.success() => {
            println!(" {} installed successfully!", helper);
            // Clean up
            let _ = Command::new("rm").arg("-rf").arg(&temp_dir).status();
        }
        _ => {
            eprintln!("⚠️  Failed to build/install {}", helper);
            eprintln!("You can manually complete the installation:");
            eprintln!("   cd {}", temp_dir);
            eprintln!("   makepkg -si");
        }
    }
}

fn show_config_preview(config: &InstallConfig) {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║       Hyprland.conf Preview (Dry Run)        ║");
    println!("╚══════════════════════════════════════════════╝\n");
    
    let statements = generate_exec_once_statements(config);
    
    if statements.is_empty() {
        println!("ℹ️  No exec-once statements would be added");
        return;
    }
    
    println!("The following would be added to ~/.config/hypr/hyprland.conf:\n");
    println!("# === AUTO-GENERATED EXEC-ONCE START ===");
    for stmt in statements {
        println!("{}", stmt);
    }
    println!("# === AUTO-GENERATED EXEC-ONCE END ===\n");
}

fn enable_service(service: &str) {
    println!(" Enabling service: {}", service);
    
    let status = Command::new("systemctl")
        .arg("enable")
        .arg(service)
        .status();
    
    match status {
        Ok(status) if status.success() => {
            println!("✓ Service enabled: {}", service);
        }
        _ => {
            eprintln!("⚠️  Failed to enable service: {}", service);
        }
    }
}

fn get_username() -> Option<String> {
    // Try to get username from SUDO_USER environment variable
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        return Some(sudo_user);
    }
    
    // Try to get from USER environment variable
    if let Ok(user) = std::env::var("USER") {
        if user != "root" {
            return Some(user);
        }
    }
    
    // Ask the user
    match Text::new("Enter your username (for config file location):").prompt() {
        Ok(username) => Some(username),
        Err(_) => None,
    }
}

fn get_hyprland_config_path() -> Option<PathBuf> {
    let username = get_username()?;
    
    // Try XDG_CONFIG_HOME first
    let config_path = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config)
    } else {
        PathBuf::from(format!("/home/{}/.config", username))
    };
    
    let hypr_config = config_path.join("hypr/hyprland.conf");
    Some(hypr_config)
}

fn update_hyprland_config(config: &InstallConfig) {
    println!("\n Updating hyprland.conf...\n");
    
    let config_path = match get_hyprland_config_path() {
        Some(path) => path,
        None => {
            eprintln!("⚠️  Could not determine config path");
            return;
        }
    };
    
    // Create hypr directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("⚠️  Failed to create config directory: {}", e);
                return;
            }
        }
    }
    
    // Check if config file exists
    if !config_path.exists() {
        println!("ℹ️  No existing hyprland.conf found at: {}", config_path.display());
        println!("   Hyprland will create a default config on first run.");
        println!("   The exec-once statements will be saved to: ~/.config/hypr/hyprland-autostart.conf");
        println!("   You can include this in your main config with:");
        println!("   source = ~/.config/hypr/hyprland-autostart.conf\n");
        
        // Create a separate autostart config file instead
        let autostart_path = config_path.parent().unwrap().join("hyprland-autostart.conf");
        let exec_statements = generate_exec_once_statements(config);
        
        if exec_statements.is_empty() {
            println!("ℹ️  No exec-once statements to add");
            return;
        }
        
        let autostart_content = format!(
            "# Auto-generated autostart configuration\n# Generated by hyprland-installer\n# Include this in your main hyprland.conf with: source = ~/.config/hypr/hyprland-autostart.conf\n\n{}\n",
            exec_statements.join("\n")
        );
        
        match fs::write(&autostart_path, autostart_content) {
            Ok(_) => {
                println!("✓ Created autostart config at: {}", autostart_path.display());
                println!("\n   To use these settings, add this line to your hyprland.conf:");
                println!("   source = ~/.config/hypr/hyprland-autostart.conf");
                
                // Fix ownership if running as root
                if check_root() {
                    if let Some(username) = get_username() {
                        let _ = Command::new("chown")
                            .arg(format!("{}:{}", username, username))
                            .arg(&autostart_path)
                            .status();
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️  Failed to create autostart config: {}", e);
            }
        }
        return;
    }
    
    // Read existing config
    let mut config_content = {
        // Backup existing config
        let backup_path = config_path.with_extension("conf.backup");
        if let Err(e) = fs::copy(&config_path, &backup_path) {
            eprintln!("⚠️  Failed to create backup: {}", e);
        } else {
            println!("✓ Backed up existing config to: {}", backup_path.display());
        }
        
        match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("⚠️  Failed to read config file: {}", e);
                return;
            }
        }
    };
    
    // Generate exec-once statements
    let exec_statements = generate_exec_once_statements(config);
    
    if exec_statements.is_empty() {
        println!("ℹ️  No exec-once statements to add");
        return;
    }
    
    // Check for existing exec-once section
    let marker_start = "# === AUTO-GENERATED EXEC-ONCE START ===";
    let marker_end = "# === AUTO-GENERATED EXEC-ONCE END ===";
    
    if config_content.contains(marker_start) {
        // Replace existing auto-generated section
        let start_idx = config_content.find(marker_start).unwrap();
        let end_idx = config_content.find(marker_end).unwrap() + marker_end.len();
        
        let new_section = format!(
            "{}\n{}\n{}",
            marker_start,
            exec_statements.join("\n"),
            marker_end
        );
        
        config_content.replace_range(start_idx..end_idx, &new_section);
    } else {
        // Add new auto-generated section
        config_content.push_str("\n\n");
        config_content.push_str(marker_start);
        config_content.push('\n');
        config_content.push_str(&exec_statements.join("\n"));
        config_content.push('\n');
        config_content.push_str(marker_end);
        config_content.push('\n');
    }
    
    // Write updated config
    match fs::write(&config_path, config_content) {
        Ok(_) => {
            println!(" Successfully updated hyprland.conf at: {}", config_path.display());
            println!("\n Added exec-once statements:");
            for stmt in exec_statements {
                println!("   {}", stmt);
            }
            
            // Fix ownership if running as root
            if check_root() {
                if let Some(username) = get_username() {
                    let _ = Command::new("chown")
                        .arg("-R")
                        .arg(format!("{}:{}", username, username))
                        .arg(config_path.parent().unwrap())
                        .status();
                    println!("\n✓ Fixed file ownership for user: {}", username);
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️  Failed to write config file: {}", e);
        }
    }
}

fn generate_exec_once_statements(config: &InstallConfig) -> Vec<String> {
    let mut statements = Vec::new();
    
    // Add NVIDIA-specific environment variables if NVIDIA is selected
    if let Some(ref gpu) = config.gpu_driver {
        if gpu == "nvidia" {
            statements.push("# NVIDIA-specific environment variables".to_string());
            statements.push("env = LIBVA_DRIVER_NAME,nvidia".to_string());
            statements.push("env = XDG_SESSION_TYPE,wayland".to_string());
            statements.push("env = GBM_BACKEND,nvidia-drm".to_string());
            statements.push("env = __GLX_VENDOR_LIBRARY_NAME,nvidia".to_string());
            statements.push("".to_string());
        }
    }
    
    // Notification daemon
    if let Some(ref notif) = config.notification_daemon {
        statements.push("# Notification daemon".to_string());
        let cmd = match notif.as_str() {
            "dunst" => "exec-once = dunst",
            "mako" => "exec-once = mako",
            "fnott" => "exec-once = fnott",
            "swaync" => "exec-once = swaync",
            _ => "",
        };
        if !cmd.is_empty() {
            statements.push(cmd.to_string());
            statements.push("".to_string());
        }
    }
    
    // Audio system
    if let Some(ref audio) = config.audio {
        if audio == "pipewire" {
            statements.push("# Audio system (PipeWire)".to_string());
            statements.push("exec-once = /usr/bin/pipewire".to_string());
            statements.push("exec-once = /usr/bin/pipewire-pulse".to_string());
            statements.push("exec-once = /usr/bin/wireplumber".to_string());
            statements.push("".to_string());
        }
    }
    
    // XDG Desktop Portal
    if config.xdg_portal {
        statements.push("# XDG Desktop Portal".to_string());
        statements.push("exec-once = dbus-update-activation-environment --systemd WAYLAND_DISPLAY XDG_CURRENT_DESKTOP".to_string());
        statements.push("exec-once = systemctl --user import-environment WAYLAND_DISPLAY XDG_CURRENT_DESKTOP".to_string());
        statements.push("".to_string());
    }
    
    // Authentication agent
    if let Some(ref auth) = config.auth_agent {
        statements.push("# Authentication agent".to_string());
        let cmd = match auth.as_str() {
            "hyprpolkitagent" => "exec-once = hyprpolkitagent",
            "polkit-kde-agent" => "exec-once = /usr/lib/polkit-kde-authentication-agent-1",
            "polkit-gnome" => "exec-once = /usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1",
            _ => "",
        };
        if !cmd.is_empty() {
            statements.push(cmd.to_string());
            statements.push("".to_string());
        }
    }
    
    // Status bar
    if let Some(ref bar) = config.status_bar {
        statements.push("# Status bar".to_string());
        let cmd = match bar.as_str() {
            "waybar" => "exec-once = waybar",
            "polybar" => "exec-once = polybar",
            "eww" => "exec-once = eww daemon && eww open bar",
            "ironbar" => "exec-once = ironbar",
            _ => "",
        };
        if !cmd.is_empty() {
            statements.push(cmd.to_string());
            statements.push("".to_string());
        }
    }
    
    // Wallpaper utilities
    if !config.wallpaper_utils.is_empty() {
        statements.push("# Wallpaper utilities".to_string());
        for wallpaper in &config.wallpaper_utils {
            let cmd = match wallpaper.as_str() {
                "hyprpaper" => "exec-once = hyprpaper",
                "swww" => "exec-once = swww-daemon",
                "swaybg" => "exec-once = swaybg -i /path/to/wallpaper.png  # Update path",
                "mpvpaper" => "exec-once = mpvpaper '*' /path/to/video.mp4  # Update path",
                "wpaperd" => "exec-once = wpaperd",
                _ => "",
            };
            if !cmd.is_empty() {
                statements.push(cmd.to_string());
            }
        }
        statements.push("".to_string());
    }
    
    // Clipboard manager
    if let Some(ref clip) = config.clipboard_manager {
        statements.push("# Clipboard manager".to_string());
        let cmd = match clip.as_str() {
            "cliphist" => "exec-once = wl-paste --type text --watch cliphist store",
            "clipman" => "exec-once = wl-paste -t text --watch clipman store",
            "clipse" => "exec-once = clipse -listen",
            "copyq" => "exec-once = copyq",
            "wl-clip-persist" => "exec-once = wl-clip-persist --clipboard both",
            _ => "",
        };
        if !cmd.is_empty() {
            statements.push(cmd.to_string());
            statements.push("".to_string());
        }
    }
    
    // UWSM
    if config.uwsm {
        statements.push("# UWSM is configured via systemd units".to_string());
        statements.push("".to_string());
    }
    
    statements
}
