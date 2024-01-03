use jassioxide::run;
use tracing_subscriber::FmtSubscriber;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    run()
}
