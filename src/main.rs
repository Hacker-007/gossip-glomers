mod echo;
mod maelstrom;

fn main() -> anyhow::Result<()> {
    echo::run_service()?;
    Ok(())
}
