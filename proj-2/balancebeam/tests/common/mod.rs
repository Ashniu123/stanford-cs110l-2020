mod balancebeam;
mod echo_server;
mod error_server;
mod server;

use std::sync;
use tokio::time::{advance, pause, resume, Duration};

pub use balancebeam::BalanceBeam;
pub use echo_server::EchoServer;
pub use error_server::ErrorServer;
pub use server::Server;

static INIT_TESTS: sync::Once = sync::Once::new();

pub fn init_logging() {
    INIT_TESTS.call_once(|| {
        pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("info")
            .init();
    });
}

pub async fn skip_time(duration: Duration) {
    pause();
    advance(duration).await;
    resume();
}
