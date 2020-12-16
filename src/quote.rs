use chrono::NaiveTime;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Tick {
    pub time: NaiveTime,
    pub code: String,
    pub name: String,
    pub current_price: f64,
    pub pre_close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub total_amount: f64,
    pub total_vol: f64,
    pub ask: [f64; 5],
    pub bid: [f64; 5],
    pub ask_vol: [i32; 5],
    pub bid_vol: [i32; 5],
}

#[async_trait]
pub trait Quote {
    fn new() -> Result<Self, isahc::Error>
    where
        Self: Sized;

    fn base_url() -> String;

    fn client(&self) -> &surf::Client;

    fn parse_out_tick(msg: &str) -> Option<Tick>;

    async fn small_tick_map(&self, codes: &Vec<&str>) -> HashMap<String, Tick> {
        let mut res_map = HashMap::new();
        let request = surf::get(format!("{}{}", Self::base_url(), codes.join(","))).build();
        if let Ok(s) = self
            .client()
            .send(request)
            .await
            .unwrap()
            .body_string()
            .await
        {
            for item in s.split("\n") {
                if let Some(tick) = Self::parse_out_tick(item) {
                    res_map.insert(tick.code.clone(), tick);
                }
            }
        }
        res_map
    }

    async fn tick_map(&self, codes: &Vec<&str>) {
        let request = surf::get(format!("{}sz000001", Self::base_url())).build();
        match self
            .client()
            .send(request)
            .await
            .unwrap()
            .body_string()
            .await
        {
            Ok(s) => {
                println!("{:?}", s);
                Self::parse_out_tick(&s);
            }
            Err(e) => println!("err = {}", e),
        }
    }
}
