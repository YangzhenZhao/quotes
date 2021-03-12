use async_std::task;
use quotes::stock_list;

#[test]
fn test_stock_list() {
    task::block_on(async {
        let res = stock_list().await;
        assert!(res.is_ok());
        assert!(res.unwrap().len() > 4000);
    });
}
