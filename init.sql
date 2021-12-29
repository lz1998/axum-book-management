CREATE DATABASE test;
use test;

CREATE TABLE `user`
(
    `username` varchar(255) NOT NULL PRIMARY KEY COMMENT 'username',
    `password` varchar(255) NOT NULL COMMENT 'password'
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='user';

CREATE TABLE `book`
(
    `id`         integer      NOT NULL PRIMARY KEY AUTO_INCREMENT COMMENT 'id',
    `name`       varchar(255) NOT NULL COMMENT 'name',
    `operator`   varchar(255) NOT NULL COMMENT 'operator',
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time'
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='book';