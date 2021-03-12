use async_std::task;
use quotes::{Quote, SinaQuote, TencentQuote};

#[test]
fn test_sina() {
    task::block_on(async {
        let quote = SinaQuote::new();
        let tick = quote.tick("000001").await.unwrap();
        assert!(tick.name == "平安银行");
    });
}

#[test]
fn test_tencent() {
    task::block_on(async {
        let quote = TencentQuote::new();
        let tick = quote.tick("000001").await.unwrap();
        assert!(tick.name == "平安银行");
    });
}
