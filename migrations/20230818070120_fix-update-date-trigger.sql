-- DROP uncorrected functions and triggers for updated_at
DROP TRIGGER set_orders_timestamp ON orders;

DROP FUNCTION set_orders_timestamp;

DROP TRIGGER set_products_timestamp ON products;

DROP FUNCTION set_products_timestamp;

DROP TRIGGER set_orders_products_timestamp ON orders_products;

DROP FUNCTION set_orders_products_timestamp;

DROP TRIGGER set_buckets_timestamp ON buckets;

DROP FUNCTION set_buckets_timestamp;

-- CREATE new function and triggers

CREATE OR REPLACE FUNCTION set_update_timestamp()
RETURNS TRIGGER
AS
  $set_users_timestamp$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$set_users_timestamp$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER set_orders_update_timestamp BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION set_update_timestamp();

CREATE OR REPLACE TRIGGER set_products_update_timestamp BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION set_update_timestamp();

CREATE OR REPLACE TRIGGER set_orders_products_update_timestamp BEFORE UPDATE ON orders_products
    FOR EACH ROW EXECUTE FUNCTION set_update_timestamp();

CREATE OR REPLACE TRIGGER set_buckets_update_timestamp BEFORE UPDATE ON buckets
    FOR EACH ROW EXECUTE FUNCTION set_update_timestamp();