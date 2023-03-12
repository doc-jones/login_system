use auth::{login, LoginAction, Role, DeniedReason};

fn user_accepted(role: &Role) {
    println!("You are logged in as {role:?}");
}
fn main() {
    println!("Welcome to the not at all secure login system");
    println!("Enter your username:");

    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line( &mut input).expect("No keyboard!");

    match login(&input) {
        None => println!("{} is not a known user.", input.trim()),
        Some(login_action) => {
            login_action.do_login(user_accepted, |reason| {
                println!("Log in not allowed! {reason:?}");
            })
        }
        _ => {}
    }
}

