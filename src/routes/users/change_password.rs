use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::users::get_username;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?;
    if user_id.is_none() {
        println!("Couldn't find user");
        return Ok(see_other("/login"));
    };
    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        return Ok(see_other("/users/password"));
    }
    let username = get_username(user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                // FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/users/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        };
    }
    crate::authentication::change_password(user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;
    // FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/users/password"))
}
