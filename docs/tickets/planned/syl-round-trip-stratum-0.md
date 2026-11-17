---
needs: []
---

# A term is recovered from a Reading and printed back: Stratum 0

## Why

Nothing can check a surface term against its card. A term is written by hand, `sylex_spelling`
only parses the `spelling` notation, and `english-syl-term-recovery`, the ticket that maps a
Reading to a term, waits for the whole surface checkpoint. So the surface is designed in specs
several tickets deep with nothing to run them against, and three of those specs are DRAFT because
reviews keep finding them wrong.

The parser was built the other way: one Complexity Stratum at a time, each with a gate every unit
must pass and a check that fails on a regression (`crates/english/STRATA.md`). The round trip can
be built the same way, against the declarations as they are, and what breaks says what the
surface needs next.

## Design

- **The round trip of a unit**: its Readings are recovered to candidate terms built from the
  declarations in `plugins/builtin/`, each candidate is printed from its `spelling`s, and the
  print is compared with the unit's text.
- **Stratum by stratum.** This ticket is Stratum 0, the card frame: Type Lines, Keyword Lines and
  their parameters, symbol costs, modal headers and bullets (48,376 units; 1,777 faces have no
  unit above it). Each later Stratum is its own ticket, opened when the one before it reports.
- **The declarations are the thing under test.** This ticket changes none of them. A unit that
  does not round-trip is a finding, reported with its cause; the surface ticket that removes a
  cause is built when a Stratum's findings call for it, and its spec is corrected by them.
- **A canon card's file is a second check**: where a card is authored, the term recovered for a
  unit is compared with what the file writes for it.

## Open questions

- **How a term is printed.** Through the English grammar's own realization, so that inflection,
  agreement and punctuation are the grammar's (`docs/design/spelling.md` §1), or by a printer of
  `spelling`s alone for the Strata that need neither.
- **Where the code lives**, `crates/spelling` or a crate beside it, and what the command is
  called.
- **What a unit's outcome is when it has several Readings**: whether every Reading must
  round-trip or one.

## Done when

- The open questions are answered in `docs/design/spelling.md`.
- One command runs the round trip over every Stratum 0 unit of the pool and reports, per unit,
  one outcome: it round-trips; no candidate term, with the node of the Reading that no
  declaration spells; more than one candidate, with the declarations that compete; or a
  candidate that prints other text, with the difference. It reports the totals by outcome and by
  cause, and for authored cards the units whose recovered term differs from the card's file.
- A second run fails if a unit that round-tripped no longer does, as `cargo xtask english strata
  --check` does for Readings.
- The landing gives the totals, the causes ranked by the units each accounts for, and for each
  cause the planned ticket that removes it or the fact that none does.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Not in scope

- Changing a declaration so that a unit round-trips: the ticket its cause names.
- What a term means: the round trip says a term prints its card, not that it lowers correctly.
- Strata above 0.

## Pointers

`crates/english/STRATA.md`, `crates/english/README.md` (Complexity Strata, the forest database);
`crates/spelling/` (the notation's parser); `docs/design/spelling.md`, `docs/design/architecture.md`
("Spelling is a construction grammar"); `docs/tickets/planned/english-syl-term-recovery.md` (the
run over every canon card, which this trajectory ends in).
