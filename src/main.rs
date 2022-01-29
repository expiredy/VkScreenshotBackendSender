use std::{env, io};
use vkrs::*;

fn main() {
    let api = api::Client::new();
    let oauth = api.auth(
        env::var("VK_APP_ID").unwrap(),
        env::var("VK_APP_SECRET").unwrap());

    let auth_uri = oauth.auth_uri(auth::Permission::Audio);

    // Or if you want to get permissions for specific request:
    // let auth_uri = oauth.auth_uri_for::<audio::Search>();
    println!("Go to {} and enter code below...", auth_uri);

    let inp = io::stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf);
        buf
    };

    // You may want to save this token for future use to avoid asking user
    // to authorize the app on each run.
    let token = oauth.request_token(code.trim()).unwrap();

    // The access token is JSON serializable with serde, so you can do it this way:
    // File::create(TOKEN_FILE).ok().map(|mut f| serde_json::to_writer(&mut f, &token).ok()).unwrap();
    //
    // And then you can load it again:
    // let token: auth::AccessToken = File::open(TOKEN_FILE).ok().and_then(|mut f| serde_json::from_reader(&mut f).ok()).unwrap();

    let songs = api.call(Some(&token),
        audio::Search::new()
            .q("Poets Of The Fall")
            .performer_only(true)
            .count(200))
        .unwrap();

    for song in &songs.items {
        println!("{:?}", song);
    }
}