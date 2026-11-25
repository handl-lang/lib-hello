# lib-hello

`hello` — Handl stdlib port: the `greet` arrow.

The package surface is pure `.hnl`: consumers vendor `pkg.hnl` +
`src/**/*.hnl` via `handl-pkg-vendor` and embed the sources with
`include_str!` at their own build time (`plugin-lib-stdlib` does this —
the `handl-lib-hello` crate's `GREET_HNL`/`HELLO_HNL` consts are no
longer consumed).

`Cargo.toml` + `src/lib.rs` are retained **only** as a manifest shim:
cargo eagerly validates every `[patch]` path entry (even unused ones),
so the manifest must keep satisfying `handl-lib-hello` patches across
the workspace repos until the org-wide patch sweep removes them. They
will be deleted together with those patch entries.

## Layout

```text
src/index.hnl   the arrow (pure handl)
src/lib.rs      const shim — pending patch sweep (see above)
pkg.hnl         package manifest — modules, deps, description, license
workflows/gate.yaml           bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
