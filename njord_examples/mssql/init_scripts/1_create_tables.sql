USE [master];
GO

IF NOT EXISTS (SELECT * FROM sys.sql_logins WHERE name = 'njord_user')
BEGIN
    CREATE LOGIN [njord_user] WITH PASSWORD = 'njord_password', CHECK_POLICY = OFF;
    ALTER SERVER ROLE [sysadmin] ADD MEMBER [njord_user];
END
GO

CREATE TABLE njord_user.neo (
    id INT IDENTITY(1,1) PRIMARY KEY,
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
