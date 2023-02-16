use reqwest::{
    header::{HeaderMap, HeaderValue},
};
use hmac::{Hmac, Mac, NewMac};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
    secret: String,
}

struct KrakenClient {
    http: reqwest::blocking::Client,
    config: Config,
    nonce: u64,
}

impl KrakenClient {
    fn new(config: Config) -> KrakenClient {
        KrakenClient {
            http: reqwest::blocking::Client::new(),
            config,
            nonce: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
    fn get_signature(
        &self,
        api_path: String,
        url_encoded_body: String,
    ) -> String {
        let hash_digest =
            Sha256::digest(format!("{}{}", self.nonce, url_encoded_body).as_bytes());
        let private_key =
            base64::decode(&self.config.secret).expect("invalid private key");
        let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
        let mut hmac_data = api_path.into_bytes();
        hmac_data.append(&mut hash_digest.to_vec());
        mac.update(&hmac_data);
        base64::encode(mac.finalize().into_bytes())
    }

    fn get_headers(&self, signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("API-Key", HeaderValue::from_str(&self.config.key).unwrap());
        headers.insert("API-Sign", HeaderValue::from_str(signature).unwrap());
        headers
    }

    fn send_post(&mut self, path: &str, body: &str) -> Result<String, reqwest::Error> {
        let payload_nonce = format!("nonce={}", &self.nonce.to_string());
        let signature = self.get_signature(path.to_owned(), payload_nonce.to_owned());

        let endpoint = format!("https://api.kraken.com{}", path);

        let headers = self.get_headers(&signature);

        let response = self
            .http
            .post(&endpoint)
            .headers(headers)
            .body(payload_nonce)
            .send()?;

        let text = response.text()?;
        Ok(text)
    }

    fn send_get(&mut self, path: &str) -> Result<String, reqwest::Error> {
        let endpoint = format!("https://api.kraken.com{}", path);

        let response = self.http.get(&endpoint).send()?;

        let text = response.text()?;
        Ok(text)
    }

}

#[derive(Debug, Deserialize, Serialize)]
struct Ohlc {
    time: f64,
    open: String,
    high: String,
    low: String,
    close: String,
    vwap: String,
    volume: String,
    count: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct OrderBook {
    asks: Vec<(String, String, u64)>,
    bids: Vec<(String, String, u64)>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Trade {
    price: String,
    volume: String,
    time: f64,
    buy_sell: String,
    market_limit: String,
    miscellaneous: String,
    trade_id: f64,
}

struct TradingData {
    kraken_client: KrakenClient,
    ohlc_data: Option<Vec<Ohlc>>,
    order_book_data: Option<OrderBook>,
    trade_data: Option<Vec<Trade>>,
}

impl TradingData {
    fn new(kraken_client: KrakenClient) -> Self {
        TradingData {
            kraken_client,
            ohlc_data: None,
            order_book_data: None,
            trade_data: None,
        }
    }

    fn get_ohlc_data(&mut self, pair: &str, interval: &str, since: Option<u64>, use_cache: bool) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("/0/public/OHLC?pair={}&interval={}", pair, interval);

        if use_cache && self.ohlc_data.is_some() {
            return Ok(());
        }
        println!("hello!");

        let response = if let Some(since) = since {
            self.kraken_client.send_get(&format!("{}&since={}", path, since))
        } else {
            self.kraken_client.send_get(&path)
        }?;

        let value: Value = serde_json::from_str(&response)?;
        let prep = &value["result"]["XXBTZUSD"].to_string();
        let data: Vec<Ohlc> = serde_json::from_str(&prep)?;
        self.ohlc_data = Some(data);

        Ok(())
    }

    fn get_order_book_data(
        &mut self,
        pair: &str,
        count: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("/0/public/Depth?pair={}&count={}", pair, count);
        let response = self.kraken_client.send_get(&path)?;
        let value: Value = serde_json::from_str(&response)?;
        let prep = &value["result"]["XXBTZUSD"].to_string();
        let data: OrderBook = serde_json::from_str(&prep)?;
        self.order_book_data = Some(data);
        Ok(())
    }

    fn get_trade_data(
        &mut self, 
        pair: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("/0/public/Trades?pair={}", pair);
        let response = self.kraken_client.send_get(&path)?;
        let value: Value = serde_json::from_str(&response)?;
        let prep = &value["result"]["XXBTZUSD"].to_string();
        let data: Vec<Trade> = serde_json::from_str(&prep)?;
        self.trade_data = Some(data);

        Ok(())
    }
}

fn main() {

    let config_data = std::fs::read_to_string("config.json").expect("unable to read file");
    let config: Config = serde_json::from_str(&config_data).expect("config file is incorrect");
    
    let kraken_client = KrakenClient::new(config);

    let mut trading_data = TradingData::new(kraken_client); 
    trading_data.get_ohlc_data("XBTUSD", "1", None, false); // get OHLC data for the XBT/USD pair, 1 minute interval, last 10 data points
    trading_data.get_order_book_data("XBTUSD", 20);
    trading_data.get_trade_data("XBTUSD");
    
    if let Some(ohlc_data) = &trading_data.trade_data {
        println!("{:?}", ohlc_data)
    } else {
        println!("Fail");
    }

}


