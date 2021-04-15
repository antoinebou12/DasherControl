CREATE TABLE login_history
(
    id SERIAL PRIMARY KEY NOT NULL,
    tenant_id integer NOT NULL REFERENCES tenants(id),
    login_timestamp TIMESTAMP NOT NULL default now()
);