---
needs: []
---
# A keyword's final cost picks its em-dash form from the cost, not from a second frame

Standard constraints apply. This is an English grammar and lexicon ticket.

`DashNonManaCost` is a spelling of a `Cost`, not a different Keyword Parameter. The Oracle style
guide ("End a keyword's em-dash-separated verbal cost with a period", `docs/oracle-style-guide.md`)
fixes the form by the cost itself: a mana-only cost follows the keyword after a space
(`Madness {1}{R}`, `Ward {2}`), and a cost with words follows an em dash and ends in a period
(`Madness—Pay six {C}.`, `Ward—{2}, Pay 2 life.`).

The catalog instead declares both forms as alternative frames on 22 keywords
(`parameters: [[Cost], [DashNonManaCost]]` in
`crates/deckmaste_lexical_source/catalogs/keyword_abilities.ron`), plus equip's and splice's
variants and escape's dash-only list.

## Change

- A keyword's final `Cost` slot realizes and reads both forms, choosing by whether the cost has
  words. Check the census for printed exceptions (an em dash before a mana-only cost, or a verbal
  cost after a space) before refusing either mismatch; read any that exist.
- Drop `DashNonManaCost` from the catalog and from `KeywordParameter`: `[[Cost], [DashNonManaCost]]`
  becomes `[[Cost]]`, equip's and splice's dash variants fold into their `Cost` frames, and
  escape's `[[DashNonManaCost]]` becomes `[[Cost]]`.
- `DashCost` (suspend's mandatory dash after an amount, `Suspend 4—{1}{U}`) is a different frame
  and stays.
- Update the `DashNonManaCost` paragraph in `crates/deckmaste_lexical_source/README.md`.

## Acceptance

- No face loses its Reading: every printed keyword cost in both forms still reads.
- The judgment set holds.
