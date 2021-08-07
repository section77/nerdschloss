use rppal::gpio::Gpio;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use warp;
use warp::Filter;

enum Direction {
    Open,
    Close,
}

fn open(channel: &mpsc::Sender<Direction>) -> String {
    channel.send(Direction::Open).unwrap();
    "\"success\"".to_string()
}

fn close(channel: &mpsc::Sender<Direction>) -> String {
    channel.send(Direction::Close).unwrap();
    "\"success\"".to_string()
}

fn run_stepper(channel: mpsc::Receiver<Direction>) {
    let mut direction = Gpio::new().unwrap().get(24).unwrap().into_output();
    let mut is_open = false;
    loop {
        match channel.recv() {
            Err(_) => return,
            Ok(Direction::Open) => {
                if !is_open {
                    direction.set_high();
                    do_steps();
                    is_open = true;
                }
            }
            Ok(Direction::Close) => {
                if is_open {
                    direction.set_low();
                    do_steps();
                    is_open = false;
                }
            }
        }
    }
}

fn do_steps() {
    let mut stepper = Gpio::new().unwrap().get(23).unwrap().into_output();
    println!("Start stepper");
    for _ in 1..279 {
        stepper.set_high();
        std::thread::sleep(std::time::Duration::from_millis(5));
        stepper.set_low();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("Stepper done");
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
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
