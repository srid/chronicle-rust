SET
    TIME ZONE 'UTC';

DROP SCHEMA IF EXISTS chronicle CASCADE;

create schema chronicle;

create table chronicle.thought (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content text not null,
    created timestamptz DEFAULT now(),
    -- Each thought may have arbitrary properties (eg: "tags" or "is_task").
    -- These are not defined, and are type-checked only by the application in
    -- use. Once the application stabilizes, we could shift that type-checking
    -- to the SQL level for added safety.
    properties jsonb
);

create table chronicle.connection (
    from_ UUID NOT NULL REFERENCES chronicle.thought ON DELETE CASCADE,
    to_ UUID NOT NULL REFERENCES chronicle.thought ON DELETE CASCADE,
    -- Branching connection means this is a parent->child relationship. i.e.,
    -- `from_` is the parent of `to_`.
    is_branch boolean DEFAULT false,
    PRIMARY KEY(from_, to_)
);

insert into
    chronicle.thought (content, properties)
values
    ('first thought', '{"tags": ["foo"]}'),
    ('second thought', '{}');

select
    *
from
    chronicle.thought;

select
    *
from
    chronicle.connection;