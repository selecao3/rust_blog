-- Your SQL goes here
CREATE TABLE Blog
(id INTEGER,
title VARCHAR(40) NOT NULL,
published DATETIME NOT NULL,
body MEDIUMTEXT NOT NULL,
completed BOOLEAN NOT NULL
);