use chrono::NaiveTime;
use http_client::isahc::IsahcClient;

use crate::quote;
use quote::{Quote, Tick};

pub struct TencentQuote {
    pub client: surf::Client,
}

impl Quote for TencentQuote {
    fn new() -> Self {
        let isahc_client = IsahcClient::new();
        Self {
            client: surf::Client::with_http_client(isahc_client),
        }
    }

    fn base_url() -> String {
        "http://qt.gtimg.cn/q=".to_string()
    }

    fn client(&self) -> &surf::Client {
        &self.client
    }

    fn parse_out_tick(msg: &str) -> Option<Tick> {
        let field_list: Vec<&str> = msg.split('~').collect();
        if field_list.len() < 30 {
            return None;
        }
        let time_str = field_list[30].get(8..)?;
        let time = NaiveTime::parse_from_str(time_str, "%H%M%S").ok()?;
        let name = field_list[1].to_string();
        let code = field_list[2].to_string();
        let current_price: f64 = field_list[3].parse().ok()?;
        let pre_close: f64 = field_list[4].parse().ok()?;
        let open: f64 = field_list[5].parse().ok()?;
        let high: f64 = field_list[33].parse().ok()?;
        let low: f64 = field_list[34].parse().ok()?;
        let total_amount: f64 = field_list[37].parse::<f64>().ok()? * 10000.0;
        let total_vol: f64 = field_list[6].parse::<f64>().ok()? * 100.0;
        let mut ask: [f64; 5] = [0.0; 5];
        let mut bid: [f64; 5] = [0.0; 5];
        let mut ask_vol: [i32; 5] = [0; 5];
        let mut bid_vol: [i32; 5] = [0; 5];
        for i in 0..5usize {
            ask[i] = field_list[19 + i * 2].parse().ok()?;
            bid[i] = field_list[9 + i * 2].parse().ok()?;
            ask_vol[i] = field_list[20 + i * 2].parse::<i32>().ok()? * 100;
            bid_vol[i] = field_list[10 + i * 2].parse::<i32>().ok()? * 100;
        }
        Some(Tick {
            time,
            code,
            name,
            current_price,
            pre_close,
            open,
            high,
            low,
            total_amount,
            total_vol,
            ask,
            bid,
            ask_vol,
            bid_vol,
        })
    }
}
