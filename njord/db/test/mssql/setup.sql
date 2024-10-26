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
    -- Table: users
    CREATE TABLE users (
        id INT IDENTITY(1, 1) PRIMARY KEY,
        -- Auto incrementing primary key for the user ID
        username VARCHAR(255) NOT NULL,
        -- Username field
        email VARCHAR(255) NOT NULL,
        -- Email field
        address VARCHAR(255) -- Address field
    );

-- Table: categories
CREATE TABLE categories (
    id INT PRIMARY KEY,
    -- Primary key for categories
    name VARCHAR(255) NOT NULL -- Name of the category
);

-- Table: products
CREATE TABLE products (
    id INT PRIMARY KEY,
    -- Primary key for products
    name VARCHAR(255) NOT NULL,
    -- Product name
    description TEXT,
    -- Product description (using TEXT for large text)
    price DECIMAL(10, 2) NOT NULL,
    -- Price with up to two decimal places
    stock_quantity INT NOT NULL,
    -- Stock quantity
    category_id INT NOT NULL,
    -- Foreign key to categories (one-to-one relationship)
    discount DECIMAL(5, 2) DEFAULT 0.00,
);