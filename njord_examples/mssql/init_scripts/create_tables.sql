-- Step 1: Create a new database (if not already created)
USE [master];

GO
    IF NOT EXISTS (
        SELECT
            name
        FROM
            sys.databases
        WHERE
            name = 'NjordDatabase'
    ) BEGIN CREATE DATABASE NjordDatabase;

END
GO
    -- Step 3: Use the created database
    USE NjordDatabase;

GO
    -- Step 4: Create the table in the new database
    CREATE TABLE neo (
        id INT IDENTITY(1, 1) PRIMARY KEY,
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

GO