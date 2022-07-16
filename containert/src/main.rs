use clap::{Parser, Subcommand}; // bring parser and subcommand traits into scope

mod runtime;
mod filesystem;

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
        
        /// Specifies the command to run
        #[clap(short, long, action)]
        command: String,
        
        /// Specifies the arguments to the command to run
        #[clap(short, long, action)]
        args: Vec<String>,

        /// Specifies the path to the directory to mount as the rootfs of the container
        #[clap(short, long, action)]
        rootfs: String
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
        Some(Commands::Run { image, command, args, rootfs}) => {
            if !image.is_empty() {
                println!("Running image: {}", image);
                let runtime = runtime::Runtime{cmd: command.to_string(), args: args.to_vec(), rootfs: rootfs.to_string()};
                let result = runtime.run().unwrap();
                println!("{}", result);
            } else {
                println!("No image specified");
            }
        },
        Some(Commands::Pull {image}) => {
            if !image.is_empty() {
                let output = filesystem::pull_image(&image.to_string()).expect("error pulling image");
                let output_string = String::from_utf8(output).unwrap();
                println!("{:?}", output_string);
            } else {
                println!("No image specified");
            }
        },
        &None => todo!()
    }

}
