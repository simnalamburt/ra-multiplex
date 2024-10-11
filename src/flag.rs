use tokio::sync::watch;
use tracing::error;

/// An asynchronous primitive that allows raising a flag once
///
/// All past and future callers of the `wait` function will be woken up if or
/// once the flag is raised.
#[derive(Clone)]
pub struct Flag {
    sender: watch::Sender<bool>,
}

impl Flag {
    pub fn new() -> Self {
        let (sender, _) = watch::channel(false);
        Flag { sender }
    }

    pub async fn wait(&self) {
        match self.sender.subscribe().wait_for(|flag| *flag).await {
            Ok(_) => {}
            Err(e) => error!(?e, "unreachable"),
        }
    }

    pub fn raise(&self) {
        let _ = self.sender.send_replace(true);
    }
}
