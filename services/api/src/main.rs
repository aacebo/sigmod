use actix_web::{App, HttpServer, web};
use amqp::{Action, Key};
use sqlx::postgres::PgPoolOptions;

mod config;
mod context;
mod request_context;
mod routes;

pub use config::Config;
pub use context::Context;
pub use request_context::{RequestContext, RequestContextMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("../../crates/storage/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let amqp = amqp::new(&config.rabbitmq_url)
        .with_app_id("sigmod[api]")
        .with_queue(Key::new("workspace", Action::Create))
        .with_queue(Key::new("workspace", Action::Update))
        .connect()
        .await
        .expect("error while connecting to rabbitmq");

    let classifier = ai::local::classify::LocalClassifierBuilder::new()
        .build_async()
        .await
        .expect("Failed to build local classifier");

    let classifier_client = ai::client::Client::new().with_classifier(classifier);
    let chat_client = ai::client::Client::new().with_chat(ai::openai::OpenAIClient::new());
    let runner = eval::Runner::new()
        .with("local/bart-mnli".parse().unwrap(), classifier_client)
        .with("openai/gpt-4o-mini".parse().unwrap(), chat_client);

    let ctx = Context::new(pool, amqp, runner);
    println!("Starting server at http://0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .wrap(RequestContextMiddleware)
            .service(routes::index::get)
            .service(routes::evals::create)
            .configure(routes::console::configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
