-- Add migration script here

CREATE EXTENSION "uuid-ossp";

CREATE TABLE orders (
    order_uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_uuid UUID NOT NULL,
    courier_uuid UUID NOT NULL,
    rating FLOAT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (order_uuid)
);

CREATE TABLE products (
    product_uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
    product_type VARCHAR(128) NOT NULL,
    product_name VARCHAR(128) NOT NULL,
    restaurant VARCHAR(128) NOT NULL,
    price FLOAT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (product_uuid)
);

CREATE TABLE orders_products (
    order_uuid UUID NOT NULL,
    product_uuid UUID NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (order_uuid) REFERENCES orders (order_uuid) ON DELETE CASCADE,
    FOREIGN KEY (product_uuid) REFERENCES products (product_uuid) ON DELETE CASCADE,
    PRIMARY KEY (order_uuid, product_uuid)
);

CREATE TABLE buckets (
    user_uuid UUID NOT NULL UNIQUE,
    product_uuid UUID NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (product_uuid) REFERENCES products (product_uuid) ON DELETE CASCADE,
    PRIMARY KEY (user_uuid, product_uuid)
);