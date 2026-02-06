use tokio;
mod sync_closures;
use sync_closures::*;
mod async_closures;
use async_closures::*;

#[tokio::main]
async fn main() {
    let s = "sync borrow".to_string();
    sync_closure_that_borrows(s);

    let s = "sync move".to_string();
    sync_closure_that_takes_ownership(s);

    let s = "async borrow".to_string();
    async_closure_that_borrows(s).await;

    let s = "async move".to_string();
    async_closure_that_takes_ownership(s).await;

    let s = "reusable".to_string();
    async_closure_reusable(s).await;

    let s = "single-use".to_string();
    async_closure_single_use(s).await;

    let s = "spawned".to_string();
    async_closure_with_spawn(s).await;

    // Give spawned task time to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
}
