---
needs: [cleanup-step6-surface, research-semantics-reading-slice]
---

# `surface.md` holds the surface design, and `semantics.md` the reading

## Why

`docs/design/semantics.md` was written when the term language's own sorts said what a clause
means, so it holds two things: what a term is built from, and how a term is read. Its §3
(Definitions and Templates) and most of its §4 (one Action per verb, voice, patients, how a
static statement is worded) are the first, and they belong in `docs/design/surface.md` with
what that document already says. Its §2 (the Discourse Representation Structure, boxes,
references) and §6 to §8 are the second. The planned voice tickets edit §4 by section number, so
the two are separated once those tickets and the slice that tests §6 have landed.

## Done when

- Every passage of `semantics.md` §3, §4 and §5 is in exactly one of the two documents:
  `surface.md` where it says what a term is built from or how it is worded, `semantics.md`
  where it says how a term is read or what a clause means. The landing lists each subsection
  with where it went.
- `semantics.md` is ordered as the reading runs: classification, with the contexts, the event
  gate, the before-and-after reads and the lookback rules as what classification decides;
  boxes; references; building the meaning; then the pipeline from text to a stored term.
- `surface.md` states the clause, the verb phrase and its operators, the passive and the
  object phrases as they stand in `plugins/builtin/` after the surface gate, with what
  `docs/specs/voice-and-object-phrases.md` leaves to a design doc.
- No passage of either document describes the scaffolding sorts as what a term says without
  saying that they are scaffolding.
- Every reference to a moved section, in `docs/design/`, `docs/specs/`, `docs/tickets/planned/`
  and the doc comments under `plugins/` and `crates/`, names where the passage now is.
- `docs/design/README.md` describes both documents as they then stand.
- `cargo xtask cite check` reports 0 stale and `cargo xtask cite check --list-noncompliant` is
  empty.

## Not in scope

- Changing what either document says. A passage the slice's findings contradict is edited by
  `research-semantics-reading-slice`.
- `docs/design/syl.md`, which stays the language: syntax, name resolution, expansion, read
  order.

## Pointers

- `docs/design/semantics.md`, `docs/design/surface.md`, `docs/design/syl.md`,
  `docs/design/architecture.md` ("The layers")
- `docs/specs/voice-and-object-phrases.md` §10, which lists the sections its tickets edit
