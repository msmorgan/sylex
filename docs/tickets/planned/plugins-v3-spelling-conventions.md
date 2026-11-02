---
needs: [syl-parameter-sigil, semantics-rust-rewrite]
---
# Settle the `spelling` conventions and apply them to the corpus

Standard constraints apply. This absorbs `plugins-v3-spelling-deferred-reasons`. It needs
`syl-parameter-sigil` only because both rewrite the whole `.syl` corpus.

A `spelling` string is meant to be parsed through the English grammar with typed holes, and
recovery and rendering unify against that parse (`deckmaste_spelling`, not built yet). These
conventions make the strings fit for that. They also settle the 151 `TODO(spelling)` markers,
123 of which carry generated text instead of a reason.

## Decisions already made

- **Uninflected.** Spellings are written uninflected wherever agreement depends on a hole or on
  the performer. An Action's spelling is the bare Verb Phrase: the performer stays outside it, and
  agreement is solved when a clause is rendered. So `die` is `"die"`, `crew` is
  `"crew <vehicle>"`, `win` is `"win the game"`, and `becomeBlocked` is
  `"<subject> become blocked"`.
- **Holes are `<name>`.** Angle brackets are the only markup, because they never appear in Oracle
  text. Square brackets do appear in Oracle text, so no bracket markup is used for optional text.
- **Optional parameters list their wordings.** A declaration with a defaulted or `Option`
  parameter whose presence changes the wording writes one pattern per printed wording:
  `spelling = ["return <subject> to its owner's hand", "return <subject> from <from> to its owner's
  hand"]`. A parameter whose hole is absent from a pattern is at its default in that pattern.
  - Two patterns with the same set of holes are a load error.
  - Every call in `builtin/` and `canon/` must have a pattern for its set of non-default
    arguments.
  - `connive(amount = 1)` becomes `["connive", "connive <amount>"]`.
  - A later drop rule may merge pattern lists semi-mechanically. That is not part of this ticket.
- **List holes coordinate with "and".** A list hole spells as a Coordination of its entries, with
  the serial comma at three or more. `<hole:or>` names a different Coordinator from the
  lexicon's coordinators (`closed_class.ron`: `and`, `and/or`, `but`, `nor`, `or`), so
  `ofAnyColorAmong` is `"one mana of any of <colors:or>"`. Correlatives are settled by the first
  ticket that needs one.
- **Out of scope:** Entry Riders and Durations. They become surface adjunct nodes
  (`lean-v3-entry-rider-and-duration-adjuncts`), so their markers ("Include or omit the clauses
  supplied by duration", the `riders` list entries) move to that ticket. The same goes for any
  other marker whose parameter becomes an adjunct node there.
- **A template with several parses** keeps every parse that recovers to its `body`, and fails to
  load if none does. This is recorded in the ADR for `deckmaste_spelling`; no code here.
- **Rendering reproduces the analyzed Oracle text.** That is the text after balanced reminder text
  is removed and keyword-line semicolons become commas, with cleave's Bracketed text written back
  from where it is kept (`english-lexical-analysis.md` §1).
  - A wording difference with the same meaning lives in which Template a card calls, never in the
    AST. Put and return are the existing case.
  - This replaces "byte-exact replay is out of scope" in `semantics-spelling-lowering.md` §3.

## Change

- **ADR:**
  - Record the conventions where `spelling` is defined (`docs/decisions/semantics-v3.md` §4.4).
  - Amend `semantics-spelling-lowering.md` §3 as above.
- **Glossary:** add entries for the terms this introduces (a spelling's pattern and its holes) to
  the owning glossary, via the `domain-modeling` skill.
- **Rust `semantics` and `deckmaste_dsl`:**
  - `spelling` accepts a string or a list of strings.
  - `semantics` checks that no two patterns share a set of holes, and that every call's set of
    non-default arguments has a pattern. `semantics` is used because it resolves every call.
    (Not SylGen: after `semantics-rust-rewrite` it is only a mirror of the DSL.)
  - Add negative cases for both errors.
- **Corpus, in its own commits:**
  - Rewrite every Action spelling to the bare Verb Phrase.
  - Write pattern lists for optional parameters.
  - Resolve every marker this ticket owns: write the spelling, or replace generated text with the
    specific wording or realization that is missing.

## Acceptance

- No Action spelling is in a finite form. No marker carries generated text, and every remaining
  marker names something specific.
- The `semantics` checks pass over `builtin/` and `canon/`, and their negative cases fail as
  expected.
- The expansions `semantics` produces are identical before and after the corpus rewrite, since
  spelling carries no meaning.
- Per-crate tests and clippy for `deckmaste_dsl` and `semantics`.
