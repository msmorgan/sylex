---
needs: [dsl-parser, dsl-parse-differential]
---
# `.syl` parameter reads are written `$name`

Standard constraints apply. In a declaration's body, a read of one of its parameters is written
with a `$`: `exileWithCounters(thisCard, $amount, timeCounter)`. Right now a bare word that names
a parameter means that parameter (`docs/decisions/semantics-v3.md` §4.5 item 2, step 1). The aim
is that parameters stand out, and that what a word refers to can be read from its spelling.

## Decisions already made

- **`$` means "the value of".** It marks a read of a parameter's value and nothing else.
  Parameter declarations in headers (`ability suspend(amount: Amount, cost: Cost)`), argument
  labels (`play(thisCard, payment = withoutPaying)`) and record field names stay plain.
- `$name` is a sixth value form: a parameter reference. It takes no arguments, so `$name(...)` is
  a parse error, as are a lone `$` and a space between `$` and the name. The part after `$`
  follows the word rule.
- **Resolution:** `$name` resolves only to a parameter of the enclosing declaration, at its
  declared sort, or at `U` when the expected sort is `Option U`. Anything else is a load error:
  an unknown parameter, or a parameter used at the wrong sort. A bare word never resolves to a
  parameter. So step 1 of §4.5 item 2 becomes "`$name` is a parameter", and a parameter no longer
  hides a word with the same name.
- `spelling` template holes (`"suspend <amount>—<cost>"`) are strings and are out of scope here;
  they belong to the spelling work.

## Change

- **ADR:** amend `docs/decisions/semantics-v3.md` §4.4 (abstract grammar, concrete syntax) and
  §4.5 item 2 (step 1, and the "bare word naming a parameter at another sort" load error, which
  becomes the `$name` sort error).
- **`deckmaste_dsl`:** a `$name` token and a parameter-reference value, with tests.
- **SylGen:** parse `$name` and resolve it as above. Bare words no longer look up parameters.
  Add negative cases in `lean/SylGen/Tests/negative.sh`: unknown `$name`, `$name` at the wrong
  sort, `$name(...)`, a lone `$`, and a bare word that used to name a parameter now resolving as
  an ordinary word, or failing as one.
- **Differential:** extend `ParseDump` and the `cargo xtask dsl differential` projection to cover
  the new value.
- **Corpus rewrite**, in its own commit. In every value that SylGen resolves with the parameter
  map (find these in `lean/SylGen/Resolve.lean`: `body`, `confers`, `rules`, defaults; not
  `lexical` or `spelling`, which are discarded), rewrite each bare zero-argument word that names
  a parameter of that declaration to `$word`. The rewrite never changes meaning, because today
  such a word always means the parameter or is a load error.

## Acceptance

- The Lean modules generated under `lean/.lake/build/gen/` are byte-identical before and after the
  corpus rewrite. That is the proof that the rewrite changes nothing.
- `cargo xtask dsl differential` shows no differences. `cargo xtask lean check` passes.
- Per-crate tests and clippy for `deckmaste_dsl` and `xtask`.
