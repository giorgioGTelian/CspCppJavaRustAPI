// Cargo.toml dependencies
// [dependencies]
// actix-web = "4"
// serde = { version = "1.0", features = ["derive"] }
// anyhow = "1.0"
// thiserror = "1.0"
// log = "0.4"
// env_logger = "0.9"

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use thiserror::Error;
use log::{info, error};
use env_logger;

#[derive(Serialize)]
struct MyResponse {
    message: String,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("Internal Server Error")]
    InternalError,
}

impl actix_web::ResponseError for MyError {}

async fn greet() -> Result<impl Responder, MyError> {
    info!("Handling greet request");
    let response = MyResponse {
        message: String::from("Hello, Gio!"),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/greet", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// setting up some more server configurations
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_greet() {
        let app = test::init_service(App::new().route("/greet", web::get().to(greet))).await;
        let req = test::TestRequest::get().uri("/greet").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

let app_state = database::get_connection_pool(&config);
    let app = app::create_app(app_state);

    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port) {
        .parse()
        .expect("Unable to parse socket address");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(...)
        .await
        .expect("Failed to start server");
    }

    fn init_tracer(config: &Configurations) -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(config.tracing.host.clone()),
            )
            .with_trace_config(
                sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    config.service.name.clone(),
                )])),
            )
            .install_batch(runtime::Tokio)
    }
    
    #[tokio::main]
    async fn main() {
    ...
    // initialize tracing
        let tracer = init_tracer(&config).expect("Failed to initialize tracer.");
        let fmt_layer = tracing_subscriber::fmt::layer();
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from(&config.logger.level))
            .with(fmt_layer)
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();
    ...
    }
