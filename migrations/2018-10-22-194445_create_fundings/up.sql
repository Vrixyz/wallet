CREATE TABLE fundings (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  amount INTEGER NOT NULL,
  UNIQUE(user_id)
)