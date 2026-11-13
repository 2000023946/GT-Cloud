pub trait SendHeartbeat {
    fn send(&self) -> Result<(), String>;
}