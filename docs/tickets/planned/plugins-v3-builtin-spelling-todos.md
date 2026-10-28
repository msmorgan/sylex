---
needs: [plugins-v3-primitive-template-boundary]
---
# Triage the 614 `-- TODO(spelling):` markers in `plugins_v3/builtin`

Standard constraints apply. The `TODO(spelling)` marker was introduced by
`plugins-v3-builtin-keywords` and `plugins-v3-builtin-actions-and-definitions` for declarations
written without a `spelling := "..."` string. `spelling` is the Layer 3 surface template, and
Lean discards it (`docs/decisions/semantics-v3.md` §4.4). It currently appears 614 times, mostly
with one of two stock texts:

- "No fixed surface wording; this helper composes semantic structure" (163).
- "Structural constructor; surface wording belongs to its phrasing templates" (107).

Sort every marker into exactly one bucket:

1. **No surface wording of its own.** This covers structural primitives, helpers,
   `byCardClass` and `flavorWord`. Delete the marker; nothing is deferred.
2. **The wording is fixed, and only inflection, article choice or cost punctuation held it
   back.** Examples: `amass`, `affinity`, `bandsWithOther`, `champion`, `ward`, `cycling`,
   `equip`, `activatedBy`, `activatedOnly*` and `attachChoosing`. Write the uninflected
   `spelling` in the form already used by `crew.lean` and `saddle.lean`
   (`"crew <amount>"`). Inflection belongs to the future spelling layer (`deckmaste_spelling`
   is a stub today), so it is no reason to leave the string out.
3. **The surface wording is genuinely deferred.** Keep the marker and state what is missing.

Do the triage with a script that produces the list of declarations in each bucket. That list
goes in the change description; it is not tracked evidence.

## Acceptance

- Every remaining `TODO(spelling)` falls in bucket 3 and says what is deferred.
- Every bucket-2 declaration has a `spelling`.
- The expansion baseline accepts every bucket-2 declaration via `--expect`. Review
  every unified diff to confirm that only `spelling` changed; semantic expansions
  and `lexical` must remain unchanged.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-builtin-spelling-todos/before
```

The triage script writes every bucket-2 declaration key (`name : Type`, one per
line, no blank lines) to `docs/evidence/plugins-v3-expansions/plugins-v3-builtin-spelling-todos/bucket-2.keys`. Pass every key
through `--expect` after adding its spelling:

```bash
mapfile -t bucket2 < docs/evidence/plugins-v3-expansions/plugins-v3-builtin-spelling-todos/bucket-2.keys
expected=()
for key in "${bucket2[@]}"; do expected+=(--expect "$key"); done
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-builtin-spelling-todos/before "${expected[@]}"
```

Check every unified diff: only `spelling` may change. Semantic expansions and
`lexical` must stay unchanged, including on expected keys; `--expect` accepts a
declaration difference without classifying its contents. No rename map is needed.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.
