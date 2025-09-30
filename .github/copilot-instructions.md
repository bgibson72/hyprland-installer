# Copilot Instructions for Hyprland Installer

## Project Overview

This is an interactive Rust-based installer for setting up Hyprland (a dynamic tiling Wayland compositor) on Arch Linux. The installer provides a comprehensive TUI (Terminal User Interface) that guides users through 19 configuration steps to set up a complete Hyprland environment.

### Key Features
- Interactive terminal UI using the `inquire` crate
- Dry run mode for testing without system changes
- Automatic configuration file generation (hyprland.conf)
- Package management for both official repos and AUR
- Auto-detection of NVIDIA GPUs and appropriate configuration
- Configuration backup before making changes

## Code Structure

### Main Components
- `InstallConfig` struct: Central configuration state for all installer options
- Step functions (e.g., `select_aur_helper`, `select_terminal_shell`): Interactive prompts for each configuration category
- Installation functions: Handle package installation and system configuration
- Configuration generation: Auto-generates `hyprland.conf` exec-once statements

## Coding Standards

### Rust Style
- Follow standard Rust formatting conventions (use `rustfmt`)
- Use descriptive function names that start with a verb (e.g., `select_`, `install_`, `generate_`)
- Keep functions focused on a single responsibility
- Use `Option<String>` for optional configuration choices
- Use `Vec<String>` for multi-select options (e.g., wallpaper utilities)

### Error Handling
- Use `Result` types for operations that can fail
- Provide user-friendly error messages with emoji indicators (⚠️, ❌, ✅)
- Handle prompt errors gracefully with `.unwrap_or()` or `.unwrap_or_default()`
- Never panic in production code; always provide fallback behavior

### User Experience
- Always provide clear, descriptive prompts
- Include default values for common choices
- Show progress indicators during installations
- Use consistent formatting with box-drawing characters for visual clarity
- Respect dry run mode in all operations that modify the system

## Important Patterns

### Interactive Prompts
Use the `inquire` crate patterns:
```rust
// Single selection
Select::new("Prompt text:", options).prompt()

// Multiple selection
MultiSelect::new("Prompt text:", options).prompt()

// Confirmation
Confirm::new("Question?").with_default(true).prompt()
```

### Configuration Options
- Always provide a "SKIP" option for optional steps
- Store selections in `InstallConfig` struct
- Handle None values appropriately in installation logic

### Auto-Configuration
The installer automatically generates `hyprland.conf` sections:
- Marks auto-generated sections with `# === AUTO-GENERATED EXEC-ONCE START/END ===`
- Backs up existing config files before modification
- Preserves manual changes outside marked sections
- Handles NVIDIA-specific environment variables

### Package Management
- Separate official repo packages from AUR packages
- Install official packages with `pacman`
- Install AUR packages using selected AUR helper (yay/paru)
- Build AUR helpers from source if not installed
- Respect dry run mode (show what would be installed)

## Arch Linux & Hyprland Specifics

### Package Names
- Know the difference between official repo packages and AUR packages
- Common AUR packages: hyprland-git, uwsm, ghostty
- Official packages: hyprland, waybar, kitty, dunst, etc.

### Hyprland Configuration
- Config location: `~/.config/hypr/hyprland.conf`
- Use `exec-once` for startup applications
- Include environment variables for NVIDIA setups
- Proper path handling for user directories

### System Integration
- Handle root vs. user permissions correctly
- Fix file ownership when running as root
- Enable systemd services where appropriate
- Create XDG user directories if requested

## Testing Guidelines

### Manual Testing
- Always test in dry run mode first
- Test each configuration step independently
- Verify config file generation without actual installation
- Check that SKIP options work correctly

### Edge Cases to Consider
- Running as root vs. regular user
- Existing vs. fresh Hyprland installations
- Different GPU types (NVIDIA, AMD, Intel, none)
- Missing or conflicting packages
- Invalid config file states

## When Making Changes

### Adding New Configuration Options
1. Add field to `InstallConfig` struct
2. Initialize in `Default` impl
3. Create a `select_*` function for user interaction
4. Add to main configuration flow
5. Handle in `perform_installation` function
6. Update `generate_exec_once_statements` if applicable
7. Update README.md documentation

### Modifying Package Lists
- Verify package names in Arch repos or AUR
- Test installation in both dry run and actual modes
- Consider package dependencies
- Update help text if package behavior changes

### Configuration File Changes
- Always backup before modification
- Preserve user's manual changes
- Use clear markers for auto-generated sections
- Test on empty and existing config files
- Handle file ownership properly

## Documentation

- Keep README.md in sync with code changes
- Update feature list when adding capabilities
- Provide example configurations for new options
- Include troubleshooting tips for common issues

## Security Considerations

- Never store or log sensitive information
- Validate all user inputs
- Be cautious with command execution
- Properly escape shell arguments
- Check permissions before file operations
- Warn users about running as root

## Common Pitfalls to Avoid

- Don't assume package availability without checking
- Don't modify config files without backup
- Don't silently fail; always inform the user
- Don't hardcode paths that may vary per user
- Don't forget to handle the dry run flag
- Don't break existing auto-generated config sections

## Resources

- [Hyprland Wiki](https://wiki.hyprland.org)
- [Arch Wiki](https://wiki.archlinux.org)
- [Hyprland GitHub](https://github.com/hyprwm/Hyprland)
- [Rust inquire crate docs](https://docs.rs/inquire/)
