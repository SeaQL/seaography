CREATE TYPE PERMISSION AS ENUM ('read', 'write', 'admin');

CREATE TABLE accounts (
    id             UUID NOT NULL PRIMARY KEY,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    deleted_at     TIMESTAMP WITH TIME ZONE,
    name           TEXT NOT NULL,
    email          TEXT NOT NULL
);

CREATE TABLE projects (
    id             UUID NOT NULL PRIMARY KEY,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    deleted_at     TIMESTAMP WITH TIME ZONE,
    name           TEXT NOT NULL
);

CREATE TABLE drawings (
    id             UUID NOT NULL PRIMARY KEY,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    deleted_at     TIMESTAMP WITH TIME ZONE,
    project_id     UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name           TEXT NOT NULL,
    width          BIGINT NOT NULL,
    height         BIGINT NOT NULL
);

CREATE TABLE objects (
    id             UUID NOT NULL PRIMARY KEY,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    deleted_at     TIMESTAMP WITH TIME ZONE,
    project_id     UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    drawing_id     UUID NOT NULL REFERENCES drawings(id) ON DELETE CASCADE,
    fill           JSONB NOT NULL,
    stroke         JSONB NOT NULL,
    shape          JSONB NOT NULL
);

CREATE TABLE project_permissions (
    project_id     UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    account_id     UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    permission     PERMISSION NOT NULL,
    PRIMARY KEY (project_id, account_id)
);

INSERT INTO accounts (id, created_at, updated_at, name, email) VALUES (
    'b5a6d582-322a-459e-ba2a-28d51a4e63ec',
    now(),
    now(),
    'root',
    'root@example.com'
);
