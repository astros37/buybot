# Buy Bot

This is a simple Rust bot that buys tokens in extreme volume from a cryptocurrency exchange.

## Requirements

- Rust
- An API key and secret from the cryptocurrency exchange

## Setup

1. Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

2. Clone the repository:
    ```sh
    git clone https://github.com/astros37/buy_bot.git
    cd buy_bot
    ```

3. Create a `.env` file in the root directory and add your API key and secret:
    ```sh
    echo "API_KEY=your_api_key" >> .env
    echo "API_SECRET=your_api_secret" >> .env
    ```

4. Update the `Cargo.toml` file to include necessary dependencies. This has already been done in the provided `Cargo.toml`.

5. Replace the API endpoint `"https://api.exchange.com/v1/order"` in `src/main.rs` with the actual endpoint of the exchange you are targeting.

6. Adjust the `Order` struct in `src/main.rs` to match the expected format of the exchange's API.

## Running the Bot

Run the bot using the following command:
```sh
cargo run
```

## Example Output

The output will display the response from the exchange's API after attempting to place an order.

```
Order response: {"order_id":"12345","status":"FILLED"}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.