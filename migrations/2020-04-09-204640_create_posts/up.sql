CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE day_data (
  id SERIAL PRIMARY KEY,
  day DATE NOT NULL,
  mood_value TEXT NOT NULL
);