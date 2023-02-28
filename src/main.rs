use arti_client::{TorClient, TorClientConfig};
use arti_hyper::*;
use hyper::{Body, Client, Method, Request, Uri};
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use tls_api::{TlsConnector as TlsConnectorTrait, TlsConnectorBuilder};
use tls_api_native_tls::TlsConnector;
use tor_rtcompat::PreferredRuntime;
use tracing::warn;

const REQSIZE: u64 = 1024;
const TORURL: &str =
    "https://dist.torproject.org/torbrowser/12.0.3/tor-browser-linux64-12.0.3_ALL.tar.xz";
const TESTURL: &str = "https://gotlou.srht.site/pubkey.pgp";

// TODO: Handle all unwrap() effectively

// Create new HTTPS connection with a new circuit
async fn get_new_connection() -> Client<ArtiHttpConnector<PreferredRuntime, TlsConnector>> {
    let config = TorClientConfig::default();
    let tor_client = TorClient::create_bootstrapped(config).await.unwrap();
    let tls_connector = TlsConnector::builder().unwrap().build().unwrap();

    let connection = ArtiHttpConnector::new(tor_client, tls_connector);
    let http = hyper::Client::builder().build::<_, Body>(connection);
    http
}

// Get the size of file to be downloaded
async fn get_content_length(url: &'static str) -> u64 {
    let http = get_new_connection().await;
    let uri = Uri::from_static(url);
    warn!("Requesting content length of {} via Tor...", url);
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let resp = http.request(req).await.unwrap();
    let raw_length = resp.headers().get("Content-Length").unwrap();
    let length = raw_length.to_str().unwrap().parse::<u64>().unwrap();
    warn!("Content-Length of resource: {}", length);
    length
}

// Just get the file from the server and store it in a Vec
async fn request(url: &'static str, start: usize, end: usize) -> Vec<u8> {
    let http = get_new_connection().await;
    let uri = Uri::from_static(url);
    let partial_req_value =
        String::from("bytes=") + &start.to_string() + &String::from("-") + &end.to_string();
    warn!("Requesting {} via Tor...", url);
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Range", partial_req_value)
        .body(Body::default())
        .unwrap();
    let mut resp = http.request(req).await.unwrap();

    if resp.status() == 206 {
        warn!("Good request, getting partial content...");
    } else {
        warn!("Non 206 Status code: {}", resp.status());
    }

    let body = hyper::body::to_bytes(resp.body_mut())
        .await
        .unwrap()
        .to_vec();
    body
}

fn save_to_file(fd: &mut File, start: usize, body: Vec<u8>) {
    fd.seek(std::io::SeekFrom::Start(start as u64)).unwrap();
    fd.write_all(&body).unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    warn!("Creating download file");
    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open("download")
        .unwrap();
    let url = TORURL;
    //let url = TESTURL;
    let length = get_content_length(url).await;
    fd.set_len(length).unwrap();
    let steps = length / REQSIZE;
    let mut start = 0;
    for _ in 0..steps {
        let end = start + (REQSIZE as usize) - 1;
        let body = request(url, start, end).await;
        save_to_file(&mut fd, start, body);
        start = end + 1;
    }
    if start < length as usize {
        let body = request(url, start, length as usize).await;
        save_to_file(&mut fd, start, body);
    }
}
