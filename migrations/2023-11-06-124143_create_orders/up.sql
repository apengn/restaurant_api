-- Your SQL goes here

CREATE TABLE orders(
  id SERIAL PRIMARY KEY,
  uuid TEXT NOT NULL,
  restaurant_id INTEGER NOT NULL REFERENCES restaurants (id),
  user_id INTEGER NOT NULL REFERENCES users (id),
  wx_open_id INTEGER NOT NULL REFERENCES wx_openid (id),
  qrcode_location_id INTEGER NOT NULL REFERENCES qrcode_location (id),
  state TEXT NOT NULL,
  total_cost DOUBLE PRECISION NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('orders');