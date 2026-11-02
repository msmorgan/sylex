---
needs: []
---
# No trailing-underscore names in `.syl`

The trailing underscores exist because the words are Lean keywords. `sylGen` maps a fixed table of 19 Lean-keyword words to
`name_` (wrapped in `«»`) when it emits `.lean`, so nothing in `.syl` needs them even while Lean is the oracle; this
lands before `semantics-rust-crate`, so the Rust crates never see the old names. Semantics-v3 §4.5 already says plain words (`in`,
`from`, `by`, `or`, `this`, …) are written as themselves.

What remains:

- The `at` trigger Template: declared `template at(...)` in `templates/ability/at_.syl`, but called
  `at_(...)` (`definitions/designation/monarch.syl`, `keyword_ability/cumulativeUpkeep.syl`,
  `keyword_ability/echo.syl`, and any card). Rename the file and every call to `at`.
- Comments and `TODO(spelling)` markers naming parameters by their old underscored names
  (`from_`, `for_`, `this_`, `by_`, `while_`, `if_`, `or_`): they now read `from`, `for`, `this`,
  `by`, `while`, `if`, `or`. Fix them to the current names.
- Anything in `deckmaste_dsl` that kept an underscored name only to mirror Lean. Lean's own field
  names (`from_`, `for_`, …) stay until Lean is deleted; `sylGen` maps between them.

Check: `grep -rnE '\b[a-z][A-Za-z0-9]*_\b' plugins_v3` finds nothing, and `cargo xtask dsl fmt
--check` and `cargo xtask lean check` pass.
