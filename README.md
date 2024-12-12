# WebSocket Listener for BTC/USDT Prices

This Rust project connects to the Binance WebSocket to listen for real-time BTC/USDT trade prices. The application can operate in two modes: 

1. **Cache mode**: Listens to the WebSocket for a specified duration, calculates the average price of BTC, and saves the data points along with the average price to files.
2. **Read mode**: Reads and displays previously saved price data from text files.

---

## Features

- Connects to Binance WebSocket for live BTC/USDT trade prices.
- Listens for price updates for a specified number of seconds in **cache mode**.
- Saves the received price data and the calculated average price to both simple text and JSON files.
- In **read mode**, it allows you to read and display previously saved price data from a text file.

---

## Requirements

- **Rust**: Make sure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
- **Dependencies**:
  - `futures`
  - `tokio-tungstenite`
  - `serde`
  - `serde_json`
  - `clap`
  - `tokio`

These dependencies are listed in the `Cargo.toml` file.

---

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/wdcs-pruthvithakor/rust-simple-client.git
   ```

2. Navigate into the project directory:

   ```bash
   cd rust-simple-client
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

---

## Usage

### 1. Running in **cache mode**

In **cache mode**, the program listens to the WebSocket for a specified number of seconds, collects price data, and saves the data points and the average price to files.

To run the program in **cache mode**:

```bash
cargo run -- --mode cache --times <seconds>
```

- `--mode cache`: Specifies that the program should run in cache mode.
- `--times <seconds>`: Specifies the number of seconds to listen for WebSocket messages (default is `1` second).

Example:

```bash
cargo run -- --mode cache --times 5
```

This will listen for 5 seconds and save the price data to files.

---

### 2. Running in **read mode**

In **read mode**, the program reads and prints the price data saved in the text file (`prices.txt`).

To run the program in **read mode**:

```bash
cargo run -- --mode read
```

This will display the saved price data from the file.

---

## File Outputs

- **prices.txt**: Contains the list of price data points and the calculated average price in a simple text format.
  
  Example content:
  ```
  Data Points: [34912.45, 34914.32, 34910.12]
  Average Price: 34912.30
  ```

- **prices.json**: Contains the price data points and average price in a structured JSON format.
  
  Example content:
  ```json
  {
    "data_points": [34912.45, 34914.32, 34910.12],
    "average_price": 34912.30
  }
  ```

---

## Code Overview

- **`parse_arguments`**: Handles command-line arguments parsing using the `clap` library.
- **`connect_to_websocket`**: Connects to the Binance WebSocket server for BTC/USDT price updates.
- **`run_websocket`**: Listens to WebSocket messages for a given duration and processes them.
- **`process_message`**: Extracts the BTC/USDT price from a WebSocket message.
- **`calculate_average`**: Calculates the average of the received BTC prices.
- **`save_to_files`**: Saves the received price data and the calculated average to text and JSON files.
- **`read_mode`**: Reads and displays the saved data from `prices.txt`.

---

## Error Handling

If any error occurs while processing the WebSocket messages, connecting to the WebSocket, or reading/saving files, appropriate error messages will be displayed in the console.

---

## Contribution

Feel free to fork the repository and submit pull requests. Issues and suggestions are welcome!

---

## License

This project is open-source and available under the [MIT License](LICENSE).

---

## Example

Run the program to listen to the WebSocket for 10 seconds and save the results:

```bash
cargo run -- --mode cache --times 10
```

After completion, you can open `prices.txt` and `prices.json` to view the saved price data and the calculated average.

