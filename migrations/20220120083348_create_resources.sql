-- Create resources table
CREATE TABLE resources
(
    history_id UUID NOT NULL PRIMARY KEY,
    entity_id BINARY(16) NOT NULL,
    resource_id VARCHAR(32) NOT NULL,
    resource_type VARCHAR(32) NOT NULL,
    entry_repo_id UUID,
    version INT NOT NULL,
    name VARCHAR(4) NOT NULL,
    config JSON NOT NULL,
    is_published BOOLEAN,
    last_modified DOUBLE NOT NULL,
    last_modified_by VARCHAR(0) NOT NULL,
    message VARCHAR(0) NOT NULL,
    op ENUM('ADDED', 'UPDATED', 'DELETED') NOT NULL,
    discarded BOOLEAN,
    CONSTRAINT resource_version_unique_constraint UNIQUE (resource_id, version)
)
    CHARACTER SET 'utf8mb4'
    COLLATE 'utf8mb4_swedish_ci';
