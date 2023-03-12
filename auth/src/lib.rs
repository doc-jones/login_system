pub struct User {
    username: String,
    password: String,
    action: LoginAction,
}

impl User {
    pub fn new(username: &str, password: &str, action: LoginAction)-> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            action 
        }
    }
}

pub fn get_users() -> [User; 3] {
    [
        User::new("doc", "password", LoginAction::Accept(Role)),
        User::new("bob", "password2", LoginAction::Accept(Role)),
        User::new("susan", "password3", LoginAction::Denied(DeniedReason)),
    ]
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
    Limited,
}

#[derive(PartialEq, Debug)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked { reason: String },
    NoAccount { action: String },
}

pub fn login(name: &str) -> Option<LoginAction> {
    match name.to_lowercase().trim() {
        "doc" => Some(LoginAction::Accept(Role::Admin)),
        "bob" | "susan" => LoginAction::standard_user(),
        _ => None,
    }
}

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

    #[test]
    fn test_enums() {
        assert_eq!(login("doc"), Some(LoginAction::Accept(Role::Admin)));
        assert_eq!(login("bob"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(login("susan"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(login("anonymous"), None);
    }
}
