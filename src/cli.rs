use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Run the puzzle for the given day and part
    Run {
        day: Option<u8>,
        part: Option<u8>,
        sample: Option<bool>,
    },
}
