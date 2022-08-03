create database if not exists `token` charset = 'utf8';
use `token`;

create table if not exists points (
    id bigint auto_increment primary key,
    parent char(42) not null,
    owner char(42) not null,
    point int not null,
    create_time int(11) not null,
    update_time int(11) not null
) charset = utf8mb4;