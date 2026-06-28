use std::sync::Arc;

use axum::{
    Json, Router,
    body::Body,
    debug_handler,
    extract::{Extension, State},
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, SET_COOKIE},
    },
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::cookie;
use serde_json::json;

use crate::{
    Result,
    context::AppContext,
    middlewares::{AuthError, AuthLayer, RefreshLayer},
    models::{LoginUser, RegisterUser, User},
};

#[debug_handler]
async fn register(
    State(ctx): State<Arc<AppContext>>,
    Json(params): Json<RegisterUser<'static>>,
) -> Result<Response> {
    let _new_user = User::create_user(&ctx.db, &params).await?;

    Ok((
        StatusCode::CREATED,
        Json(json! ({
            "message": "User created succesfully"
        })),
    )
        .into_response())
}

#[debug_handler]
async fn login(
    State(ctx): State<Arc<AppContext>>,
    Json(params): Json<LoginUser<'static>>,
) -> Result<Response> {
    let user = User::find_by_email(&ctx.db, params.email())
        .await?
        .ok_or(crate::Error::Auth(AuthError::WrongCredentials))?;

    user.verify_password(params.password())?;

    let access_token = ctx.auth.access.generate_token(user.pid())?;
    let refresh_token = ctx.auth.refresh.generate_token(user.pid())?;

    ctx.store_refresh_token(&refresh_token).await?;

    let access_token = access_token.token.unwrap();
    let refresh_token = refresh_token.token.unwrap();

    let access_cookie = cookie::Cookie::build(("access_token", &access_token))
        .path("/")
        .http_only(false)
        .max_age(time::Duration::seconds(ctx.auth.access.exp as i64))
        .same_site(cookie::SameSite::Lax);

    let refresh_cookie = cookie::Cookie::build(("refresh_token", &refresh_token))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(ctx.auth.refresh.exp as i64))
        .same_site(cookie::SameSite::Lax);

    let mut res = Response::builder().status(StatusCode::OK).body(Body::from(
        json!({
            "access_token": &access_token,
            "name": user.name(),
            "created_at": user.created_at().to_string()
        })
        .to_string(),
    ))?;

    res.headers_mut().append(
        AUTHORIZATION,
        HeaderValue::from_str(access_token.as_str()).unwrap(),
    );
    res.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(access_cookie.to_string().as_str()).unwrap(),
    );
    res.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(refresh_cookie.to_string().as_str()).unwrap(),
    );

    Ok(res)
}

#[debug_handler]
async fn current(
    Extension(auth): Extension<crate::models::token::TokenDetails>,
    State(ctx): State<Arc<AppContext>>,
) -> Result<Response> {
    let user = User::find_by_pid(&ctx.db, auth.user_pid).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "name": user.name(),
            "pid": user.pid(),
            "email": user.email()
        })),
    )
        .into_response())
}

async fn logout(
    State(_ctx): State<Arc<AppContext>>,
) -> Result<Response> {
    Ok((StatusCode::OK, Json(json!({"message": "Logged out"}))).into_response())
}

pub fn router(ctx: &Arc<AppContext>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/current", get(current)
            .layer(AuthLayer::new(ctx))
            .layer(RefreshLayer::new(ctx)),
        )
        .route("/logout", post(logout)
            .layer(AuthLayer::new(ctx))
            .layer(RefreshLayer::new(ctx)),
        )
        .with_state(ctx.clone())
}