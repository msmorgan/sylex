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
- The expansion baseline shows no changes, because Lean discards `spelling`.
