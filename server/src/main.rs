use fretwise_server::{db::Db, router, AppState};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tracing_subscriber::EnvFilter;

/// Settings come from environment variables so `cargo run` works with no arguments.
///
/// - `FRETWISE_ADDR`  address to listen on, default `127.0.0.1:8080` (this computer only)
/// - `FRETWISE_DB`    SQLite file, default `fretwise.db` in the current folder
/// - `FRETWISE_INDEX` the app page, default `../index.html` (the repo's index.html when run from `server/`)
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "fretwise_server=info".into()))
        .init();

    let addr: SocketAddr = std::env::var("FRETWISE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()
        .expect("FRETWISE_ADDR must look like 127.0.0.1:8080");
    let db_path = PathBuf::from(std::env::var("FRETWISE_DB").unwrap_or_else(|_| "fretwise.db".into()));
    let index_path = PathBuf::from(std::env::var("FRETWISE_INDEX").unwrap_or_else(|_| "../index.html".into()));

    if !index_path.exists() {
        tracing::warn!("{} not found: set FRETWISE_INDEX to the app's index.html", index_path.display());
    }
    let db = Db::open(&db_path).unwrap_or_else(|e| panic!("cannot open {}: {e}", db_path.display()));

    let state = Arc::new(AppState {
        db: Mutex::new(db),
        index_path,
        storage: format!("sqlite:{}", db_path.display()),
    });
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("cannot listen on {addr}: {e}"));
    tracing::info!("Fretwise running at http://{addr}  (songs in {})", db_path.display());
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .unwrap();
}
