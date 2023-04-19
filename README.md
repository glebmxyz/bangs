# Localhost Alternative to DuckDuckGo !Bangs

## Quick Start
1. Clone repo
2. Change `SHORTCUTS` to suit your needs
2. `cargo build --release`
3. Run `./target/release/bangs <address>` (eg. `127.0.0.1:8000`)
4. Add search engine to browser ([chrome](https://support.google.com/chrome/answer/95426)|[firefox](https://support.mozilla.org/en-US/kb/assign-shortcuts-search-engines)) with URL `http://<address>/?q=%s`
5. Use it with shortcut or as a default search engine
