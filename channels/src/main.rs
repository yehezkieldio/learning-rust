// Channels are multi-producer-single-consumer data structures, consisting of many senders but only single receiver.
// Under the hood, the channel does not lock but relies on an unsafe data structure that allows the detection and management of
// the state of the stream. The channel handles simply sending data across threads well and 
// can be used to create an actor-style framework or a reactive map-reduce style data-processing engine.

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use rand::random;

enum ChartValue {
    Star(usize),
    Pipe(usize),
}

fn main() {
    // tx is the sending half (tx for transmission), and rx is the receiving half (rx for receiving).
    let (tx, rx): (Sender<ChartValue>, Receiver<ChartValue>) = mpsc::channel();

    let pipe_sender = tx.clone();

    thread::spawn(move || loop {
        pipe_sender
            .send(ChartValue::Pipe(random::<usize>() % 80))
            .unwrap();
        thread::sleep(Duration::from_millis(random::<u64>() % 800));
    });

    let star_sender = tx.clone();

    thread::spawn(move || loop {
        star_sender
            .send(ChartValue::Star(random::<usize>() % 80))
            .unwrap();
        thread::sleep(Duration::from_millis(random::<u64>() % 800));
    });

    while let Ok(val) = rx.recv_timeout(Duration::from_secs(3)) {
        println!(
            "{}",
            match val {
                ChartValue::Pipe(v) => "|".repeat(v + 1),
                ChartValue::Star(v) => "*".repeat(v + 1),
            }
        )
    }
}
