---
needs: [syl-bodiless-keywords, syl-single-face-card-call, cleanup-step6-surface]
---

# One keyword, one declaration

## Why

Several keywords are declared many times, once per printed variant: `hexproof` and
`hexproofFrom`, and `partner` with `partnerWith`, `partnerCharacterSelect`, `partnerFatherAndSon`,
`partnerFriendsForever`, `partnerSurvivors`, `chooseABackground` and `doctorsCompanion`: the
partner abilities are partner, partner—[text], partner with [name], choose a Background and
Doctor's companion [CR#702.124a], and `partner.syl` already lists the last two among its
variants.

Six more keyword abilities are declared apart from the keyword they are a variant of, and joined
to it only by a `label` string naming that keyword: `devourQuality` (devour [CR#702.82c]),
`emergeFrom` (emerge [CR#702.119b]), `equipQuality` (equip [CR#702.6c]), `megamorph` (morph
[CR#702.37b]), `multikicker` (kicker [CR#702.33c]) and `typecycling` (cycling [CR#702.29e]).
`hexproofFrom` is joined to `hexproof` the same way. Only that string makes a megamorph cost a
morph cost or a multikicker cost a kicker cost [CR#702.37b,702.33c], and nothing checks that it
names a keyword.

A narrower ticket rewrites the same declarations first: `syl-bodiless-keywords` (the seven
partner-ability files).

Written against the surface (`cleanup-step6-surface`).

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Design

- A keyword is one Definition, and its printed variants are Templates or parameters over it.

## Done when

- `hexproof`, `partner`, devour, emerge, equip, morph, kicker and cycling are each
  one Definition; each variant is a Template or an argument over it, keeping its printed wording;
  every caller in `plugins/` is updated.
- No `ability` declaration carries a `label`, and the `ability` kind has no `label` field
  (`crates/xtask/src/dsl_check/schema.rs`).
- Any variant kept as its own Definition states in its doc comment the rule that makes it one.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The keyword actions declared once per variant under a shared Deed label (`exchange`, `exile`,
  `double`, `triple` and others): merging them merges their Deeds, so it is
  `syl-one-deed-per-keyword-action`.

## Pointers

`plugins/builtin/definitions/keyword_ability/{hexproof,hexproofFrom}.syl`,
`plugins/builtin/definitions/keyword_ability/{partner*,chooseABackground,doctorsCompanion}.syl`,
`plugins/builtin/definitions/keyword_ability/{devour,devourQuality,emerge,emergeFrom,equip,equipQuality,morph,megamorph,kicker,multikicker,cycling,typecycling}.syl`,
`docs/decisions/semantics.md` §3.1.
