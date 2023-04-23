-- Your SQL goes here

create table users (
    id          char(21) primary key,
    email       text unique not null,
    password    text not null,
    created_at  timestamp not null default now(),
    updated_at  timestamp not null default now()
);

SELECT diesel_manage_updated_at('users');

create table repositories (
    id          char(21) primary key,
    user_id     char(21) not null references users(id),
    slug        text unique not null,
    name        text not null,
    description text,
    created_at  timestamp not null default now(),
    updated_at  timestamp not null default now()
);

SELECT diesel_manage_updated_at('repositories');

create table documents (
    id              char(21) primary key,
    repository_id   char(21) not null references repositories(id),
    slug            text unique not null,
    name            text not null,
    description     text,
    created_at      timestamp not null default now(),
    updated_at      timestamp not null default now()
);

SELECT diesel_manage_updated_at('documents');

create table blocks (
    id              char(21) primary key,
    document_id     char(21) not null references documents(id),
    line_number     integer not null,
    created_at      timestamp not null default now(),
    unique (document_id, line_number)
);

create type tag as enum ('H1', 'H2', 'H3', 'P');

create table text_blocks (
    block_id        char(21) primary key references blocks(id),
    tag             tag not null,
    content         text,
    updated_at      timestamp not null default now()
);

SELECT diesel_manage_updated_at('text_blocks');

create table image_blocks (
    block_id        char(21) primary key references blocks(id),
    url             text,
    updated_at      timestamp not null default now()
);

SELECT diesel_manage_updated_at('image_blocks');

