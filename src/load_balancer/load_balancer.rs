pub trait LoadBalancer {
    fn new() -> Self;
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn stop(&self);
}
