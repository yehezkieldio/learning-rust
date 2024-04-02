// Arc stands for Atomic Reference Counter. This makes them very similar
// to regular reference counters (Rc), with the exception that an Arc does its
// job with an atomic increment, which is thread-safe. Therefore, they're the
// only choice for cross-threaded reference counting.

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
enum Shade {
    Black,
    White,
}

fn new_painter_thread(data: Arc<Mutex<Vec<Shade>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let mut d = data.lock().unwrap();
        if d.len() > 0 {
            match d[d.len() - 1] {
                Shade::Black => d.push(Shade::White),
                Shade::White => d.push(Shade::Black),
            }
        } else {
            d.push(Shade::Black)
        }
        if d.len() > 5 {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    })
}

// Rust ownership is a double-edged sword: on the other hand, it protects from unintended consequences and enables compile-time memory management; on the other
// hand, mutable access is significantly more difficult to obtain. While it is more complex to
// manage, shared mutable access can be great for performance.

fn main() {
    let data = Arc::new(Mutex::new(vec![]));
    let threads: Vec<thread::JoinHandle<()>> =
        (0..2).map(|_| new_painter_thread(data.clone())).collect();

    let _: Vec<()> = threads.into_iter().map(|t| t.join().unwrap()).collect();

    println!("Result: {:?}", data);
}
