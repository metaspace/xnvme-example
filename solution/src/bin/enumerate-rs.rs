use anyhow::Context;
use anyhow::Result;
use xnvme::enumerate;

fn main() -> Result<()> {
    let mut list =
        enumerate::list::ListHandle::new(100).context("Failed to create enumeration list")?;
    enumerate::Enumerator::enumerate(&mut list)?;
    list.pp().context("Failed to pretty print")?;
    Ok(())
}
