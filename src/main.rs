use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::Value;
use clap::{Command, Arg};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Define the command-line arguments
    let matches = Command::new("WebSocket Listener")
        .version("1.0")
        .author("Pruthvi Thakor, pruthvi.thakor@codezeros")
        .about("Listens to the WebSocket for BTC/USDT prices")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Specifies the mode of operation")
                .default_value("cache"),
        )
        .arg(
            Arg::new("times")
                .short('t')
                .long("times")
                .value_name("NUMBER")
                .help("The number of seconds to listen")
                .default_value("10"),
        )
        .get_matches();

    // Parse the arguments
    let mode = matches.get_one::<String>("mode").unwrap();

    // Get the value of the "times" argument and convert it to u64
    let times: u64 = matches
        .get_one::<String>("times")
        .unwrap()
        .parse()
        .unwrap_or(10);

    // Print out the parsed values for verification
    println!("Mode: {}", mode);
    println!("Will listen for {} seconds.", times);

    if mode == "cache" {
        // Run the WebSocket listener in "cache" mode (as per the original example)
        run_websocket(times).await;
    } else {
        println!("Unknown mode: {}", mode);
    }
}

async fn run_websocket(times: u64) {
    // Binance WebSocket endpoint for BTC/USDT ticker
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";

    // Connect to the WebSocket server
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    println!("Connected to Binance WebSocket for BTC/USDT.");

    // Start listening to the WebSocket for the given duration
    let start_time = tokio::time::Instant::now();

    loop {
        let msg = ws_stream.next().await;

        match msg {
            Some(Ok(Message::Text(text))) => {
                // Parse the WebSocket message as JSON
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    // Extract and print the price from the message
                    if let Some(price) = json.get("p") {
                        println!("Price: {} USD", price);
                    }
                }
            },
            Some(Err(e)) => {
                eprintln!("Error on WebSocket stream: {}", e);
                break;
            },
            _ => {}
        }

        // Sleep for a short duration (to avoid spamming too fast)
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Check if the time has passed
        if start_time.elapsed().as_secs() >= times {
            break;
        }


    }

    println!("Finished listening after {} seconds.", times);
}

