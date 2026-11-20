# lib-hello

`hello` — Handl stdlib port: the `greet` arrow.

## Layout

```text
src/index.hnl   the arrow (pure handl)
src/lib.rs      `GREET_HNL` (bare) + `HELLO_HNL` (module record)
pkg.hnl         package manifest — modules, deps, description, license
ci.sh           bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
