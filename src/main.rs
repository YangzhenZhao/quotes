mod quote;
mod sina;
use quote::Quote;
use sina::SinaQuote;
mod stock_list;
mod tencent;
use stock_list::{stock_list, stock_list_sz};
use tencent::TencentQuote;
mod error;

#[async_std::main]
async fn main() {
    // let quote = SinaQuote::new().unwrap();
    // let res = quote.small_tick_map(&vec!["sz000001", "sz000002"]).await;
    let l = stock_list().await.unwrap();
    println!("{:?} {}", l.get(..10), l.len());
}
