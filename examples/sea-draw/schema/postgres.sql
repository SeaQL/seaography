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
