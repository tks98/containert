use clap::{Parser, Subcommand}; // bring parser and subcommand traits into scope

#[path = "oci/image.rs"] mod image;

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
                let output = image::pull_image(image.to_string()).expect("error pulling image");
                let output_string = String::from_utf8(output).unwrap();
                println!("{:?}", output_string);
            } else {
                println!("No image specified");
            }
        },
        &None => todo!()
    }

}
