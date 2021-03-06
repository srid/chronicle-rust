--() { :; }; exec psql -U postgres -v schema=$1 -f "$0"
SET
    TIME ZONE 'UTC';

-- Uncomment this if recreating the database fro mscratch
--
-- DROP SCHEMA IF EXISTS :schema CASCADE;
-- create schema :schema;
select
    1;

create table IF NOT EXISTS :schema.thought (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content text not null,
    created timestamptz DEFAULT now(),
    -- Each thought may have arbitrary properties (eg: "tags" or "is_task").
    -- These are not defined, and are type-checked only by the application in
    -- use. Once the application stabilizes, we could shift that type-checking
    -- to the SQL level for added safety.
    properties jsonb
);

create table IF NOT EXISTS :schema.connection (
    from_ UUID NOT NULL REFERENCES :schema.thought ON DELETE CASCADE,
    to_ UUID NOT NULL REFERENCES :schema.thought ON DELETE CASCADE,
    -- Branching connection means this is a parent->child relationship. i.e.,
    -- `from_` is the parent of `to_`.
    is_branch boolean DEFAULT false,
    PRIMARY KEY(from_, to_)
);

-- Popular some test data for dev server
-- insert into :schema.thought (content, properties)
-- values ('first thought', '{"tags": ["foo"]}'), ('second thought ...', '{}');