use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 's', long = "stdin")]
    /// Read the message from standard input
    stdin: bool,

    #[clap(short = 'f', long = "file")]
    /// Load a cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("Cats do not bark!");
        std::process::exit(1);
    }

    let eye = if options.dead { "x" } else { "0" };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", &message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("   \\");
            println!("    /\\_/\\");
            println!("   ( {eye}.{eye} )", eye = eye.red().bold());
            println!("    > ^ <");
            println!("   /  -  \\");
            println!("  /        \\/");
            println!(" /__|___|___\\");
        }
    }
    return Ok(());
}
