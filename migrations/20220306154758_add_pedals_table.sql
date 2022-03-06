-- Add migration script here
-- create trigger to set the updated_at field
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

create table pedals (
    id bigserial primary key,
    name varchar NOT NULL,
    kind varchar NOT NULL, -- like 'overdrive' or 'delay'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);