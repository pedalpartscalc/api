-- Add migration script here
create table users (
    id bigserial primary key,
    auth_zero_id varchar
);