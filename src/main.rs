use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let version:String  = String::from("v0.01");
    info!("Starting Semantic IMS {}", version);
}
