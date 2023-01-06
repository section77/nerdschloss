use motordriver::Direction;

use poem::{handler, web::Data, IntoResponse};
use tokio::sync::mpsc::Sender;

#[handler]
pub async fn state() -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("state");
}

#[handler]
pub async fn open(sender: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("open");
    sender.0.send(Direction::Open).await.unwrap();
}

#[handler]
pub async fn close(sender: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("close");
    sender.0.send(Direction::Close).await.unwrap();
}
