---
needs: [plugins-v3-builtin-cleanup, plugins-v3-primitive-template-boundary]
---
# Restore doc comments and CR citations dropped by the `plugins_v3/builtin` extraction

Standard constraints apply. The extraction generated files from `lean/Semantics/` docstrings
only. It did not merge the comments from `plugins_v2/builtin` (still readable at `tkryskrm`,
for example `jj file show -r tkryskrm plugins_v2/builtin/macros/keyword_abilities/<x>.ron`).

## Gap, as measured on 2026-10-27

Counts are files with a `/--` comment against total files, compared with the v2 source.

| Folder | v3 documented | v2 source |
|---|---|---|
| `definitions/keyword_ability` | 68 / 207 | 211 / 211 (202 cite the CR) |
| `definitions/keyword_action` | 67 / 89 | — |
| `definitions/designation` | 6 / 21 | 20 / 20 |
| `definitions/relation` | 0 / 3 | 3 / 3 |
| `definitions/counter` | 4 / 218 | 29 / 218 |
| `definitions/subtype` | 7 / 463 | 122 / 462 |
| `primitives` | 305 / 624 | v2 `macros/{abilities,instructions,…}` |
| `templates` | 322 / 473 | same |

Also delete the stray `/- ## Keyword abilities -/` section-header fragments at the top of
extracted files (for example `absorb.lean`).

## How to run it

- Comments carry no meaning, so the expansion baseline must show zero changes.
- Split the work by folder across parallel agents (Sonnet is enough), all given the same brief.
  Each agent ports the v2 comment where one exists. Otherwise it writes a short comment citing
  the CR for keyword abilities, keyword actions, designations, relations and card types.
- Counters and subtypes get a comment only where v2 had one or the CR defines rules for them.
  A subtype with no rules of its own needs none.
- Every ported citation is re-read against `data/rules/` before landing. Run
  `jj diff --git | cargo xtask cite audit --diff`, because v2 citations can be stale.

## Acceptance

- Every keyword ability, keyword action, designation and relation carries a `/-- … -/` with
  its CR citation.
- Every v2 comment for a declaration that still exists has been carried over or deliberately
  replaced.
- The CR citation gates are clean: `cite check --list-noncompliant` is empty, `cite check`
  reports 0 stale, and `bless` has been run.
- The baseline shows no changes.
