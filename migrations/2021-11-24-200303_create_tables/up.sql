-- Your SQL goes here
create table users (
    id SERIAL PRIMARY KEY,
    name VARCHAR not null unique
);

create table challenges (
    id serial primary key,
    name varchar not null,
    description text not null,
    owner_id int,

    CONSTRAINT fk_users
        FOREIGN KEY(owner_id)
            REFERENCES users(id)
            ON DELETE SET NULL
);

create table files (
    id serial primary key,
    real_os_path varchar not null,
    submission_path varchar not null
);

create table submissions (
    id serial primary key,
    challenge_id int not null,
    user_id int,

    CONSTRAINT fk_challenge
        FOREIGN KEY(challenge_id)
            REFERENCES challenges(id)
            ON DELETE CASCADE,

    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE set null

);
