#[allow(unused)]
use axum_server::tls_rustls::RustlsConfig;
#[allow(unused)]
use ffxiv_simhelper_api::api_server::api_router::create_ffxiv_simhelper_service_router;
#[allow(unused)]
use ffxiv_simhelper_api::config::{AppState, FfxivSimhelperConfig};
#[allow(unused)]
use std::{net::SocketAddr, path::PathBuf};
#[allow(unused)]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(not(target_os = "windows"))]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from("/etc/letsencrypt/live/www.ffxivsimhelper.com/fullchain.pem"),
        PathBuf::from("/etc/letsencrypt/live/www.ffxivsimhelper.com/privkey.pem"),
    )
    .await
    .unwrap();

    let config_directory = PathBuf::from("./config/backend_server_config.yml");

    let backend_config = FfxivSimhelperConfig::try_new(config_directory)
        .expect("Failed to load backend config file");
    let app_state = AppState::from(backend_config);

    let app = create_ffxiv_simhelper_service_router(app_state);

    // run https server
    let addr = SocketAddr::from(([0, 0, 0, 0], 13406));
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
