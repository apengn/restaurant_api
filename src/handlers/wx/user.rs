use axum::{extract::Query, http::StatusCode, response::IntoResponse};

use crate::handlers::wx::{WXAuthSession, WxLoginCode};

pub async fn wx_request(
    mut auth_session: WXAuthSession,
    wx_login_code: Option<Query<WxLoginCode>>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(wx_login_code.unwrap().0).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    StatusCode::OK.into_response()
}

async fn get_wx_access_token() {
    let mut token_url = "https://api.weixin.qq.com/cgi-bin/token".to_string();
    token_url.push('?');

    token_url.push_str("appid=");
    token_url.push_str("");
    token_url.push('&');

    token_url.push_str("secret=");
    token_url.push_str("");
    token_url.push('&');

    token_url.push_str("grant_type=");
    token_url.push_str("client_credential");

    // let resp = reqwest::get(token_url.as_ref())
    //     .await.unwrap()
    //     .json::<HashMap<String, String>>()
    //     .await;

    println!("{:#?}", token_url);
}

pub async fn logout(mut auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.logout() {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn protected(auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
