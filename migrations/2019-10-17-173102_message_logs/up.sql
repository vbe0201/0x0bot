CREATE TABLE message_states (
    id BIGINT,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    channel BIGINT NOT NULL,
    author BIGINT NOT NULL,
    content TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id, created_at)
);
