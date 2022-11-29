use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use motordriver::{run_stepper, Direction};
use warp::Filter;

fn open(channel: &mpsc::Sender<Direction>) -> String {
    channel.send(Direction::Open).unwrap();
    "\"success\"".to_string()
}

fn close(channel: &mpsc::Sender<Direction>) -> String {
    channel.send(Direction::Close).unwrap();
    "\"success\"".to_string()
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel();
    let ch1 = Arc::new(Mutex::new(sender.clone()));
    let ch2 = Arc::new(Mutex::new(sender));
    std::thread::spawn(|| run_stepper(receiver));
    let routes = warp::path::end()
        .and(warp::fs::file("./static/index.html"))
        .or(warp::path("open").map(move || open(&ch1.lock().unwrap())))
        .or(warp::path("close").map(move || close(&ch2.lock().unwrap())));
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
