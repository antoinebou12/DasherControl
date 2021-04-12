CREATE TABLE containers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id)
)