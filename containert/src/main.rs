use clap::{Parser, Subcommand}; // bring parser and subcommand traits into scope
use std::fs;

#[path = "oci/pull.rs"] mod pull;

#[derive(Parser)] // implement the Parser trait for the containert struct
#[clap(author, version, about)] // in help system, output author, version, about
struct Containert {
    #[clap(value_parser)] 
    name: Option<String>,

    #[clap(subcommand)] // specifies to clap that this is the subcommand to containert
    command: Option<Commands>
}

#[derive(Subcommand)] // implements the subcommand trait, specifies which subcommands are valid for containert
enum Commands {
    /// Runs the specified image with the containert runtime
    Run {
        /// Specifies the image to run
        #[clap(short, long, action)]
        image: String,
    },

    Pull {
        /// Pulls an image
        #[clap(short, long, action)]
        image: String,
    }
}

fn main() {
    let cli = Containert::parse();

    match &cli.command {
        Some(Commands::Run { image}) => {
            if !image.is_empty() {
                println!("Running image: {}", image);
            } else {
                println!("No image specified");
            }
        },
        Some(Commands::Pull {image}) => {
            if !image.is_empty() {
                println!("Pulling image: {}", image);
                let image_registry = format!("docker://{}", image);
                let dir = format!("/var/lib/containert/{}", image);
                fs::create_dir_all(dir).expect("failed creating image directory");
                let full_dir = format!("dir:/var/lib/containert/{}", image);
                let output = pull::pull_image(full_dir.to_string(), image_registry.to_string()).expect("error pulling image");
                let output_string = String::from_utf8(output).ok();
                println!("{:?}", output_string);
            } else {
                println!("No image specified");
            }
        },
        &None => todo!()
    }

}
