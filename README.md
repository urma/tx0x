# tx0x

A minimalistic 8-track drum machine TUI inspired by the Roland TR-909.

## Interface

```
tx0x  BPM: 120  ■
      1  2  3  4 │ 5  6  7  8 │ 9 10 11 12 │13 14 15 16
Kick   [█][·][·][█]│[█][·][·][█]│[█][·][·][█]│[█][·][·][█]
Snare  [·][·][·][·]│[·][·][·][·]│[·][·][·][·]│[·][·][·][·]
...
```

## Controls

| Key | Action |
|---|---|
| `Space` | Toggle step at cursor |
| `↑`/`↓` or `j`/`k` | Select track |
| `←`/`→` or `h`/`l` | Move cursor |
| `Enter` | Start/stop playback |
| `+`/`-` | Adjust BPM (40–300) |
| `q` or `Esc` | Quit |

## Build & Run

```sh
# Download samples
scripts/download-samples.sh

# Run
cargo run
```

Uses [ratatui](https://ratatui.rs) for the TUI, [crossterm](https://crates.io/crates/crossterm) for terminal handling, and [rodio](https://crates.io/crates/rodio) for sample playback.
