use axum_login::AuthUser;
use diesel::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub img: String,
    pub info: String,
    pub phone: String,
    pub location: String,
    #[serde(skip_deserializing)]
    pub hashed_password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Insertable)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub img: String,
    pub info: String,
    pub phone: String,
    pub location: String,
    pub hashed_password: String,
}

impl AuthUser for Restaurant {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.phone.as_bytes()
    }
}
