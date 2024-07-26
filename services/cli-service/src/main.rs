use clap::{Command, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    username: String,
    content: String,
}

#[derive(Parser)]
#[command(name = "social_media_cli")]
#[command(version = "1.0")]
#[command(about = "A simple CLI social media app, to stay in touch with other people.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Signup {
        username: String,
        password: String,
    },
    Post {
        username: String,
        password: String,
        content: String,
    },
    View,
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Signup { username, password } => {
            println!("you are now signed up")
        }
        Commands::Post {
            username,
            password,
            content,
        } => {
            println!("you have posted: {content}")
        }
        Commands::View => {
            println!("Nothing to view")
        }
    }
}
