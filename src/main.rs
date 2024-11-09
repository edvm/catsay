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
}


fn main() {
    let options = Options::parse();
    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("Cats do not bark!");
        std::process::exit(1);
    }

    let eye = if options.dead { "x" } else { "0" };

    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("   \\");
    println!("    /\\_/\\");
    println!("   ( {eye}.{eye} )", eye=eye.red().bold());
    println!("    > ^ <");
    println!("   /  -  \\");
    println!("  /        \\/");
    println!(" /__|___|___\\");
}
