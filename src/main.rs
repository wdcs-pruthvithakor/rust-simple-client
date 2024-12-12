use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream};
use serde_json::Value;
use clap::{Command, Arg};
use tokio::net::TcpStream;
use std::fs::File;
use std::io::{self, BufRead, Write, BufReader};

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


    // Start the WebSocket listener in the "cache" mode
    match mode.as_str() {
        "cache" => {
            println!("Will listen for {} seconds.", times);
            match run_websocket(times).await {
                Ok(_) => println!("Finished listening after {} seconds.", times),
                Err(e) => eprintln!("Error occurred while listening: {}", e),
            }
        },
        "read" => read_mode().expect("Failed to read price data"),
        _ => eprintln!("Invalid mode: {mode}. Use --mode=cache or --mode=read.")
    }

}

/// Parse the command-line arguments
fn parse_arguments() -> clap::ArgMatches {
    Command::new("WebSocket Listener")
        .version("1.0")
        .author("Pruthvi Thakor")
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
            match process_message(&text) {
                Err(e) => eprintln!("Error processing message: {}", e),
                Ok(price) => price_data.push(price),
            }
        } else {
            eprintln!("Error receiving message or unexpected message format");
            break;
        }
    }

    // println!("Data Points: {:?}", price_data);
    match calculate_average(&price_data) {
        Some(avg) => {
            println!("Cache complete. The average USD price of BTC is: {:.4}", avg);
            save_to_files(&price_data, avg)?;
        }
        None => println!("No prices available to calculate the average"),
    };
    Ok(())
}

fn calculate_average(prices: &Vec<f64>) -> Option<f64> {
    if prices.is_empty() {
        return None;
    }
    let sum: f64 = prices.iter().sum();
    let count = prices.len() as f64;
    Some(sum / count)
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

/// Save the data points and average to simple and JSON files
fn save_to_files(price_data: &Vec<f64>, average: f64) -> io::Result<()> {
    // Save to a simple text file
    let mut simple_file = File::create("prices.txt")?;
    writeln!(simple_file, "Data Points: {:?}", price_data)?;
    writeln!(simple_file, "Average Price: {:.4}", average)?;

    // Save to a JSON file
    let mut json_file = File::create("prices.json")?;
    let json_data = serde_json::json!({
        "data_points": price_data,
        "average_price": average
    });
    write!(json_file, "{}", serde_json::to_string_pretty(&json_data)?)?;

    Ok(())
}

fn read_mode() -> io::Result<()> {
    println!("Reading prices data ...\n\n");
    let file = File::open("prices.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
