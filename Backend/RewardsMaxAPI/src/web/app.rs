use axum_login:: {
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use axum::Extension;
use axum_messages::MessagesManagerLayer;
use sqlx::SqlitePool; // Retained for session storage
use time::Duration;
use tokio:: { signal, task::AbortHandle};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::SqliteStore;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool}; // Import Diesel's connection pool
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use http::header::{CONTENT_TYPE};
use tower_http::cors::Any;
use http::Method;

use crate:: {
    users::Backend,
    web::{auth, protected, user, index, companies, vendor_deals, crowdsourcing, card},
};

// Define a type alias for the Diesel connection pool
type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct App {
    db: SqlitePool,
    diesel_pool: DbPool, // Add Diesel pool to the struct
}

impl App {

    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // SQLx pool for session storage
        let db = SqlitePool::connect("database/rewards_maximizer.db").await?;

        // Diesel connection pool for app queries
        let database_url = "database/rewards_maximizer.db";
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let diesel_pool = Pool::builder()
            .build(manager)
            .expect("Failed to create Diesel connection pool");

        Ok(Self { db, diesel_pool })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        // Session layer for SQLx sessions
        let session_store = SqliteStore::new(self.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // Generate a cryptographic key to sign the session cookie
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        // Auth service setup
        let backend = Backend::new(self.db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let cors = CorsLayer::new()
            //.allow_credentials(true) -> will have to change .allow_origin in order to use
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)
            .allow_headers([CONTENT_TYPE]);

        // Main router setup
        let serve_dir = ServeDir::new("static"); // Serve static files
        let app = protected::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .merge(user::router())
            .merge(index::router())
            .merge(companies::router())
            .merge(vendor_deals::router())
            .merge(crowdsourcing::router())
            .merge(card::router())
            .layer(Extension(self.diesel_pool.clone())) // Provide Diesel pool to handlers
            .layer(MessagesManagerLayer)
            .layer(auth_layer)
            .layer(cors)
            .nest_service("/static", serve_dir.clone());

        // Listen on 0.0.0.0:8080 (localhost:8080 for local testing)
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

        // Run the Axum server with a graceful shutdown signal
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        deletion_task.await??;

        Ok(())
    }
}

// Signal handler for graceful shutdown
async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
