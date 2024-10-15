use axum::{routing::post, Router};
use ffxiv_simhelper_api::api_server::api_router::create_ffxiv_simhelper_service_router;
use hyper::server::conn::Http;
use hyper::Method;
use hyper::StatusCode;
use log::LevelFilter::Info;
use log::{info, Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::net::SocketAddr;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio_rustls::rustls;
use tokio_rustls::TlsAcceptor;
use tower_http::cors::{Any, CorsLayer};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}

const PORT_NUMBER: i32 = 13406;

#[tokio::main]
async fn main() {
    info!("Loading Logger");
    init(Info).expect("failed to load logger");

    let app = create_ffxiv_simhelper_service_router();

    let certs = load_certs("/etc/letsencrypt/live/www.ffxivsimhelper.com/fullchain.pem");
    let key = load_private_key("/etc/letsencrypt/live/www.ffxivsimhelper.com/privkey.pem");

    let tls_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 13406));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    info!("Started Server at port {}", PORT_NUMBER);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT_NUMBER))
        .await
        .unwrap();

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let acceptor = acceptor.clone();
        let app = app.clone();

        tokio::spawn(async move {
            if let Ok(stream) = acceptor.accept(stream).await {
                Http::new()
                    .serve_connection(stream, app.into_make_service())
                    .await
                    .expect("failed to serve connection");
            }
        });
    }
}

fn load_certs(path: &str) -> Vec<rustls::Certificate> {
    let certfile = File::open(path).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);

    rustls_pemfile::certs(&mut reader)
        .expect("failed to read certificate")
        .into_iter()
        .map(rustls::Certificate)
        .collect()
}

fn load_private_key(path: &str) -> rustls::PrivateKey {
    let keyfile = File::open(path).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader).expect("failed to read private key");

    assert!(!keys.is_empty(), "failed to load private key");

    rustls::PrivateKey(keys[0].clone())
}
