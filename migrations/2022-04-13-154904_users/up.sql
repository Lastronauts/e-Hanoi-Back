-- Your SQL goes here

create table users(
    id varchar(255) not null primary key,
    name varchar(255) not null
);

create table scores(
    id serial not null primary key,
    user_id varchar(255) references users (id) not null,
    clear_time int not null ,
    created_at timestamp with time zone not null default current_timestamp
);
