---
needs: [syl-unused-parameter-refusal]
---

# Explicit Deeds in `.syl`

## Why

`.syl` cannot write a raw Deed (`docs/decisions/syl.md` §4): an `action` declaration names its
Deed only through its `deed`, `permanent` and `ofAbility` fields
(`crates/xtask/src/dsl_check/schema.rs`), and a keyword ability names none. Crew shows the cost.
The `crew` keyword ability's cost is a bare `tap`, and a separate bodiless primitive
`crew(vehicle)` (`plugins/builtin/primitives/action/crew.syl`, likewise `saddle.syl`) stands in
for the Deed, linked to nothing. Resolution by sort hides the clash: `crew(lit(2))` (*Debris
Beetle*) resolves to the ability, `crew(allOf(hasSubtype(vehicle)))` (*Revoke Privileges*) to the
primitive. So *Revoke Privileges*' "can't crew Vehicles" [CR#702.122d] forbids nothing the cost
does, *Hotshot Mechanic*'s "crews Vehicles as though its power were 2 greater" never reaches the
power total, "crewed by" [CR#702.122c] has nothing to read, and `docs/decisions/semantics.md` §3's
claim that Crew's Definition carries a Deed is false.

## Design

- `.syl` writes Deeds explicitly, reversing `syl.md` §4's "cannot write a raw Deed".
- A Deed is declared by its own declaration kind, `deed_action`, kept apart from ordinary
  Definitions. It is the one kind exempt from the unused-parameter refusal
  (`syl-unused-parameter-refusal`).

## Open questions

These are settled with the owner at the end of step 6, after the rest of its tickets.

- The `deed_action` syntax, and what replaces the `deed`, `permanent` and `ofAbility` fields of
  `action`. A seed, not settled:
  ```
  deed_action crew(what: NounPhrase) {
    spelling = "crew <what>",
    body = instruct(controllerOf(theActor), tap(it))
  }
  ```
  The creature is the actor and `what` (the Vehicle) is the Deed's object, unused in the body
  because crewing is done by tapping and only names what got crewed [CR#702.122b]; `it` reaches
  back past the handoff, where the actor is the controller.
- The crew, saddle [CR#702.171c] and convoke [CR#702.51c] Deeds: each chosen creature performs the
  Deed in the cost, the cost's power total is read in that Deed's context so *Hotshot Mechanic*'s
  +2 reaches it, and the `crew` and `saddle` primitives go. Improvise names no Deed.
- One shared exile Deed: `exile`, `exileFrom`, `exileFaceDown`, `exileFaceDownFrom` and
  `exileWithCounters` (`definitions/keyword_action/`) each declare their own, so "exiled" can be
  matched only by its label.
- Search marking what it finds: "a search found this card, so it is known after the shuffle"
  [CR#701.23a] is keyed to the Deed name `search`; whether `search`'s body marks the found card
  instead.
- Whether `cast`, `play` and `activate` are matched by their Deeds rather than by their labels,
  and how a `cast` relates to a `play`.
- Regenerate. The instruction "regenerate [permanent]" creates a regeneration shield, a
  replacement effect [CR#701.19a,701.19b]. The event is a permanent actually being regenerated;
  creating a shield is not regenerating, and "can't be regenerated" stops the shield from being
  applied [CR#701.19c]. Today `regenerate` (`deed = none`) installs the shield and `regeneration`
  carries the `"regenerate"` Deed. Which declaration carries the Deed, and whether the two are
  inverted, is settled in this discussion.

## Done when

- The settled answers are in `docs/decisions/syl.md` §4 and `docs/decisions/semantics.md` §3 and
  §3.1, and §3's statement about Crew's Deed is true.
- `deed_action` exists in `cargo xtask dsl check`, and a `deed_action` with an unused parameter is
  accepted, with a test, while the same parameter on any other kind is refused.
- *Debris Beetle*'s `crew(lit(2))` and *Revoke Privileges*' "can't crew Vehicles" name the same
  Deed, and `plugins/builtin/primitives/action/crew.syl` and `saddle.syl` stand for nothing that
  the keyword's Deed does not.
- What *Revoke Privileges* forbids, how *Hotshot Mechanic*'s +2 reaches the power total and what
  "crewed by" reads are not checkable until the Rust resolution layer exists; the landing shows
  each reading for the owner to review.
- The `schema.rs` items marked "Step 6" for Deeds (the `action` kind's Deed fields, its
  `secondary` reservation, and `EXPANSION_CONSTRUCTORS`) are reshaped to the settled design.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/{crew,saddle,convoke}.syl`,
  `plugins/builtin/primitives/action/{crew,saddle}.syl`
- `plugins/builtin/definitions/keyword_action/{regenerate,regeneration,search}.syl` and the five
  `exile*` keyword actions
- `plugins/canon/cards/{Debris Beetle,Revoke Privileges,Hotshot Mechanic}.syl`
- `crates/xtask/src/dsl_check/schema.rs` (`action`, `EXPANSION_CONSTRUCTORS`),
  `crates/xtask/src/dsl_check.rs`
- `docs/decisions/semantics.md` §3 and §3.1, `docs/decisions/syl.md` §4
