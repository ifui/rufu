CREATE TABLE IF NOT EXISTS admin_users
(
    id CHAR(36) PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(511) NOT NULL,
    nickname VARCHAR(255),
    avatar VARCHAR(1000),
    sex CHAR(1),
    email VARCHAR(255),
    phone VARCHAR(20)
);
