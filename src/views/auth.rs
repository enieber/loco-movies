use crate::models::_entities::users;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

/// Render auth of `movies`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn login(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "auth/login.html", data!({}))
}

/// Render auth of `movies`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn register(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "auth/register.html", data!({}))
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}
