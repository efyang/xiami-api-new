extern crate hyper;
#[macro_use]
extern crate json;
use hyper::client::Client;
use hyper::header::*;
use std::io::Read;
use json::JsonValue;

const API_URL: &'static str = "http://api.xiami.com/web?";

fn default_xiami_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set(Cookie(vec!["user_from=2;XMPLAYER_addSongsToggler=0;XMPLAYER_isOpen=0;\
                             _xiamitoken=cb8bfadfe130abdbf5e2282c30f0b39a;"
                                .to_string()]));
    headers.set(Referer("http://h.xiami.com/".to_string()));
    headers.set(UserAgent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like \
                           Gecko) Chrome/55.0.2883.75 Safari/537.36"
        .to_string()));
    headers
}

pub struct XiamiClient {
    client: Client,
}

impl XiamiClient {
    pub fn new() -> XiamiClient {
        XiamiClient { client: Client::new() }
    }

    pub fn get_xiami(&self, querystring: String) {
        let mut resp = String::new();
        self.client
            .post(&format!("{}{}", API_URL, querystring))
            .headers(default_xiami_headers())
            .send()
            .unwrap()
            .read_to_string(&mut resp)
            .unwrap();
        println!("{:#?}", json::parse(&resp));
    }

    pub fn search_song<S: AsRef<str>>(&self, key: S, limit: usize, page: usize) {
        let params = object!{
            "v" => "2.0",
            "key" => key.as_ref(),
            "limit" => limit,
            "page" => page,
            "r" => "search/songs",
            "app_key" => 1
        };
        self.get_xiami(json_to_urlstring(params));
    }

    pub fn get_playlist_by_hot(&self) {
        let params = object!{
            "v" => "2.0",
            "r" => "collect/recommend",
            "app_key" => 1
        };
        self.get_xiami(json_to_urlstring(params));
    }

    pub fn get_songs_by_artist(&self, artist_id: usize, limit: usize, page: usize) {
        let params = object!{
            "v" => "2.0",
            "id" => artist_id,
            "page" => page,
            "limit" => limit,
            "app_key" => 1,
            "r" => "artist/hot-songs"
        };
        self.get_xiami(json_to_urlstring(params));
    }

    //pub fn get_song(&self, id: usize) {
        //self.client.
    //}
}

fn json_to_urlstring(json: JsonValue) -> String {
    let parsed = json.entries().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>();
    parsed.join("&")
}
