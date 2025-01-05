use tokio::{sync::mpsc::unbounded_channel, task};

fn heavy_batch_task(numbers: Vec<u8>) {
    println!("Handling {:?} numbers", numbers);
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = unbounded_channel::<u8>();
    task::spawn(async move {
        let limit = 100;
        loop {
            let mut buffer = Vec::new();
            receiver.recv_many(&mut buffer, limit).await;
            println!("Received a batch with {} items", buffer.len());
            task::spawn_blocking(move || {
                heavy_batch_task(buffer);
            })
            .await
            .unwrap();
        }
    });

    for i in 0..100 {
        sender.send(i).unwrap();
    }
}
