-- Your SQL goes here
create table files (
    id serial primary key,
    real_os_path varchar not null,
    submission_path varchar not null
)
