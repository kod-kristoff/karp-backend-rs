-- Create resources table
CREATE TABLE resources
(
    history_id INTEGER NOT NULL PRIMARY KEY,
    entity_id UUID NOT NULL,
    resource_id VARCHAR(32) NOT NULL,
    resource_type VARCHAR(32) NOT NULL,
    entry_repo_id UUID,
    version INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    config JSON NOT NULL,
    is_published BOOLEAN,
    last_modified DOUBLE NOT NULL,
    last_modified_by VARCHAR(100) NOT NULL,
    message VARCHAR(200) NOT NULL,
    op ENUM('ADDED', 'UPDATED', 'DELETED') NOT NULL,
    discarded BOOLEAN,
    CONSTRAINT entity_id_version_unique_constraint UNIQUE (entity_id, version)
);
