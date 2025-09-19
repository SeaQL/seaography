CREATE TABLE accounts (
    id             TEXT NOT NULL PRIMARY KEY,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    deleted_at     TEXT,
    name           TEXT NOT NULL,
    email          TEXT NOT NULL
);

CREATE TABLE projects (
    id             TEXT NOT NULL PRIMARY KEY,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    deleted_at     TEXT,
    name           TEXT NOT NULL
);

CREATE TABLE drawings (
    id             TEXT NOT NULL PRIMARY KEY,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    deleted_at     TEXT,
    project_id     TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name           TEXT NOT NULL,
    width          INTEGER NOT NULL,
    height         INTEGER NOT NULL
);

CREATE TABLE objects (
    id             TEXT NOT NULL PRIMARY KEY,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    deleted_at     TEXT,
    project_id     TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    drawing_id     TEXT NOT NULL REFERENCES drawings(id) ON DELETE CASCADE,
    fill           JSONB NOT NULL,
    stroke         JSONB NOT NULL,
    shape          JSONB NOT NULL
);
