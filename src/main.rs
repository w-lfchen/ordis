use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    if let Err(e) = dotenvy::dotenv()
        && !e.not_found()
    {
        return Err(anyhow::Error::new(e));
    }
    let _ = Args::parse();

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short('t'), long, env)]
    bot_token: String,
}
