use rufu_common::enums::user_domain_enum::UserDomainEnum;

#[derive(Clone)]
pub struct UserExt {
    user_id: String,
    domain: UserDomainEnum,
}

pub trait UserEntityTrait {
    fn get_user_id(&self) -> String;
    fn get_user_domain(&self) -> UserDomainEnum;
}

impl UserExt {
    pub fn new(user_id: String, domain: String) -> Self {
        match domain.to_uppercase().as_str() {
            "ADMIN" => Self {
                user_id,
                domain: UserDomainEnum::ADMIN,
            },
            "API" => Self {
                user_id,
                domain: UserDomainEnum::API,
            },
            _ => {
                // 默认为API
                Self {
                    user_id,
                    domain: UserDomainEnum::API,
                }
            }
        }
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.to_string()
    }

    pub fn get_domain(&self) -> UserDomainEnum {
        self.domain.clone()
    }
}
