#[derive(Debug, Clone)]
pub struct Irq {
    pub priority: u8,
    pub is_enabled: bool,
    pub is_pending: bool,
}
