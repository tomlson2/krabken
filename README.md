


This code is an implementation of the Kraken API. It provides a library of functions to access the Kraken exchange and fetch data such as current order book, OHLC (Open, High, Low, Close) data and trades.

Requirements:
- reqwest
- hmac
- sha2
- serde
- base64
- serde_json

Usage:
1. Create a KrakenClient object with an api key and secret key, which can be obtained through the Kraken account.

let config = Config {
    key: String::from("your api key"),
    secret: String::from("your secret key")
};
let mut kraken_client = KrakenClient::new(config);

2. Create a TradingData object with the KrakenClient object. 

let mut trading_data = TradingData::new(kraken_client);

3. Use the TradingData object to access the Kraken data.

// Get OHLC data
trading_data.get_ohlc_data("XXBTZUSD", "1m", None, true).unwrap();

// Get order book data
trading_data.get_order_book_data("XXBTZUSD", 100).unwrap();

// Get trade data 
trading_data.get_trade_data("XXBTZUSD").unwrap();