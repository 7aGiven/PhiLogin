use tokio::runtime;
use reqwest::Client;
use axum::routing;
use axum::body::Bytes;
use axum::extract::{Path, State};
use hmac::Mac;
use base64::Engine;
use rand::{RngCore, SeedableRng};

#[derive(Clone)]
struct Share {
    client: Client,
    tap: reqwest::header::HeaderMap,
    phi: reqwest::header::HeaderMap
}

#[derive(serde::Deserialize)]
struct Wrap<T> {
    success: bool,
    data: T
}

#[derive(serde::Deserialize)]
struct Token {
    kid: String,
    mac_key: String
}

#[derive(serde::Deserialize)]
struct Account {
    openid: String,
    unionid: String
}

fn mac(token: &Token) -> String {
    let ts: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let nonce: u32 = rand::rngs::SmallRng::seed_from_u64(ts).next_u32();
    let input: String = format!("{}\n{}\nGET\n/account/basic-info/v1?client_id=rAK3FfdieFob2Nn8Am\nopen.tapapis.cn\n443\n\n", ts, nonce);
    let mut mac = hmac::Hmac::<sha1::Sha1>::new_from_slice(token.mac_key.as_bytes()).unwrap();
    mac.update(input.as_bytes());
    let mac: String = base64::prelude::BASE64_STANDARD.encode(mac.finalize().into_bytes());
    format!("MAC id=\"{}\",ts=\"{}\",nonce=\"{}\",mac=\"{}\"", token.kid, ts, nonce, mac)
}

async fn login(State(state): State<Share>, device_id: Bytes) -> String {
    let json: Wrap<serde_json::Value> = state.client.post("https://www.taptap.cn/oauth2/v1/device/code")
        .headers(state.tap.clone())
        .body(format!("client_id=rAK3FfdieFob2Nn8Am&response_type=device_code&scope=basic_info&version=1.2.0&platform=unity&info=%7b%22device_id%22%3a%22{}%22%7d", percent_encoding::percent_encode(device_id.as_ref(), percent_encoding::NON_ALPHANUMERIC)))
        .send().await.unwrap().json().await.unwrap();
    serde_json::to_string(&json.data).unwrap()
}

async fn token(State(state): State<Share>, Path(device_code): Path<String>, device_id: Bytes) -> String {
    let json = state.client.post("https://www.taptap.com/oauth2/v1/token")
        .headers(state.tap.clone())
        .body(format!("grant_type=device_token&client_id=rAK3FfdieFob2Nn8Am&secret_type=hmac-sha-1&code={}&version=1.0&platform=unity&info=%7b%22device_id%22%3a%22{}%22%7d", device_code, percent_encoding::percent_encode(device_id.as_ref(), percent_encoding::NON_ALPHANUMERIC)))
        .send().await.unwrap().json::<Wrap<serde_json::Value>>().await.unwrap();
    if json.success == false {
        return serde_json::to_string(&json.data).unwrap();
    }
    let token: Token = serde_json::from_value(json.data).unwrap();
    let account: Account = state.client.get("https://open.tapapis.cn/account/basic-info/v1?client_id=rAK3FfdieFob2Nn8Am")
        .headers(state.tap.clone())
        .header("Authorization", mac(&token))
        .send().await.unwrap().json::<Wrap<Account>>().await.unwrap().data;
    state.client.post("https://rak3ffdi.cloud.tds1.tapapis.cn/1.1/users")
        .headers(state.phi.clone())
        .body(format!("{{\"authData\":{{\"taptap\":{{\"kid\":\"{}\",\"access_token\":\"{}\",\"token_type\":\"mac\",\"mac_key\":\"{}\",\"mac_algorithm\":\"hmac-sha-1\",\"openid\":\"{}\",\"unionid\":\"{}\"}}}}}}", token.kid, token.kid, token.mac_key, account.openid, account.unionid))
        .send().await.unwrap().text().await.unwrap()
}

async fn start_server() {
    let mut share = Share {
        client: reqwest::ClientBuilder::new().http1_title_case_headers()
            .user_agent("TapTapUnitySDK/1.0 UnityPlayer/2021.3.40f1c1")
            .build().unwrap(),
        tap: reqwest::header::HeaderMap::new(),
        phi: reqwest::header::HeaderMap::new()
    };
    share.tap.append("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    share.tap.append("User-Agent", "TapTapAndroidSDK/3.16.5".parse().unwrap());
    share.phi.append("User-Agent", "LeanCloud-CSharp-SDK/1.0.3".parse().unwrap());
    share.phi.append("X-LC-Id", "rAK3FfdieFob2Nn8Am".parse().unwrap());
    share.phi.append("X-LC-Key", "Qr9AEqtuoSVS3zeD6iVbM4ZC0AtkJcQ89tywVyi0".parse().unwrap());
    share.phi.append("Content-Type", "application/json".parse().unwrap());
    let app = axum::Router::new()
        .route("/login", routing::post(login))
        .route("/token/:device_code", routing::post(token))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(share);
    let listener = tokio::net::TcpListener::bind(std::env::args().next_back().unwrap()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    let rt: runtime::Runtime = runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build().unwrap();
    rt.block_on(start_server());
}
