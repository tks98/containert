use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(value_parser)]
    name: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the specified image with the containert runtime
    Run {
        /// Specifies the image to run
        #[clap(short, long, action)]
        image: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { image}) => {
            if !image.is_empty() {
                println!("Running image: {}", image);
            } else {
                println!("No image specified");
            }
        }
        None => {}
    }

}
