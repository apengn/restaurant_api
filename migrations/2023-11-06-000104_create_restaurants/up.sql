-- Your SQL goes here
CREATE TABLE restaurants(
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  img TEXT NOT NULL,
  info TEXT NOT NULL,
  phone VARCHAR(11) UNIQUE NOT NULL,
  location TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('restaurants');