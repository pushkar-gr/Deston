pub trait LoadBalancer {
    //returns a LoadBalancer
    fn new() -> Self;
    //starts the load balancer
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    //stops the load balancer
    fn stop(&self);
}
