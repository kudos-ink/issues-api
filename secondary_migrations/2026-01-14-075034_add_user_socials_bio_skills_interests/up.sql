-- Your SQL goes here
ALTER TABLE users ADD COLUMN bio TEXT;
ALTER TABLE users ADD COLUMN skills TEXT[];
ALTER TABLE users ADD COLUMN interests TEXT[];
ALTER TABLE users ADD COLUMN telegram TEXT;
ALTER TABLE users ADD COLUMN twitter TEXT;

ALTER TABLE users
  ADD CONSTRAINT users_bio_not_blank
    CHECK (bio IS NULL OR length(btrim(bio)) > 0),
  ADD CONSTRAINT users_telegram_not_blank
    CHECK (telegram IS NULL OR length(btrim(telegram)) > 0),
  ADD CONSTRAINT users_twitter_not_blank
    CHECK (twitter IS NULL OR length(btrim(twitter)) > 0);