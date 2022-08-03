create database if not exists `token` charset = 'utf8';
use `token`;

create table if not exists users (
    id bigint auto_increment primary key,
    username varchar(255) not null,
    password varchar(255) not null
) charset = utf8mb4;
