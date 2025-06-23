# Phantom Persistence (Rust Implementation)

A Rust implementation of the Phantom Persistence technique, originally discovered and documented by Grant Smith (S1n1st3r) at Phantom Security Group.

## Overview

This library implements a Windows persistence technique that allows applications to survive system reboots, shutdowns, and logoffs without writing to registry keys or requiring elevated privileges. The technique hijacks the Windows shutdown process to ensure the application restarts when the system boots back up.

## How It Works

The Phantom Persistence technique works by:

1. **Registering for restart**: Using `RegisterApplicationRestart` to tell Windows to restart the application after system reboot
2. **Creating a hidden window**: Establishing a message loop to intercept shutdown events
3. **Intercepting shutdown**: Catching `WM_QUERYENDSESSION` messages when shutdown/restart is initiated
4. **Hijacking the process**: Blocking the shutdown, then forcing a restart with `EWX_RESTARTAPPS` flag
5. **Automatic persistence**: The kernel instructs `csrss.exe` to write to the registry late in the shutdown process

## Key Advantages

- **No registry writes**: The application never writes to registry keys itself
- **No elevated privileges**: Only requires standard shutdown privileges (available to all processes)
- **Late-stage execution**: Registry modifications happen so late in shutdown that most monitoring tools have already exited
- **Stealth**: Appears as normal application behavior to security tools

## Usage

### Basic Example

Add this to your `Cargo.toml`:

```toml
[dependencies]
phantom_persist_rs = { git = "https://github.com/Teach2Breach/phantom_persist_rs.git", branch = "main" }
```

```rust
use phantom_persist_rs;

fn main() {
    // Register the application for restart
    phantom_persist_rs::register_application_restart();
    
    // Sleep for 60 seconds to ensure registration
    std::thread::sleep(std::time::Duration::from_secs(60));
    
    // Start the message loop thread to handle shutdown events
    phantom_persist_rs::message_loop_thread();
}
```

### Library Functions

- `register_application_restart()`: Registers the current process for automatic restart
- `message_loop_thread()`: Creates a hidden window and message loop to intercept shutdown events

## Requirements

- Windows operating system
- Rust toolchain
- `winapi` crate with appropriate features

## Important Notes

- This technique is not 100% reliable - hard shutdowns or power outages will prevent restart
- The technique requires the application to be running when shutdown is initiated
- Registry entries are created by `csrss.exe` in `HKCU\Software\Microsoft\Windows\CurrentVersion\RunOnce\Application Restart #<NUMBER>`

## Original Research

This implementation is based on the original research and proof-of-concept by Grant Smith (S1n1st3r) at Phantom Security Group. For the complete technical details, background, and discovery process, please refer to the original blog post:

**[👻 Phantom Persistence - Phantom Security Group](https://blog.phantomsec.tools/phantom-persistence)**

The original blog post includes:
- Discovery process and background
- Detailed technique explanation
- Windows shutdown process analysis
- C++ proof-of-concept code
- Indicators of Compromise (IOCs)
- OPSEC considerations

## License

This project is provided as-is for educational purposes. Please refer to the original blog post for attribution and additional context.

## Credits

- **Original Research**: Grant Smith (S1n1st3r) - President @ Phantom Security Group
- **Original Blog**: [Phantom Persistence](https://blog.phantomsec.tools/phantom-persistence)
- **Rust Implementation**: Based on the original C++ proof-of-concept 