use clap::Subcommand;

pub mod actions;
pub mod model;

#[derive(Subcommand)]
pub enum WlCommands {
    /// Add a new show to your watch later list
    Add {
        #[arg(short, long)]
        show: String,
    },
    /// Remove a show from your watch later list
    Remove {
        #[arg(short, long)]
        show: String,
    },
    /// Remove a show from your watch later list, and start watching it.
    Start {
        #[arg(short, long)]
        show: String,
        #[arg(short, long)]
        link: String,
    },
    /// Print the entire contents of your watch later file
    List,
}
