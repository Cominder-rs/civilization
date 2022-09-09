use crate::common_structs::Env;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_service() -> Env {
    dotenv::dotenv().ok();

    let env = match std::env::var("ENV").expect("Specify ENV variable").as_str() {
        "dev" => Env::Dev,
        "prod" => Env::Prod,
        "test" => Env::Test,
        _ => panic!("ENV var can be either dev, prod, test"),
    };

    if env == Env::Dev {
        println!("Warning! DEV env is set!")
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,hyper=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    env
}
