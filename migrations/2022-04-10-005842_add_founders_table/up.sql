-- Your SQL goes here
CREATE TABLE founders (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  company_name TEXT NOT NULL,
  bio TEXT NOT NULL,
  image TEXT NOT NULL
);