-- Your SQL goes here
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

create table users (
    id          char(21) primary key,
    email       text unique not null,
    password    text not null,
    created_at  timestamp not null default now(),
    updated_at  timestamp not null default now()
);

create trigger set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

