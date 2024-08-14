use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

pub fn get_client_credentials() -> (String, String) {
    let client_id = include_str!("../.env")
        .split('\n')
        .next()
        .expect("env file has more than one line")
        .split("DCADMIN_CLIENT_ID=")
        .nth(1)
        .expect("CLIENT_ID MUST be provided");
    let client_secret = include_str!("../.env")
        .split('\n')
        .nth(1)
        .expect("env file has more than one line")
        .split("DCADMIN_CLIENT_SECRET=")
        .nth(1)
        .expect("CLIENT_SECRET MUST be provided");

    (client_id.to_string(), client_secret.to_string())
}

pub fn setup_logging() {
    LogTracer::init().expect("Failed to set logger");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
