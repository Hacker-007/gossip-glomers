mod echo;
mod maelstrom;
mod unique_id;

fn main() -> anyhow::Result<()> {
    // echo::run_service()?;
    unique_id::run_service()?;
    Ok(())
}
