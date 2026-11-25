# lib-hello

`hello` — the smallest stdlib port: the `greet` arrow. Used as the
canonical example of the pure-`.hnl` package shape.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the arrow (pure handl) |
| `pkg.hnl` | manifest — `modules: ["hello"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Pure `.hnl` only — consumers vendor `pkg.hnl` + `src/**/*.hnl` via
  `handl-pkg-vendor` and embed with `include_str!`
  (`plugin-lib-stdlib` does this).
- Trivial by design; keep it a clean reference, not a dumping ground.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
