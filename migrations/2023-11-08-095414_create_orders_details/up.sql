-- Your SQL goes here
CREATE TABLE orders_details(
   id SERIAL PRIMARY KEY,
   order_id INTEGER NOT NULL REFERENCES orders (id),
   count INTEGER NOT NULL,
   goods_name TEXT NOT NULL,
   info TEXT NOT NULL,
   price DOUBLE PRECISION NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('orders_details');