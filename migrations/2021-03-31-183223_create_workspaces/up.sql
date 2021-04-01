CREATE TABLE workspaces (
    id SERIAL PRIMARY KEY,
    display_order integer NOT NULL,
    name VARCHAR NOT NULL,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id)
)