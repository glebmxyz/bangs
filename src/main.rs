use phf::phf_map;
use std::env;
use tiny_http::{Header, Response, Server, StatusCode};

struct ShortcutURLs(Option<&'static str>, Option<&'static str>);

static SHORTCUTS: phf::Map<&'static str, ShortcutURLs> = phf_map! {
    "!rust" => ShortcutURLs(None, Some("https://doc.rust-lang.org/std/?search=%s")),
    "!docsrs" => ShortcutURLs(None, Some("https://docs.rs/releases/search?query=%s")),
    "!cargo" => ShortcutURLs(None, Some("https://crates.io/search?q=%s")),
    "!w" => ShortcutURLs(Some("https://en.wikipedia.org/"), Some("https://en.wikipedia.org/wiki/Special:Search?go=Go&search=%s&ns0=1")),
    "!wr" => ShortcutURLs(Some("https://ru.wikipedia.org/"), Some("https://ru.wikipedia.org/w/index.php?search=%s&ns0=1")),
    "!aw" => ShortcutURLs(Some("https://wiki.archlinux.org/"), Some("https://wiki.archlinux.org/index.php?search=%s")),
    "!aur" => ShortcutURLs(None, Some("https://aur.archlinux.org/packages?K=%s")),
    "!yt" => ShortcutURLs(Some("https://youtube.com/"), Some("https://www.youtube.com/results?search_query=%s")),
    "!gh" => ShortcutURLs(Some("https://github.com/"), Some("https://github.com/search?q=%s")),
    "!g" => ShortcutURLs(Some("https://google.com/"), Some("https://google.com/search?q=%s")),
    "!ya" => ShortcutURLs(Some("https://ya.ru/"), Some("https://yandex.ru/search/?text=%s")),
    "!rc" => ShortcutURLs(None, Some("https://context.reverso.net/translation/english-russian/%s")),
    "!py" => ShortcutURLs(None, Some("https://docs.python.org/3/search.html?q=%s")),
};

static DEFAULT_SEARCH_URL: &str = "https://duckduckgo.com/?q=%s";

fn get_redirect_location(full_query: &str) -> String {
    let mut words: Vec<&str> = full_query.split('+').collect();
    let mut shortcut = None;

    for (i, word) in words.iter().enumerate() {
        let val = SHORTCUTS.get(*word);
        if let Some(urls) = val {
            shortcut = Some((i, urls));
            break;
        }
    }

    if let Some((idx, urls)) = shortcut {
        if words.len() > 1 {
            if let Some(search_url) = urls.1 {
                words.remove(idx);
                let query = words.join("+");
                return search_url.replace("%s", &query);
            }
        } else if let Some(site_url) = urls.0 {
            return site_url.to_string();
        }
    };

    DEFAULT_SEARCH_URL.replace("%s", full_query)
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
