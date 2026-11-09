---
needs: [syl-active-player-noun-phrase, syl-balance-family, syl-basic-land-mana-abilities, syl-card-type-permanent-type, syl-clash, syl-cleave-bracketed-text, syl-colored-mana-spent-count, syl-counter-conferrals, syl-cross-ability-self-replacement, syl-daybound-reconfigure-bodies, syl-deathtouch-creature-conferral, syl-doc-comment-rationales, syl-fight-both-performers, syl-ice-cauldron-noted-mana, syl-implicit-coercions, syl-indirect-phase-out, syl-later-ability-acts-before-earlier, syl-local-binding, syl-lose-control-event, syl-replacement-except-nth-event, syl-ring-tempts-you-raw-scope, syl-sacrifice-performer-controls, syl-single-use-as-though, syl-speed-rules, syl-subtype-card-types, syl-turn-face-up-spend-cost, syl-unwritable-card-shapes, syl-venture-dungeon]
---

# Step 6: the `.syl` iteration is complete

## Why

Step 6 of the cleanup reshapes the `.syl` corpus under `cargo xtask dsl check` before step 7
builds the Rust AST from it. This ticket closes when every step-6 ticket has landed. Its `needs:`
are the step-6 tickets no other step-6 ticket needs, so every step-6 ticket is reached through
them; a new step-6 ticket joins those `needs:` unless another step-6 ticket needs it.

## Done when

- Every step-6 ticket is in `done/`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.
