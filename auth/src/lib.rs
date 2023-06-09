/// # Authentication
/// 
/// This module provides an insecure login environment that can be used with other servers and apps.

/// #![warn(missing_docs)]

use serde::{Deserialize, Serialize};
pub use user::User;
use std::collections::HashMap;
mod user;

pub mod prelude {
    pub use crate::hash_password;
}

/// `hash_password` applies a SHA256 digest to a string
/// and returns a text representation of the hash (in hex).
/// 
/// ## Arguments
/// 
/// * `password` - the password to hash.
/// 
/// ```
/// use auth::hash_password;
/// println!("{}", hash_password("test"));
/// ```

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn build_users_file() {
    use std::io::Write;
    let users = get_users();
    let json = serde_json::to_string(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

pub fn save_users_file(users: &UserList) {
    use std::io::Write;
    let json = serde_json::to_string(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

/*pub fn get_users() -> HashMap<String, User> {
    let users = vec![
        User::new("doc", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password2", LoginAction::Accept(Role::User)),
        User::new("susan", "password3", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ];
    let user_tuple = users
        .iter()
        .map(|user| (user.clone(), user.clone()))
        .collect();
    user_tuple
} */

pub type UserList = HashMap<String, User>;

pub fn get_users() -> HashMap<String, User> {
    let json = std::fs::read_to_string("users.json").unwrap();
    serde_json::from_str(&json).unwrap()
}

pub fn login(users: &HashMap<String, User>, username: &str, password: &str) -> Option<LoginAction>
{
    let username = username.trim().to_lowercase();
    let password = hash_password(password.trim());
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(user.action.clone());
        }
    }
    None
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

impl LoginAction {
    fn standard_user() -> Option<Self>{
        Some(LoginAction::Accept(Role::User))
    }

    pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
        match self {
            Self::Accept(role) => on_success(role),
            Self::Denied(reason) => on_denied(reason),
        }
    }
}


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Limited,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked { reason: String },
    NoAccount { action: String },
}
/*
pub fn login(name: &str) -> Option<LoginAction> {
    match name.to_lowercase().trim() {
        "doc" => Some(LoginAction::Accept(Role::Admin)),
        "bob" | "susan" => LoginAction::standard_user(),
        _ => None,
    }
}*/

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "doc"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        let doc = "Doc".to_string();
        assert_eq!("Hello Doc", greet_user(&doc));
    }

    #[test]
    fn test_login_fail() {
        assert!(!is_login_allowed("jones"));
    }

    #[test]
    fn test_case_and_trim() {
        assert!(is_login_allowed("DoC"));
        assert!(is_login_allowed("doc\r\n"));
    }

    /*#[test]
    fn test_enums() {
        assert_eq!(login("doc"), Some(LoginAction::Accept(Role::Admin)));
        assert_eq!(login("bob"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(login("susan"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(login("anonymous"), None);
    }*/
}
