use clap::{Parser, Subcommand};
use auth::get_users;

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// List all users,
    List,
}

fn list_users(users: &UserList) {
    println!("{:<20}, {:<20}", "Username", "Login Action");
    println!("{:-<40}", "");
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            let users = get_users();
            list_users(&users);
        }
        None => {
            println!("Run with --help to see instructions.");
            std::process::exit(0);
        }
    }
}
