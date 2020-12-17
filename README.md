### quotes


[![Crates.io](https://img.shields.io/crates/v/quotes.svg)](https://crates.io/crates/quotes)
[![Documentation](https://docs.rs/quotes/badge.svg)][documentation]   

获取股票行情、股票列表等信息

<a href="https://docs.rs/quotes/" target="_blank">Documentation</a>


### Installation


```
[dependencies]
isahc = "0.1.1"
```


### Examples

```rs
#[async_std::main]
async fn main() {
    match quotes::stock_list().await {
        Ok(l) => println!("len = {}", l.len()),
        Err(e) => println!("err = {}", e),
    }
}
```
