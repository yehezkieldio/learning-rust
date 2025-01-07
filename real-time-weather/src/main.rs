use std::time::Duration;

use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use serde::{Deserialize, Serialize};
use tokio::time::interval;

#[derive(Deserialize, Serialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_units: CurrentUnits,
    current: Current,
}

#[derive(Deserialize, Serialize, Debug)]
struct WeatherData {
    temperature_2m: f64,
    wind_speed_10m: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct CurrentUnits {
    time: String,
    interval: String,
    temperature_2m: String,
    wind_speed_10m: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Current {
    time: String,
    interval: u32,
    temperature_2m: f64,
    wind_speed_10m: f64,
}

async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m",
        lat, lon
    );
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<WeatherResponse>().await?;
    println!("{:?}", weather_data);

    Ok(weather_data)
}

async fn produce_weather_data(producer: &FutureProducer, lat: f64, lon: f64) {
    let topic = "weather_data";
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        match fetch_weather(lat, lon).await {
            Ok(weather_data) => {
                let data = WeatherData {
                    temperature_2m: weather_data.current.temperature_2m,
                    wind_speed_10m: weather_data.current.wind_speed_10m,
                };

                match serde_json::to_string(&data) {
                    Ok(payload) => {
                        let delivery_status = producer
                            .send(
                                FutureRecord::to(topic)
                                    .payload(&payload)
                                    .key("weather_data_key"),
                                Duration::from_secs(0),
                            )
                            .await;

                        match delivery_status {
                            Ok(delivery) => println!("Sent: {:?}", delivery),
                            Err((e, _)) => eprintln!("Error: {:?}", e),
                        }
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let lat: f64 = -1.269160;
    let lon: f64 = 16.825264;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    match producer
        .send(
            FutureRecord::to("weather_data")
                .payload("test")
                .key("weather_data_key"),
            Duration::from_secs(0),
        )
        .await
    {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => eprintln!("Error: {:?}", e),
    }

    produce_weather_data(&producer, lat, lon).await;
}
