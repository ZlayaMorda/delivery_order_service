------------------ ORDERS ------------------

ALTER TABLE orders
    ALTER COLUMN created_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN created_at
        SET NOT NULL,
    ALTER COLUMN updated_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN updated_at
        SET NOT NULL;

CREATE FUNCTION set_orders_timestamp()
RETURNS trigger
AS
  $set_orders_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE orders
    SET    updated_at = current_timestamp
    WHERE  order_uuid = NEW.order_uuid;

  END IF;
  RETURN NULL;
END;
$set_orders_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_orders_timestamp AFTER UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION set_orders_timestamp();

------------------ PRODUCTS ------------------

ALTER TABLE products
    ALTER COLUMN created_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN created_at
        SET NOT NULL,
    ALTER COLUMN updated_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN updated_at
        SET NOT NULL;

CREATE FUNCTION set_products_timestamp()
RETURNS trigger
AS
  $set_products_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE products
    SET    updated_at = current_timestamp
    WHERE  product_uuid = NEW.product_uuid;

  END IF;
  RETURN NULL;
END;
$set_products_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_products_timestamp AFTER UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION set_products_timestamp();

------------------ ORDERS_PRODUCTS ------------------

ALTER TABLE orders_products
    ALTER COLUMN created_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN created_at
        SET NOT NULL,
    ALTER COLUMN updated_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN updated_at
        SET NOT NULL;

CREATE FUNCTION set_orders_products_timestamp()
RETURNS trigger
AS
  $set_orders_products_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE orders_products
    SET    updated_at = current_timestamp
    WHERE  product_uuid = NEW.product_uuid AND order_uuid = NEW.order_uuid;

  END IF;
  RETURN NULL;
END;
$set_orders_products_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_orders_products_timestamp AFTER UPDATE ON orders_products
    FOR EACH ROW EXECUTE FUNCTION set_orders_products_timestamp();

------------------ BUCKETS ------------------

ALTER TABLE buckets
    ALTER COLUMN created_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN created_at
        SET NOT NULL,
    ALTER COLUMN updated_at
        SET DEFAULT current_timestamp,
    ALTER COLUMN updated_at
        SET NOT NULL;

CREATE FUNCTION set_buckets_timestamp()
RETURNS trigger
AS
  $set_buckets_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE buckets
    SET    updated_at = current_timestamp
    WHERE  user_uuid = NEW.user_uuid;

  END IF;
  RETURN NULL;
END;
$set_buckets_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_buckets_timestamp AFTER UPDATE ON buckets
    FOR EACH ROW EXECUTE FUNCTION set_buckets_timestamp();