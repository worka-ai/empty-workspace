CREATE TABLE IF NOT EXISTS event (
    event_id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    received_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task (
    task_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    assignee_agent_id TEXT,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_entry (
    audit_id TEXT PRIMARY KEY,
    event_id TEXT,
    agent_id TEXT,
    tool_name TEXT,
    result_json TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS attention_request (
    attention_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    prompt_json TEXT NOT NULL,
    created_at TEXT NOT NULL
);
