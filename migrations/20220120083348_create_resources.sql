-- Create op type
-- CREATE TYPE operation AS ENUM ('ADDED', 'UPDATED', 'DELETED')
-- Create resources table
CREATE TABLE resources
(
    history_id BIGSERIAL PRIMARY KEY,
    entity_id UUID NOT NULL,
    resource_id VARCHAR(64) NOT NULL,
    resource_type VARCHAR(32) NOT NULL,
    entry_repo_id UUID,
    version INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    config JSONB NOT NULL,
    -- is_published BOOLEAN,
    last_modified TIMESTAMP NOT NULL,
    last_modified_by VARCHAR(100) NOT NULL,
    message VARCHAR(100) NOT NULL,
    -- op ENUM('ADDED', 'UPDATED', 'DELETED') NOT NULL,
    discarded BOOLEAN,
    CONSTRAINT entity_id_version_unique_constraint UNIQUE (entity_id, version)
);
