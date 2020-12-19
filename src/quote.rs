use crate::error::Error;
use crate::utils::format_stock_code;
use async_trait::async_trait;
use chrono::NaiveTime;
use futures::future::join_all;
use std::collections::HashMap;

const REQ_CODES_NUM_MAX: usize = 300;

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

    async fn small_tick_map(&self, codes: &[String]) -> Result<HashMap<String, Tick>, Error> {
        let mut res_map = HashMap::new();
        let request = surf::get(format!("{}{}", Self::base_url(), codes.join(","))).build();
        if let Ok(s) = self.client().send(request).await?.body_string().await {
            for item in s.split('\n') {
                if let Some(tick) = Self::parse_out_tick(item) {
                    res_map.insert(tick.code.clone(), tick);
                }
            }
        }
        Ok(res_map)
    }

    async fn tick(&self, code: &str) -> Result<Tick, Error> {
        let tick_map = self.tick_map(vec![code]).await?;
        match tick_map.into_iter().next() {
            None => Err(Error::Msg("Request tick error!")),
            Some((_, t)) => Ok(t),
        }
    }

    async fn current_price(&self, code: &str) -> Result<f64, Error> {
        let tick = self.tick(code).await?;
        Ok(tick.current_price)
    }

    async fn pre_close(&self, code: &str) -> Result<f64, Error> {
        let tick = self.tick(code).await?;
        Ok(tick.pre_close)
    }

    async fn open(&self, code: &str) -> Result<f64, Error> {
        let tick = self.tick(code).await?;
        Ok(tick.open)
    }

    async fn pct_change(&self, code: &str) -> Result<f64, Error> {
        let tick = self.tick(code).await?;
        let pct_tmp = tick.current_price / tick.pre_close * 10000.0 - 10000.0;
        Ok(pct_tmp.round() / 100.0)
    }

    async fn tick_map(&self, codes: Vec<&str>) -> Result<HashMap<String, Tick>, Error> {
        let codes_len = codes.len();
        let codes = format_stock_code(codes);
        let mut task_list = vec![];
        let mut idx = 0;
        while idx < codes_len {
            if idx + REQ_CODES_NUM_MAX < codes_len {
                task_list.push(self.small_tick_map(&codes[idx..idx + REQ_CODES_NUM_MAX]));
                idx += REQ_CODES_NUM_MAX;
            } else {
                task_list.push(self.small_tick_map(&codes[idx..]));
                break;
            }
        }
        let res_list = join_all(task_list).await;
        let mut ticks = HashMap::new();
        for res in res_list.into_iter() {
            for (code, tick) in res?.into_iter() {
                ticks.insert(code, tick);
            }
        }
        Ok(ticks)
    }
}
