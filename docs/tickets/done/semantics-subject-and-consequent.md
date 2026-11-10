---
needs: []
---

# A clause's Subject is a referent; a conditional's consequent exports its domain

## Why

`semantics.md` §2.1 says Actors are not referents, while §4.2 calls a clause's active Subject the
Actor, so "it" in "whenever a creature dies, … it" would read an Actor. *Guardian Beast* reads
"those artifacts" after the conditional sentence that introduces "noncreature artifacts you
control", and §2.2 lets nothing a conditional's consequent introduces reach later text. The
condition-first conditional Template is named `conditionFirstWhile`, and some `onlyWhile` callers
print another wording that an existing Template spells.

## Done when

- §2.1 and §4.2 say a Subject's noun phrase is an ordinary referent and the Actor is who performs
  the Action, read only by the `actor` noun-phrase word; the glossary entries **Subject** and
  **Actor** say the same, and no sentence in `docs/decisions/` or `docs/specs/` calls a Subject's
  noun phrase the Actor.
- §2.2 item 2 says a conditional Static Spec's consequent exports the plural domain of a bare
  plural or "all" it introduces to the text after the conditional in its ability.
- `syl-misnamed-declarations` states the `actor(np)` rename in these terms.
- The condition-first Template is `asLongAs` at `StaticSpec`, and no `conditionFirstWhile` is left
  in `plugins/`.
- The `onlyWhile` callers whose rule text prints "<spec> if <condition>" (freerunning, prowl,
  spectacle, surge) use `onlyIfSo`, and living metal's "During your turn" uses `partScope`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask`, `cargo xtask cite
  check` and `kata kanban check` pass.

## Not in scope

- Renaming `actor(np)` to `subject(np)`: `syl-misnamed-declarations`.

## Pointers

`docs/decisions/semantics.md` §2.1, §2.2, §4.2; `docs/contexts/game-model/CONTEXT.md`;
`plugins/builtin/templates/static/{asLongAs,onlyWhile,onlyIfSo}.syl`;
`plugins/canon/cards/Guardian Beast.syl`; `plugins/builtin/definitions/supertype/legendary.syl`.
