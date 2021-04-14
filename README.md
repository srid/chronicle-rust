A [Yew](https://yew.rs/) / [PostgREST](https://postgrest.org/en/stable/) app to explore thought-curation, partly based on the [2015 idea](https://github.com/srid/chronicle-2015).

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
- TODO web: render a basic timeline of thoughts
- TODO jwt login
- TODO deploy