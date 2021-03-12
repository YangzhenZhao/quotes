use chrono::NaiveTime;
use http_client::isahc::IsahcClient;

use crate::quote;
use quote::{Quote, Tick};

pub struct SinaQuote {
    pub client: surf::Client,
}

impl Quote for SinaQuote {
    fn new() -> Self {
        let isahc_client = IsahcClient::new();
        Self {
            client: surf::Client::with_http_client(isahc_client),
        }
    }

    fn base_url() -> String {
        "http://hq.sinajs.cn/list=".to_string()
    }

    fn client(&self) -> &surf::Client {
        &self.client
    }

    fn parse_out_tick(msg: &str) -> Option<Tick> {
        let field_list: Vec<&str> = msg.split(',').collect();
        if field_list.len() < 30 {
            return None;
        }
        let code_name_part: Vec<&str> = field_list[0].split(r#"=""#).collect();
        let name = code_name_part[1].to_string();
        let code = code_name_part[0]
            .get(code_name_part[0].len() - 6..)?
            .to_string();
        let time = NaiveTime::parse_from_str(field_list[31], "%H:%M:%S").ok()?;
        let current_price: f64 = field_list[3].parse().ok()?;
        let pre_close: f64 = field_list[2].parse().ok()?;
        let open: f64 = field_list[1].parse().ok()?;
        let high: f64 = field_list[4].parse().ok()?;
        let low: f64 = field_list[5].parse().ok()?;
        let total_amount: f64 = field_list[9].parse().ok()?;
        let total_vol: f64 = field_list[8].parse().ok()?;
        let mut ask: [f64; 5] = [0.0; 5];
        let mut bid: [f64; 5] = [0.0; 5];
        let mut ask_vol: [i32; 5] = [0; 5];
        let mut bid_vol: [i32; 5] = [0; 5];
        for i in 0..5usize {
            ask[i] = field_list[21 + i * 2].parse().ok()?;
            bid[i] = field_list[11 + i * 2].parse().ok()?;
            ask_vol[i] = field_list[20 + i * 2].parse().ok()?;
            bid_vol[i] = field_list[10 + i * 2].parse().ok()?;
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
