-- Your SQL goes here
create table challenges (
    id serial primary key,
    name varchar not null,
    description text not null,
    owner_id int,

    CONSTRAINT fk_users
        FOREIGN KEY(owner_id)
            REFERENCES users(id)
            ON DELETE SET NULL
)
