use crate::errors::AppError;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use rand::Rng;
use std::iter;

/// 生成哈希密码
pub async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generating password hash")?
}

/// 验证密码
pub async fn verify_password(
    password: Option<String>,
    password_hash: Option<String>,
) -> Result<(), AppError> {
    if password.is_none() || password_hash.is_none() {
        return Err(AppError::VALIDATE_FIELD_ERROR("密码不正确".to_string()));
    }

    let password = password.unwrap();
    let password_hash = password_hash.unwrap();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|_| AppError::VALIDATE_FIELD_ERROR("密码读取失败".to_string()))?;
        let result = hash.verify_password(&[&Argon2::default()], password);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::VALIDATE_FIELD_ERROR("密码不正确".to_string())),
        }
    })
    .await
    .context("panic in verifying password hash")?
}

/// 生成指定长度的字符串
#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::thread_rng().sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}
