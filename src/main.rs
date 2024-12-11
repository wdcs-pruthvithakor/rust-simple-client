use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream};
use serde_json::Value;
use clap::{Command, Arg};
// use std::time::Duration;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // Parse the command-line arguments
    let matches = parse_arguments();

    // Extract the mode and times arguments
    let mode = matches.get_one::<String>("mode").unwrap();
    let times: u64 = matches
        .get_one::<String>("times")
        .unwrap()
        .parse()
        .unwrap_or(1);

    // Print the parsed arguments
    println!("Mode: {}", mode);
    println!("Will listen for {} seconds.", times);

    // Start the WebSocket listener in the "cache" mode
    if mode == "cache" {
        match run_websocket(times).await {
            Ok(_) => println!("Finished listening after {} seconds.", times),
            Err(e) => eprintln!("Error occurred while listening: {}", e),
        }
    } else {
        println!("Unknown mode: {}", mode);
    }
}

/// Parse the command-line arguments
fn parse_arguments() -> clap::ArgMatches {
    Command::new("WebSocket Listener")
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
                .default_value("1"),
        )
        .get_matches()
}

/// Connect to the WebSocket server
async fn connect_to_websocket() -> Result<tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn std::error::Error>> {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    
    let (ws_stream, _) = connect_async(url).await?;
    Ok(ws_stream)
}

/// Listen to the WebSocket and process the messages
async fn run_websocket(times: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the WebSocket server
    let mut ws_stream = connect_to_websocket().await?;

    println!("Connected to Binance WebSocket for BTC/USDT.");

    // Start listening to the WebSocket
    let start_time = tokio::time::Instant::now();
    let mut price_data: Vec<f64> = Vec::new();
    while start_time.elapsed().as_secs() < times {
        if let Some(Ok(Message::Text(text))) = ws_stream.next().await {
            // Process the incoming message
            if let Err(e) = process_message(&text) {
                eprintln!("Error processing message: {}", e);
            } 
            match process_message(&text) {
                Err(e) => println!("Error processing message: {}", e),
                Ok(price) => price_data.push(price),
            }
        } else {
            eprintln!("Error receiving message or unexpected message format");
            break;
        }
    }
    println!("Data Points: {:?}", price_data);
    match calculate_average(&price_data) {
        Some(avg) => println!("Cache complete. The average USD price of BTC is: {:.4}", avg),
        None => println!("No prices available to calculate the average"),
    };
    Ok(())
}

fn calculate_average(prices: &Vec<f64>) -> Option<f64> {
    if prices.is_empty() {
        return None; // Return None if the list is empty
    }
    let sum: f64 = prices.iter().sum(); // Sum up all the values in the vector
    let count = prices.len() as f64;   // Get the number of elements as f64
    Some(sum / count) // Return the average
}

/// Process a WebSocket message to extract and print the price
fn process_message(text: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Parse the WebSocket message as JSON
    let json: Value = serde_json::from_str(text)?;
    // println!("{:#?}", json);
    // Extract and print the price from the message
    if let Some(price) = json.get("p") {
        // println!("Price: {:?} USD", price);
        if let Ok(price_value) = price.as_str().unwrap().parse::<f64>() {
            // println!("Price: {} USD", price_value);
            Ok(price_value)
        } else {
            Err("Price field is not a valid f64".into())
        }
    } else {
        Err("No price found in message".into())
    }
}
