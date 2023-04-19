use std::env;
use tiny_http::{Header, Response, Server, StatusCode};

fn get_query_url(engine: &str) -> Option<&str> {
    match engine {
        "rust" => Some("https://doc.rust-lang.org/std/?search={}"),
        "docsrs" => Some("https://docs.rs/releases/search?query={}"),
        "cargo" => Some("https://crates.io/search?q={}"),
        "w" => Some("https://en.wikipedia.org/wiki/Special:Search?go=Go&search={}&ns0=1"),
        "wr" => Some("https://ru.wikipedia.org/w/index.php?search={}&ns0=1"),
        "aw" => Some("https://wiki.archlinux.org/index.php?search={}"),
        "aur" => Some("https://aur.archlinux.org/packages?K={}"),
        _ => None,
    }
}

fn get_redirect_location(url: &str) -> Option<String> {
    let (engine, query) = url[1..].split_once("%20")?;
    let query_url = get_query_url(engine)?;

    Some(query_url.replace("{}", query))
}

fn get_response(url: &str) -> Response<std::io::Empty> {
    let location = get_redirect_location(url);

    if let Some(loc) = location {
        let mut resp = Response::empty(StatusCode(301));
        let loc_header = Header::from_bytes(&b"Location"[..], &loc[..]).unwrap();
        resp.add_header(loc_header);

        resp
    } else {
        Response::empty(StatusCode(404))
    }
}

fn main() -> ! {
    let mut address = "127.0.0.1:15000".to_string();

    let mut args = env::args();
    if let Some(addr) = args.nth(1) {
        address = addr
    };

    let server = Server::http(address).unwrap_or_else(|addr| panic!("Invalid address: {addr}"));

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                eprintln!("error: {e}");
                continue;
            }
        };
        let url = request.url().to_owned();

        request.respond(get_response(&url)).unwrap();
    }
}
