use clap::{Parser, Subcommand};
use auth::{ get_users, UserList, LoginAction, Role, User};

#[derive(Parser)]
#[command()]
struct Args {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// List all users.
  List,
}

fn list_users(users: &UserList) {
    use colored::Colorize;
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    users.iter().for_each(|(_key, user)| {
        let action = format!("{:?}", user.action);
        let action = match user.action {
            auth::LoginAction::Accept(..) => action.green(),
            auth::LoginAction::Denied(..) => action.red(),
        };
        println!("{:<20}{:<20}", user.username, action);
    });
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
