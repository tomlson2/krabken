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
struct Orderbook {
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
}

struct TradingData {
    kraken_client: KrakenClient,
    ohlc_data: Option<Vec<Ohlc>>,
    order_book_data: Option<Orderbook>,
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
        let prep = value["result"]["XXBTZUSD"].to_string();
        let data: Vec<Ohlc> = serde_json::from_str(&prep)?;
        self.ohlc_data = Some(data);

        Ok(())
    }

    // fn get_order_book_data(
    //     &mut self,
    //     pair: &str,
    //     count: u32,
    // ) -> Result<(), reqwest::Error> {
    //     let path = format!("/public/Depth?pair={}&count={}", pair, count);
    //     let response = self.kraken_client.send_get(&path)?;
    //     let data: KrakenResponse<OrderBook> = serde_json::from_str(&response)?;

    //     let order_book_data = data.result.map(|x| orderbook {
    //         asks: x
    //             .get(pair)
    //             .and_then(|v| v.asks.as_ref())
    //             .map(|asks| {
    //                 asks.iter()
    //                     .map(|v| (v[0].to_string(), v[1].to_string(), v[2] as u64))
    //                     .collect()
    //             })
    //             .unwrap_or_default(),
    //         bids: x
    //             .get(pair)
    //             .and_then(|v| v.bids.as_ref())
    //             .map(|bids| {
    //                 bids.iter()
    //                     .map(|v| (v[0].to_string(), v[1].to_string(), v[2] as u64))
    //                     .collect()
    //             })
    //             .unwrap_or_default(),
    //     });

    //     self.order_book_data = order_book_data;
    //     Ok(())
    // }

    // fn get_trade_data(&mut self, pair: &str) -> Result<(), reqwest::Error> {
    //     let path = format!("/public/Trades?pair={}", pair);
    //     let response = self.kraken_client.send_get(&path)?;
    //     let data: KrakenResponse<HashMap<String, Vec<KrakenTrade>>> =
    //         serde_json::from_str(&response)?;

    //     let trade_data = data
    //         .result
    //         .and_then(|mut x| x.remove(pair))
    //         .map(|x| {
    //             x.into_iter()
    //                 .map(|v| trade {
    //                     price: v.price.to_string(),
    //                     volume: v.volume.to_string(),
    //                     time: v.time,
    //                     buy_sell: v.buy_sell,
    //                     market_limit: v.market_limit,
    //                     miscellaneous: v.miscellaneous,
    //                 })
    //                 .collect()
    //         });

    //     self.trade_data = trade_data;
    //     Ok(())
    // }
}



fn main() {

    let config_data = std::fs::read_to_string("config.json").expect("unable to read file");
    let config: Config = serde_json::from_str(&config_data).expect("config file is incorrect");
    
    let kraken_client = KrakenClient::new(config);

    let mut trading_data = TradingData::new(kraken_client); 
    trading_data.get_ohlc_data("XBTUSD", "1", None, false); // get OHLC data for the XBT/USD pair, 1 minute interval, last 10 data points
    
    if let Some(ohlc_data) = &trading_data.ohlc_data {
        for ohlc in ohlc_data {
            println!("Time: {}, Open: {}, High: {}, Low: {}, Close: {}, VWAP: {}, Volume: {}, Count: {}", ohlc.time, ohlc.open, ohlc.high, ohlc.low, ohlc.close, ohlc.vwap, ohlc.volume, ohlc.count);
        }     
    } else {
        println!("Fail");
    }

}


