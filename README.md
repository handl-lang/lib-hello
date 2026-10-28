# lib-hello

`hello` — Handl stdlib port: the `greet` arrow.

Part of the stdlib → `lib-*` split (ADR-0128). History is a `git subtree split`
of `stdlib/handl-packages/hello/`.

## Layout

```text
src/index.hnl   the arrow (pure handl)
src/lib.rs      `GREET_HNL` (bare) + `HELLO_HNL` (module record)
pkg.hnl         { modules: ["hello"] }
ci.sh           bupha entry
```

## License

MIT.
