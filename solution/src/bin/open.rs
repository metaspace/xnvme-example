use clap::Parser;

#[derive(Parser)]
struct Args {
    /// xNVME URI
    uri: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    use xnvme::options::*;
    let options = OptionsBuilder::default()
        .backend(Backend::Linux)
        .rdonly(true)
        .build();
    let dev = xnvme::device::DeviceHandle::open(args.uri.as_str(), &options?)?;
    Ok(())
}
