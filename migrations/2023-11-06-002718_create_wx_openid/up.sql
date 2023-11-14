-- Your SQL goes here
CREATE TABLE wx_openid(
  id SERIAL PRIMARY KEY,
  
  openid VARCHAR(120) NOT NULL,
  session_key VARCHAR(120) NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('wx_openid');