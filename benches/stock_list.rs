#![feature(test)]

extern crate test;

use async_std::task;
use quotes::stock_list;
use test::Bencher;

#[bench]
fn bench_stock_list(b: &mut Bencher) {
    b.iter(|| {
        task::block_on(async {
            stock_list().await;
        })
    });
}
