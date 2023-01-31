use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmailResendInfo {
    pub user_id: i64,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct ApiResult {
    pub result: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct EmailConfirmationResult {
    pub result: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct RegisterResponse {
    pub result: String,
    pub data: Option<UserInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct EmailDetail {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct UserInfo {
    pub id: i64,
    pub token: String,
    pub username: String,
    pub emails: Vec<EmailDetail>,
    pub permissions: Vec<String>,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }

    pub fn primary_email(&self) -> Option<String> {
        self.emails
            .iter()
            .filter_map(|e| {
                if e.primary {
                    Some(e.email.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .first()
            .cloned()
    }

    pub fn non_validated_emails(&self) -> Vec<String> {
        let mut emails = self.emails.clone();
        emails.sort_by(|a, b| a.primary.cmp(&b.primary));
        emails
            .iter()
            .filter_map(|e| {
                if e.verified {
                    None
                } else {
                    Some(e.email.clone())
                }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaltResponse {
    pub salt: String,
    pub challenge: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[cfg(test)]
mod tests {
    use crate::types::auth::{EmailDetail, UserInfo};

    #[test]
    fn authenticated() {
        let user = UserInfo::default();
        assert_eq!(user.is_authenticated(), false)
    }

    #[test]
    fn primary_email_none() {
        let user = UserInfo::default();
        assert_eq!(user.primary_email(), None)
    }

    #[test]
    fn primary_email_some() {
        let mut user = UserInfo::default();
        user.emails.push(EmailDetail {
            email: "test@example.com".to_string(),
            verified: false,
            primary: false,
        });
        user.emails.push(EmailDetail {
            email: "primary@example.com".to_string(),
            verified: true,
            primary: true,
        });
        assert_eq!(
            user.primary_email(),
            Some("primary@example.com".to_string())
        )
    }

    #[test]
    fn non_validated_emails() {
        let mut user = UserInfo::default();
        user.emails.push(EmailDetail {
            email: "test@example.com".to_string(),
            verified: false,
            primary: false,
        });
        user.emails.push(EmailDetail {
            email: "primary@example.com".to_string(),
            verified: true,
            primary: true,
        });
        assert_eq!(
            user.non_validated_emails(),
            vec!["test@example.com".to_string()]
        )
    }
}
