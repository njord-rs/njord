CREATE TABLE IF NOT EXISTS neo (
    id INT AUTO_INCREMENT PRIMARY KEY,
    neo_id VARCHAR(255) NOT NULL,
    neo_reference_id VARCHAR(255),
    name VARCHAR(255),
    name_limited VARCHAR(255),
    designation VARCHAR(255),
    nasa_jpl_url VARCHAR(255),
    absolute_magnitude_h FLOAT,
    is_potentially_hazardous_asteroid VARCHAR(8),
    is_sentry_object VARCHAR(8)
);