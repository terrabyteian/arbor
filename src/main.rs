use clap::Parser;
mod tree;
use std::process::Command;
use tree::TreeClimber;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    path: String,

    #[arg(short, long)]
    all: bool,

    #[arg(value_name = "COMMAND")]
    command: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let path = args.path;
    let all = args.all;
    let command = args.command;

    let t = TreeClimber::new().path(path).all(all).climb()?;
    for p in t {
        let output = Command::new(&command).current_dir(p).output()?;
        let stdout = String::from_utf8(output.stdout)?;
        println!("{}", stdout)
    }

    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    // ratatui::restore();

    Ok(())
}
