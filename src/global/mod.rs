// bindings for embedded script
use deno_core::error::AnyError;
use deno_core::op2;

#[op2(async)]
pub async fn set_timeout(delay: f64) -> Result<(), AnyError> {
    tokio::time::sleep(std::time::Duration::from_millis(delay as u64)).await;
    Ok(())
}
