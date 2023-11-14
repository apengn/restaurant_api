use std::fmt::Debug;

use axum::async_trait;
use axum_login::{AuthnBackend, UserId};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{db::Pool, error::Error, model::wx_user::WXOpenid, schema};

pub mod order;
pub mod order_detail;

pub mod user;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct WxLoginCode {
    pub js_code: String,
}

// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
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
    type Credentials = WxLoginCode;
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

        let mut conn = self.pool.get().await.map_err(Error::new)?;

        use schema::wx_openid::dsl::*;

        use diesel::dsl::exists;
        use diesel::select;
        let wx_exists = select(exists(wx_openid.filter(openid.eq(resp.openid.clone()))))
            .get_result(&mut conn)
            .await
            .map_err(Error::new)?;

        let id_v: i32;
        if wx_exists {
            id_v = diesel::update(wx_openid.filter(openid.eq(resp.openid.clone())))
                .set(session_key.eq(resp.session_key.clone()))
                .returning(id)
                .get_result::<i32>(&mut conn)
                .await
                .map_err(Error::new)?;
        } else {
            id_v = diesel::insert_into(wx_openid)
                .values((
                    session_key.eq(resp.session_key.clone()),
                    openid.eq(resp.openid.clone()),
                ))
                .returning(id)
                .get_result::<i32>(&mut conn)
                .await
                .map_err(Error::new)?;
        }
        return Ok(Some(WXOpenid {
            openid: resp.openid,
            session_key: resp.session_key,
            id: id_v,
        }));
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

// TODO 需要写进配置文件
fn build_jscode2session_url(jscode2session_url: &mut String) {
    jscode2session_url.push_str("?");

    jscode2session_url.push_str("appid=");
    jscode2session_url.push_str("");
    jscode2session_url.push_str("&");

    jscode2session_url.push_str("secret=");
    jscode2session_url.push_str("");
    jscode2session_url.push_str("&");

    jscode2session_url.push_str("grant_type=");
    jscode2session_url.push_str("authorization_code");
    jscode2session_url.push_str("&");
}
