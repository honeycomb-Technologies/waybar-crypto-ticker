# Contributing to waybar-crypto-ticker

Thanks for your interest in contributing! This project is open to improvements.

## Quick Start

```bash
git clone https://github.com/BurgessTG/waybar-crypto-ticker
cd waybar-crypto-ticker
cargo build
```

## Project Structure

```
src/
├── main.rs       # GTK4 window, layer shell, rendering loop
├── config.rs     # TOML configuration parsing
├── ticker.rs     # Price state management and display formatting
├── websocket.rs  # Kraken WebSocket connection
└── hyprland.rs   # Hyprland IPC for fullscreen detection
```

## Adding Features

### Adding a new cryptocurrency exchange

1. Create a new module in `src/` (e.g., `binance.rs`)
2. Implement the WebSocket connection similar to `websocket.rs`
3. Add exchange selection to `config.rs`

### Adding new configuration options

1. Add the field to the appropriate struct in `config.rs`
2. Add TOML parsing in `ConfigFile` structs
3. Use the new config value in the relevant module
4. Update `config.example.toml` with documentation

### Styling/theming

The ticker uses Cairo for rendering. Colors and fonts are configurable via TOML. To add new visual options:

1. Add config fields in `config.rs` under `Appearance`
2. Use them in the `set_draw_func` closure in `main.rs`

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Keep functions focused and well-documented

## Testing

```bash
# Build and run locally
cargo run

# Release build
cargo build --release
```

## Pull Requests

1. Fork the repo
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run `cargo fmt && cargo clippy`
5. Commit with a descriptive message
6. Push and open a PR

## Ideas for Contribution

- [ ] Add more exchanges (Binance, Coinbase, etc.)
- [ ] Support for stocks via Yahoo Finance API
- [ ] Configurable update intervals
- [ ] Click-to-open price chart
- [ ] Notification on significant price changes
- [ ] Sway/wlroots support (non-Hyprland)
- [ ] Package for AUR, Fedora COPR, etc.

## Questions?

Open an issue on GitHub!
