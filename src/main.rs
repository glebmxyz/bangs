use phf::phf_map;
use std::env;
use tiny_http::{Header, Response, Server, StatusCode};

static SHORTCUTS: phf::Map<&'static str, &'static str> = phf_map! {
    "!rust" => "https://doc.rust-lang.org/std/?search=%s",
    "!docsrs" => "https://docs.rs/releases/search?query=%s",
    "!cargo" => "https://crates.io/search?q=%s",
    "!w" => "https://en.wikipedia.org/wiki/Special:Search?go=Go&search=%s&ns0=1",
    "!wr" => "https://ru.wikipedia.org/w/index.php?search=%s&ns0=1",
    "!aw" => "https://wiki.archlinux.org/index.php?search=%s",
    "!aur" => "https://aur.archlinux.org/packages?K=%s",
};
static DEFAULT_SEARCH_URL: &str = "https://duckduckgo.com/?q=%s";

fn get_redirect_location(full_query: &str) -> String {
    let (url, query) = match full_query.split_once('+') {
        Some((prefix, query)) if SHORTCUTS.contains_key(prefix) => (SHORTCUTS[prefix], query),
        _ => (DEFAULT_SEARCH_URL, full_query),
    };

    url.replace("%s", query)
}

fn get_redirect(url: &str) -> Response<std::io::Empty> {
    let location = get_redirect_location(url);

    let mut resp = Response::empty(StatusCode(302));
    let loc_header = Header::from_bytes(&b"Location"[..], &location[..]).unwrap();
    resp.add_header(loc_header);

    resp
}

fn main() -> ! {
    let mut address = "127.0.0.1:15000".to_string();

    let mut args = env::args();
    if let Some(addr) = args.nth(1) {
        address = addr
    };

    let server = Server::http(address).unwrap();

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                eprintln!("error: {e}");
                continue;
            }
        };

        let response = if let Some(query) = request.url().strip_prefix("/?q=") {
            get_redirect(query)
        } else {
            Response::empty(StatusCode(400))
        };

        request.respond(response).unwrap();
    }
}
