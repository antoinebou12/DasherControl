CREATE TABLE tenant_configuration
(
    id SERIAL PRIMARY KEY NOT NULL,
    tenant_id integer NOT NULL REFERENCES tenants(id),
    config varchar DEFAULT '{}' NOT NULL
);