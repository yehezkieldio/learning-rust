use actix::prelude::*;

#[derive(Message)]
#[rtype(f64)]
struct SensorData(f64);

struct SensorActor;

impl Actor for SensorActor {
    type Context = Context<Self>;
}

impl Handler<SensorData> for SensorActor {
    type Result = f64;

    fn handle(&mut self, msg: SensorData, _: &mut Self::Context) -> f64 {
        println!("Sensor data: {}", msg.0);
        msg.0
    }
}

fn create_sensor_actor() -> Addr<SensorActor> {
    SensorActor.start()
}

#[actix::main]
async fn main() {
    let sensor_actor = create_sensor_actor();

    for i in 1..=10 {
        sensor_actor.do_send(SensorData(i as f64));
        println!("Sent sensor data: {}", i);
    }

    tokio::time::sleep_until(tokio::time::Instant::now() + tokio::time::Duration::from_secs(1))
        .await;
}
