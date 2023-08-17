-- pending delivering received
ALTER TABLE orders
    ADD COLUMN status VARCHAR(16) DEFAULT 'pending';