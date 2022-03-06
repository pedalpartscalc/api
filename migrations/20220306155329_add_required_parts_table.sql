-- Add migration script here
create table required_parts (
    id bigserial primary key,
    pedal_id bigint not null,
    part_name varchar not null,
    part_kind varchar not null,
    quantity integer not null
);