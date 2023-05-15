use clap::Parser;

const SAVES: &str = "saves.txt";

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = SAVES.to_string())]
    pub path: String,
}