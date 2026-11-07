---
needs: [comment-reviewer-hotspots]
---

# Comment discipline: sweep every crate

## Why

`crates/` holds about 29,000 comment lines (about 20,600 doc, 8,700 plain). The dominant genre is
the defensive mini-essay: real CR rulings wrapped in reviewer-directed argument, capitalized
emphasis and diff-speak. It teaches every reader to write the next one. The mass is
concentrated: 65 files with 100 or more comment lines, about 17,400 of their lines in
`sylex_engine`.

## Design

The comment rule, which the sweep applies and new and edited code follows:

- A comment states only what the code and types can't: rules meaning with its CR citation,
  an invariant, a rejected alternative and why, or a labeled seam.
- Arguing correctness is allowed where it's the idiom (a `// SAFETY:` block on `unsafe`).
- Banned: a comment excusing a known defect ("slightly wrong but okay for now because …"); a
  known defect is fixed or ticketed with a `TODO`/`SEAM` naming the ticket. Also banned: diff
  narration ("previously", "now uses") and plan narration.
- A plain `//` block over four lines is compressed, or moved to where its content belongs: the
  doc comment of the item it's about when it's local, the module doc when module-wide, an ADR
  when it's design.
- A doc comment opens with a one-sentence contract and documents the interface.
- A test comment states the scenario and the rule it proves.
- Tracked code never cites local memory.

## Done when

- Every file in `crates/` with 100 or more comment lines (regenerate the list at claim time with
  `rg -t rust -c '^\s*//' crates | awk -F: '$2>=100'`) has had a whole-file comment pass under the
  rule in Design, crate by crate: `sylex_engine`, `sylex_core`, `sylex_syntax`, then the rest. Files done by
  `comment-reviewer-hotspots` are skipped unless a violation was missed.
- The remaining files are checked for the mechanical patterns (diff-speak, imperative narration,
  commented-out code, plan-speak) hit by hit.
- No ruling is lost: every CR citation removed with a comment is either restated where it still
  applies or recorded as dropped in the round's report.
- Each round's diff changes no code outside comments and lands as its own commit.

## Not in scope

- `v1/`.
