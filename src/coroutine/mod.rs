pub mod http;
pub mod future;
pub mod coroutine;


pub fn async_main() -> impl future::Future<Output = ()> {
    coroutine::Coroutine::new()
}