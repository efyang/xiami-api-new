extern crate xiami_api_new;

use xiami_api_new::*;

fn main() {
    let client = XiamiClient::new();
    client.search_song("刘德华", 1, 1);
    //client.get_playlist_by_hot();
}
