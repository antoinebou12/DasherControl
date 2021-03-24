CREATE TABLE tenants (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  role VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL default now()
)