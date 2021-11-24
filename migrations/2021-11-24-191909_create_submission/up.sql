-- Your SQL goes here
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

)
