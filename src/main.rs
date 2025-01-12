use clap::Parser;

mod countdown;

const DEFAULT_COUNTDOWN_MINUTES: u64 = 25;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    minutes: Option<u64>,
}

fn main() {
  let args = Args::parse();
  countdown::start(args.minutes.unwrap_or(DEFAULT_COUNTDOWN_MINUTES));
}
