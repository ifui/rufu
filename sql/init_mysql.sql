CREATE TABLE IF NOT EXISTS admin_users
(
    id CHAR(10) PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(511) NOT NULL,
    nickname VARCHAR(255),
    avatar VARCHAR(1000),
    sex CHAR(1),
    email VARCHAR(255),
    phone VARCHAR(20),
    status CHAR(1),
    updated_at DATETIME,
    created_at DATETIME DEFAULT NOW()
);