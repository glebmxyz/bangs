# Localhost Alternative to DuckDuckGo !Bangs

## Quick Start
1. Clone repo
2. Change `get_query_url` to suit your needs
2. `cargo build --release`
3. Run `./target/release/bangs <address>` (eg. `127.0.0.1:8000`)
4. Add site search shortcut ([chrome](https://support.google.com/chrome/answer/95426)|[firefox](https://support.mozilla.org/en-US/kb/assign-shortcuts-search-engines)) with address `http://<address>/%s` and preferred shortcut
5. Search `<shortcut> <engine> <search query>`
