use auth::{login, LoginAction, Role, DeniedReason};

fn main() {
    println!("Welcome to the not at all secure login system");
    println!("Enter your username:");

    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line( &mut input).expect("No keyboard!");

    match login(&input) {
        None => println!("{} is not a known user.", input.trim()),
        Some(LoginAction::Accept(role)) => {
            match role {
                Role::Admin => println!("Welcome Admin!"),
                Role::Limited => println!("Welcome, your access is limited."),
                Role::User => println!("Welcome User."),
            }
        }
        Some(LoginAction::Denied(DeniedReason::PasswordExpired)) => {
            println!("Expired Password");
        }
        _ => {}
    }
}

