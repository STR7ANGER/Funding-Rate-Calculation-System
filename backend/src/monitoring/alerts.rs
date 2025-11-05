pub struct AlertService;

impl AlertService {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn send_alert(&self, _msg: &str) -> anyhow::Result<()> { Ok(()) }
}

