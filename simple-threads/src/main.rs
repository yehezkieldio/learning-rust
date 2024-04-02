use std::thread;

fn start_no_shared_data_thread() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Hello from the thread!");
    })
}

fn start_shared_data_thread(a_number: i32, a_vec: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    thread::spawn(move || {
        print!("a_vec ---> [");
        for i in a_vec.iter() {
            print!(" {} ", i);
        }
        println!("]");
        println!("A number from inside the thread: {}", a_number);
        a_vec
    })
}

fn main() {
    let _ = start_no_shared_data_thread().join();
    println!("Main thread is done!");

    let _ = start_shared_data_thread(10, vec![1, 2, 3, 4, 5]).join();
    println!("Main thread is done!");
}
