-- This file should undo anything in `up.sql`
DELETE FROM products WHERE id >= 1;
ALTER SEQUENCE products_id_seq RESTART WITH 1;