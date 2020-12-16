mod quote;
mod sina;
use quote::Quote;
use sina::SinaQuote;
mod tencent;
use tencent::TencentQuote;

#[async_std::main]
async fn main() {
    let quote = SinaQuote::new().unwrap();
    let res = quote.small_tick_map(&vec!["sz000001", "sz000002"]).await;
    println!("{:?}", res);
}
