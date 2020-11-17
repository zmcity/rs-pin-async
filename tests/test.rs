#[pin_async::pin_async]
async fn async_fn(i: i32) {
    // i + 15
}


use std::future::Future;
use std::pin::Pin;


fn get_async_block() -> (fn(i32)-> Pin<Box<dyn Future<Output=()>>>){
    async_fn
}

#[test]
fn call() {
    let f: fn(i32)-> Pin<Box<dyn Future<Output=()>>> = get_async_block();
    println!("{:?}",async_std::task::block_on(f(10)));
}
