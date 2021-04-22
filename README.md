A [Yew](https://yew.rs/) / [PostgREST](https://postgrest.org/en/stable/) app to explore thought-curation, partly based on the [2015 idea](https://github.com/srid/chronicle-2015).

**Update:** I've switched to [memoir](https://github.com/srid/memoir)

```
# Import schema.sql into postgres,
./schema.sql chronicle

# Setup postgrest,
# https://github.com/srid/nixos-config/blob/master/features/postgrest.nix

# Then run chronicle dev server,
bin/run  # Or, Ctrl+Shift+B in VSCode
```

## WHITEBOARD

- DONE Connect to local postgrest and get some info to display on frontend
- DONE postgrest: design schema
- DONE web: render a basic timeline of thoughts
- TODO web: add thoughts (forms)
- TODO web: connect thoughts, as parent/child
- TODO jwt login
- TODO deploy
