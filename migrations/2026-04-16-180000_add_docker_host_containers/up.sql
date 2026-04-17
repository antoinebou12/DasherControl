CREATE TABLE docker_host_containers (
    docker_id VARCHAR(128) PRIMARY KEY,
    image TEXT NOT NULL DEFAULT '',
    state VARCHAR(64),
    status TEXT,
    names JSONB NOT NULL,
    compose_project VARCHAR(255),
    compose_service VARCHAR(255),
    labels JSONB,
    synced_at TIMESTAMP NOT NULL DEFAULT NOW()
);
