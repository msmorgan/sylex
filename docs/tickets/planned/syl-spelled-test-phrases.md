---
needs: [syl-union-membership]
specs: [kind-sorts-and-position-unions]
---

# A card writes the phrase it prints, never a bare test

## Why

Forty-four canon files call an unspelled test directly, 49 sites, where the card prints a phrase
no declaration spells: *Coat of Arms*'s "that shares at least one creature type with it" is
`shares(subtype(creature), share, it)`, *Culling Scales*'s "with the lowest mana value" is
`superlative(min, stat(manaValue), …)`, *Keeper of the Flame*'s "who has more life than you do"
is `compare(…)`, *Gnarlback Rhino*'s "that targets this creature" is `targeting(…)`, and *Autarch
Mammoth*'s "while saddled" is `hasDesignation(…)`. A test has no spelling, so none of these terms
prints its card. Surface mode refuses a test in a card's term, and these sites are named in the
check as its exceptions. Three declarations at a kind have no spelling either: `ofChosen` and
`ofYourChoice`, called bare from 47 canon files, and `actorControls`.

## Done when

- Each of the 49 sites is read against its card's Oracle text and parse, and writes a spelled
  phrase at the kind the parse gives it, declared once per wording; the landing lists each site
  with its line and the declaration it took. A wording the clause, passive, lookback or `have`
  tickets compose is named with its ticket and left.
- `ofChosen`, `ofYourChoice` and `actorControls` are spelled, or replaced by the phrases their
  callers print.
- The check's list of cards that may write a test is empty apart from the sites left to those
  tickets, each named with its ticket.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Tests called from rule bodies and Definitions, which need not print.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §3, §5;
`docs/evidence/syl-kind-sorts/test-sort-check.md` (the callers of each test).
