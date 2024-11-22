use rand::seq::SliceRandom;
use reqwest::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let endpoints = vec![
        (
            "POST",
            "http://localhost:3000/order",
            Some(r#"{"table_id": 1, "item_ids": [1]}"#),
        ),
        (
            "POST",
            "http://localhost:3000/order",
            Some(r#"{"table_id": 1, "item_ids": [2]}"#),
        ),
        ("DELETE", "http://localhost:3000/order/1/1", None),
        ("DELETE", "http://localhost:3000/order/1/2", None),
        ("GET", "http://localhost:3000/order/1", None),
        ("GET", "http://localhost:3000/order/1/1", None),
    ];

    // Loop to send requests indefinitely
    loop {
        // Randomly pick an endpoint
        if let Some((method, url, payload)) = endpoints.choose(&mut rand::thread_rng()) {
            println!("Hitting {} {}", method, url);

            let response = match *method {
                "POST" => {
                    client
                        .post(*url)
                        .header("Content-Type", "application/json")
                        .body(payload.unwrap_or_default().to_string())
                        .send()
                        .await
                }
                "DELETE" => {
                    client
                        .delete(*url)
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                }
                "GET" => {
                    client
                        .get(*url)
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                }
                _ => unreachable!(),
            };

            match response {
                Ok(res) => println!(
                    "Response: {} {}",
                    res.status(),
                    res.text().await.unwrap_or_default()
                ),
                Err(err) => eprintln!("Error: {}", err),
            }
        }

        // Sleep to maintain the desired RPS
        sleep(Duration::from_secs(1)).await;
    }
}
