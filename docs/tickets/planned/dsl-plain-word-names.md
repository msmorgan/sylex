---
needs: [semantics-rust-rewrite]
---
# No trailing-underscore names in `.syl`

The trailing underscores exist because the words are Lean keywords; with Lean deleted by
`semantics-rust-rewrite` nothing needs them. Semantics-v3 §4.5 already says plain words (`in`,
`from`, `by`, `or`, `this`, …) are written as themselves.

What remains:

- The `at` trigger Template: declared `template at(...)` in `templates/ability/at_.syl`, but called
  `at_(...)` (`definitions/designation/monarch.syl`, `keyword_ability/cumulativeUpkeep.syl`,
  `keyword_ability/echo.syl`, and any card). Rename the file and every call to `at`.
- Comments and `TODO(spelling)` markers naming parameters by their old underscored names
  (`from_`, `for_`, `this_`, `by_`, `while_`, `if_`, `or_`): they now read `from`, `for`, `this`,
  `by`, `while`, `if`, `or`. Fix them to the current names.
- Anything in the Rust `semantics` AST or `deckmaste_dsl` that kept an underscored field or
  constructor name only to mirror Lean.

Check: `grep -rnE '\b[a-z][A-Za-z0-9]*_\b' plugins_v3` finds nothing but words that are not names
(none expected), and `cargo xtask dsl fmt --check` and the `semantics` tests pass.
