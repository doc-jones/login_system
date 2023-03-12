#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

impl LoginAction {
    fn standard_user() -> Option<Self>{
        Some(LoginAction::Accept(Role::User))
    }

    pub fn do_login(&self) {
        match self {
            Self::Accept(role) => do_something(),
            Self::Denied(reason) => do_something_else(),
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
