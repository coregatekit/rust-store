-- Your SQL goes here
INSERT INTO products (name, description, image_url, price, created_at, updated_at)
SELECT
    'Product ' || seq AS name,
    CASE (seq % 5)
        WHEN 0 THEN 'A durable everyday item for home use.'
        WHEN 1 THEN 'Lightweight and compact, perfect for travel.'
        WHEN 2 THEN 'Premium quality with a modern design.'
        WHEN 3 THEN 'Eco-friendly and sustainably sourced.'
        ELSE        'Best-seller with thousands of happy customers.'
    END                                                          AS description,
    'https://example.com/images/product_' || seq || '.jpg' AS image_url,
    ROUND((RANDOM() * 490 + 10)::numeric, 2) AS price,
    NOW() - (RANDOM() * INTERVAL '365 days') AS created_at,
    NOW() - (RANDOM() * INTERVAL '30 days') AS updated_at
FROM generate_series(1, 500) AS seq;
