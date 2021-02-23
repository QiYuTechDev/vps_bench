CREATE TABLE s_user
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name  TEXT UNIQUE                       NOT NULL,
    value TEXT                              NOT NULL,
    age   INTEGER                           NOT NULL
);
