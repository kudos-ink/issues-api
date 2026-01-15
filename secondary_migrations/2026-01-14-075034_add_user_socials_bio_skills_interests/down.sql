-- This file should undo anything in `up.sql`
ALTER TABLE users
  DROP CONSTRAINT IF EXISTS users_bio_not_blank,
  DROP CONSTRAINT IF EXISTS users_telegram_not_blank,
  DROP CONSTRAINT IF EXISTS users_twitter_not_blank;

ALTER TABLE users
  DROP COLUMN IF EXISTS twitter,
  DROP COLUMN IF EXISTS telegram,
  DROP COLUMN IF EXISTS interests,
  DROP COLUMN IF EXISTS skills,
  DROP COLUMN IF EXISTS bio;
