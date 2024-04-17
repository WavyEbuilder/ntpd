use anyhow::Result;

#[derive(Default)]
pub(crate) struct NtpDaemon {}

impl NtpDaemon {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn start(&self) -> Result<()> {
        Ok(())
    }
}
