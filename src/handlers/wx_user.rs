use std::fmt::Debug;

use axum::{async_trait, extract::Query, http::StatusCode, response::IntoResponse};
use axum_login::{AuthnBackend, UserId};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{db::Pool, error::Error, model::wx_user::WXOpenid, schema};

// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html

#[derive(serde::Deserialize, serde::Serialize)]
struct Code2SessionQuery {
    pub appid: String,
    pub secret: String,
    pub js_code: String,
    pub grant_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct WxLoingCode {
    pub js_code: String,
}

// "openid":"xxxxxx",
// "session_key":"xxxxx",
// "unionid":"xxxxx",
// "errcode":0,
// "errmsg":"xxxxx"
#[derive(serde::Deserialize, serde::Serialize)]
struct Code2SessionResponse {
    pub openid: String,
    pub session_key: String,
    pub unionid: String,
    pub errcode: i32,
    pub errmsg: String,
}

#[derive(Debug, Clone)]
pub struct WXBackend {
    pool: Pool,
}

impl WXBackend {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthnBackend for WXBackend {
    type User = WXOpenid;
    type Credentials = WxLoingCode;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let mut jscode2session_url = "https://api.weixin.qq.com/sns/jscode2session".to_string();

        build_jscode2session_url(&mut jscode2session_url);
        jscode2session_url.push_str("js_code=");
        jscode2session_url.push_str(creds.js_code.as_str());

        let resp = reqwest::get(jscode2session_url.as_str())
            .await
            .map_err(Error::new)?
            .json::<Code2SessionResponse>()
            .await
            .map_err(Error::new)?;

        if resp.errcode != 0 {
            tracing::error!("wx jscode2session {}", resp.errmsg);
            return Ok(None);
        }
        let new_wx_openid = WXOpenid {
            id: 0i32,
            openid: resp.openid,
            session_key: resp.session_key,
        };
        let mut conn = self.pool.get().await.map_err(Error::new)?;

        use schema::wx_openid::dsl::*;

        use diesel::dsl::exists;
        use diesel::select;
        let wx_exists = select(exists(
            wx_openid.filter(openid.eq(new_wx_openid.openid.clone())),
        ))
        .get_result(&mut conn)
        .await
        .map_err(Error::new)?;

        if wx_exists {
            diesel::update(wx_openid.filter(openid.eq(new_wx_openid.openid.clone())))
                .set(session_key.eq(new_wx_openid.session_key.clone()))
                .execute(&mut conn)
                .await
                .map_err(Error::new)?;
        } else {
            diesel::insert_into(wx_openid)
                .values(&new_wx_openid)
                .execute(&mut conn)
                .await
                .map_err(Error::new)?;
        }

        return Ok(Some(new_wx_openid));
    }

    async fn get_user(&self, open_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut conn = self.pool.get().await.map_err(Error::new)?;

        use schema::wx_openid::dsl::*;
        let wx_openid_v = wx_openid
            .filter(openid.eq(open_id))
            .select(WXOpenid::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map_err(Error::new)?;

        Ok(wx_openid_v)
    }
}

type WXAuthSession = axum_login::AuthSession<WXBackend>;

pub async fn wx_request(
    mut auth_session: WXAuthSession,
    wx_login_code: Option<Query<WxLoingCode>>,
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

// TODO 需要写进配置文件
fn build_jscode2session_url(jscode2session_url: &mut String) {
    jscode2session_url.push_str("?");

    jscode2session_url.push_str("appid=");
    jscode2session_url.push_str("appid");
    jscode2session_url.push_str("&");

    jscode2session_url.push_str("secret=");
    jscode2session_url.push_str("secret");
    jscode2session_url.push_str("&");

    jscode2session_url.push_str("grant_type=");
    jscode2session_url.push_str("authorization_code");
    jscode2session_url.push_str("&");
}

async fn get_wx_access_token() {
    let mut token_url = "https://api.weixin.qq.com/cgi-bin/token".to_string();
    token_url.push_str("?");

    token_url.push_str("appid=");
    token_url.push_str("appid");
    token_url.push_str("&");

    token_url.push_str("secret=");
    token_url.push_str("secret");
    token_url.push_str("&");

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
