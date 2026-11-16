---
needs: [cleanup-step6-spine, syl-number-snapshot, syl-printed-it-self-references, shortened-names-at-load]
---

# A card that prints its own name writes a name form, not `this`

## Why

Canon cards write the bare `this` primitive where the card prints its own name: *Lightning
Bolt*'s "Lightning Bolt deals 3 damage to any target" is `instruct(this, dealDamage(…))`. Text
that names the object it's on means that object [CR#201.5], and some cards print a Shortened Name
instead, treated as the full name [CR#201.5c]: *Urza, Lord High Artificer*'s "When Urza enters".
One face can print both: *Immard, the Stormcleaver* says "Whenever Immard, the Stormcleaver
enters" and "Immard deals 4 damage to any target". Written as `this`, the printed name, the
Shortened Name and "this creature" are one term, so spelling can't render what the card prints
and nothing checks it. Some sites write a `this<Head>` Template where the card prints its name:
*Atris, Oracle of Half-Truths* prints "When Atris enters" and writes `thisCreature`.

`syl-printed-it-self-references` rewrites canon self-reference sites to `it` first, and
`syl-this-head-template` rewrites the sites that print "this <head>" after; all three rewrite
canon self-reference sites, so they land in that order.

`shortened-names-at-load` makes the Shortened Name rule a public function,
`sylex_vocabulary::short_name(name, supertypes, types)` in `crates/vocabulary/src/shortening.rs`,
which this ticket's check can call, and lands first.

## Design

- `primitive ownName: NounPhrase = OwnName` (`spelling = "#"`) names the full name, and
  `template shortenedName: NounPhrase` (`spelling = "##"`, `body = ownName`) names the Shortened
  Name [CR#201.5c].
- Neither takes a string: `ownName` names the object whose characteristics hold the ability, so an
  ability gained by an object with another name reads as that name [CR#201.5b], and a named
  token's ability uses the same pair for the name its creating effect sets [CR#201.5,111.4]
  (`syl-token-records-to-templates`).
- `theGrantor` is deleted; inside an ability granted to another object, a printed name writes
  `ownName` and semantics resolves whether it names the ability's bearer or the granting source
  [CR#201.5a].
- A face stores no Shortened Name string. `shortenedName` names the shortening that `short_name`
  computes from the face's `name`, `supertypes` and `types` (a legendary permanent's name, cut by
  that rule), so names never come by hand (`docs/design/lexicon.md` §3).
- `cargo xtask dsl check` refuses `shortenedName` on a card face for which `short_name` gives no
  Shortened Name, and both forms anywhere in `plugins/builtin/` (save for `shortenedName`'s own
  `body = ownName`), where there is no face.

## Done when

- Every canon mention where the card prints its full name or its Shortened Name uses `ownName` or
  `shortenedName`, `theGrantor` is deleted, and the landing lists the faces that print a Shortened
  Name.
- Canon `this` sites that print "this spell" rather than a name (*Abolish*'s
  `altCost(this, …)`, and `costShift(this, …)` and `keywordCostPaid(kicker, this)` sites such as
  *Academy Journeymage*, *Embercleave* and *Conqueror's Pledge*) are left to `syl-this-head-template`.
- Negative tests refuse `shortenedName` on a face without one, and either form in
  `plugins/builtin/`.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- Sites that print "this <head>": `syl-this-head-template`.
- Named tokens in `plugins/canon/`: `syl-token-records-to-templates`.

## Pointers

`plugins/builtin/primitives/pronoun/{ownName,this_}.syl`,
`plugins/builtin/templates/pronoun/shortenedName.syl`, `plugins/canon/cards/`,
`docs/design/spelling.md` §7, `docs/design/semantics.md` §2.4,
`docs/contexts/oracle-english/CONTEXT.md` (**Shortened Name**, **Proper Name**).
