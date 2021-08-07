use warp;
use warp::Filter;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

fn open(is_open: Arc<AtomicBool>) -> String {
    if is_open
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        println!("opening");
        "\"success\""
    } else {
        "\"was already\""
    }
    .to_string()
}

fn close(is_open: Arc<AtomicBool>) -> String {
    if is_open
        .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        println!("closing");
        "\"success\""
    } else {
        "\"was already\""
    }
    .to_string()
}

#[tokio::main]
async fn main() {
    let is_open1 = Arc::new(AtomicBool::new(false));
    let is_open2 = is_open1.clone();
    let routes = warp::path::end()
        .and(warp::fs::file("./static/index.html"))
        .or(warp::path("open").map(move || open(is_open1.clone())))
        .or(warp::path("close").map(move || close(is_open2.clone())));
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
