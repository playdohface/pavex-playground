#[derive(Clone)]
pub struct Lifecycle;
impl Lifecycle {
    pub fn startup() -> Self {
        tracing::info!("Startup!");
        println!("Startup!");
        Lifecycle {}
    }
}
unsafe impl Sync for Lifecycle {}
unsafe impl Send for Lifecycle {}

impl core::ops::Drop for Lifecycle {
    fn drop(&mut self) {
        tracing::info!("Graceful.");
        println!("Shutdown");
    }
}
