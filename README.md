<h1 align="center">quotes</h1>
<div align="center">
 <strong>
   获取股票行情、股票列表等信息
 </strong>
</div>
<div align="center">

<img src="https://github.com/Yangzhenzhao/quotes/workflows/CI/badge.svg" />
<a href="https://crates.io/crates/quotes">
    <img src="https://img.shields.io/crates/v/quotes.svg?style=flat-square"
    alt="Crates.io version" />
</a>  
<a href="https://docs.rs/quotes">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
    alt="docs.rs docs" />
</a>   
</div>



<a href="https://docs.rs/quotes/" target="_blank">Documentation</a>


### Installation


```
[dependencies]
quotes = "0.1.2"
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
