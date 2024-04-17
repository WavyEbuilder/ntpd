mod ntpd;

use anyhow::Result;
use ntpd::NtpDaemon;

fn main() -> Result<()> {
    let ntp_daemon = NtpDaemon::new();
    ntp_daemon.start()?;

    Ok(())
}
