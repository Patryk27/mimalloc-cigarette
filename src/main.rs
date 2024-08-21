// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

use mimalloc::MiMalloc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct State {
    hotels: Vec<Hotel>,
}

impl State {
    fn new() -> Self {
        Self {
            hotels: load_hotels().collect(),
        }
    }

    fn refresh(&mut self) {
        println!("refreshing");

        for (id, hotel) in load_hotels().enumerate() {
            self.hotels[id] = hotel;
        }
    }
}

#[allow(dead_code)]
struct Hotel {
    prices: Vec<f32>,
}

fn load_hotels() -> impl Iterator<Item = Hotel> {
    (0..1_000).map(|_| Hotel {
        prices: (0..1_000_000).map(|n| n as f32).collect(),
    })
}

fn main() {
    let state = Arc::new(RwLock::new(State::new()));

    println!("ready");

    thread::spawn({
        let state = Arc::clone(&state);

        move || loop {
            thread::sleep(Duration::from_secs(5));
            state.write().unwrap().refresh();
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
