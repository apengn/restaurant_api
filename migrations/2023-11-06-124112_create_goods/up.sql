-- Your SQL goes here
CREATE TABLE goods (
  id SERIAL PRIMARY KEY,
  restaurant_id INTEGER NOT NULL REFERENCES restaurants (id),
  goods_type_id INTEGER NOT NULL REFERENCES goods_type (id),
  price DOUBLE PRECISION NOT NULL,
  state BOOLEAN NOT NULL DEFAULT FALSE,
  img TEXT NOT NULL,
  name VARCHAR(120) NOT NULL,
  info TEXT NOT NULL,
  count SERIAL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('goods');