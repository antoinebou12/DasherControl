CREATE TABLE auth_infos (
  id SERIAL PRIMARY KEY,
  tenant_id INTEGER NOT NULL,
  password_hash TEXT NOT NULL
)