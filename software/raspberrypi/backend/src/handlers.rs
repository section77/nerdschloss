use poem::{handler, web::Data, IntoResponse};
use tokio::sync::mpsc::Sender;
use tracing::debug;

use hardware::Direction;

#[handler]
pub async fn state() -> impl IntoResponse {
    #[cfg(debug_assertions)]
    debug!("state");
}

#[handler]
pub async fn open(sender: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    debug!("open");
    sender.0.send(Direction::Open).await.unwrap();
}

#[handler]
pub async fn close(sender: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    debug!("close");
    sender.0.send(Direction::Close).await.unwrap();
}
