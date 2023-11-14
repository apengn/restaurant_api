use axum_login::AuthUser;
use diesel::prelude::*;

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, Insertable, Queryable, Selectable, Default,
)]
#[diesel(table_name = crate::schema::wx_openid)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WXOpenid {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
}

impl AuthUser for WXOpenid {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.openid.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.session_key.as_bytes()
    }
}
