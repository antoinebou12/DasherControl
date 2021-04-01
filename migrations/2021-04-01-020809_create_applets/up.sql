CREATE TABLE applets (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    position_x integer NOT NULL,
    position_y integer NOT NULL,
    width integer NOT NULL,
    height integer NOT NULL,
    editable boolean NOT NULL,
    applet_data varchar,
    workspace_id integer NOT NULL REFERENCES workspaces(id)
)