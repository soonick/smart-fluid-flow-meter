use crate::api::user::{SignUpUserInput, User, UserAuthProvider::Password};
use crate::error::app_error::{
    AppError, FailedValidation,
    ValidationIssue::{Invalid, TooWeak},
};
use crate::json::extractor::Extractor;
use crate::AppState;

use axum::extract::State;
use chrono::Local;

/**
 * Creates a new user in the system.
 */
pub async fn sign_up_user(
    State(state): State<AppState>,
    Extractor(input): Extractor<SignUpUserInput>,
) -> Result<Extractor<User>, AppError> {
    if state.user_helper.password_is_weak(&input.password) {
        let validation_errors = vec![FailedValidation {
            field: "password".to_string(),
            issue: TooWeak,
        }];
        return Err(AppError::ValidationError(validation_errors));
    }

    if state
        .user_helper
        .is_bot(&state.settings.captcha.secret, &input.captcha, "userip")
        .await
    {
        let validation_errors = vec![FailedValidation {
            field: "captcha".to_string(),
            issue: Invalid,
        }];
        return Err(AppError::ValidationError(validation_errors));
    }

    let password_hash = match state.user_helper.hash(&input.password) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };

    let id = format!("{}+{}", input.email, Password);
    let user = User {
        id,
        provider: Password,
        email: input.email.clone(),
        password: Some(password_hash),
        email_verified_at: None,
        recorded_at: Local::now(),
    };
    let inserted = state.storage.sign_up_user(user).await?;

    Ok(Extractor(User {
        id: inserted.id,
        provider: inserted.provider,
        email: input.email,
        password: None,
        email_verified_at: None,
        recorded_at: inserted.recorded_at,
    }))
}
