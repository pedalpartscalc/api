-- Your SQL goes here
create table available_parts (
  id bigserial primary key,
  owner_id bigint not null,
  part_name varchar not null,
  part_kind varchar not null,
  quantity integer not null
);