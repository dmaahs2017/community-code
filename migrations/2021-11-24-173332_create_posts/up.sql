-- Your SQL goes here
CREATE Table posts (
    id serial primary key,
    title varchar not null,
    body text not null,
    published boolean not null default 'f'
)
