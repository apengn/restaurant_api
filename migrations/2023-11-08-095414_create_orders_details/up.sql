-- Your SQL goes here
CREATE TABLE orders_details(
  id SERIAL PRIMARY KEY,
  order_id INTEGER NOT NULL REFERENCES orders (id),
  good_id INTEGER NOT NULL REFERENCES goods (id),
  user_id INTEGER NOT NULL REFERENCES users (id),
  wx_open_id INTEGER NOT NULL REFERENCES wx_openid (id),
  count INTEGER,
  info TEXT NOT NULL, 
  price DOUBLE PRECISION NOT NULL,
  goods_name TEXT NOT NULL, 
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('orders_details');