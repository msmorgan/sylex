---
needs: [plugins-v3-builtin-cleanup]
---
# Clean up `plugins_v3/canon/cards` authoring syntax, coercions, and filenames

**Replace `camelCase` card identifiers, `: Spelled` annotations, raw dot-constructors, and
`{ body := ... }` wrappers in `plugins_v3/canon/cards/` with anonymous `card := ...` declarations,
structured multi-faced constructors, authoring coercions, and `card_filename` filenames.** Standard
constraints apply.

## Scope

1. **Anonymous `card := ...` header and `cardNamed` proof lookup:**
   - Change the `card` command syntax in `lean/Semantics/Authoring.lean` and
     `crates/deckmaste_semantics_v3/src/{reader.rs,bundle.rs,lower/mod.rs}` from
     `card <ident> : Spelled := ...` to anonymous `card := <term>` (with no identifier or
     `: Spelled` before `:=`).
   - Derive the internal declaration key (`Name.mkSimple` in Lean, `Ident` in Rust) from the card's
     printed name: `name` on a single-faced card (`"Saw in Half"`), `<front.name> // <back.name>` on
     `transforming` and `modalDfc` cards (`"Branchloft Pathway // Boulderloft Pathway"`),
     `<left.name> // <right.name>` on `split` and `sharedLineSplit` cards, and
     `<normal.name> // <adventure.name>` (or `<normal.name> // <alternative.name>`) on `adventurer`
     and `flip` cards.
   - Provide a `cardNamed "<name>"` term elaborator in `lean/Semantics/Authoring.lean` so proof
     suites (`lean/Semantics/Proofs/Cards*.lean`) reference a canon card by its printed name string
     (`(cardNamed "Saw in Half").card`) without invented `camelCase` Lean identifiers.

2. **Card filenames on disk via `deckmaste_core::plugin::card_filename`:**
   - Rename all 908 files in `plugins_v3/canon/cards/` from `<camelCase>.lean` to
     `<card_filename(name)>.lean` using the character escaping from
     `crates/deckmaste_core/src/plugin.rs` (`card_filename`), where `< > : " / \ | ? *` map to
     `{less}`, `{greater}`, `{colon}`, `{quote}`, `{slash}`, `{backslash}`, `{pipe}`, `{question}`,
     `{star}` and spaces, periods, commas, apostrophes, and hyphens remain literal (e.g.
     `Saw in Half.lean`, `Branchloft Pathway {slash}{slash} Boulderloft Pathway.lean`,
     `Summon{colon} Esper Ramuh.lean`).

3. **Structured `Card` layout constructors and `Coe Characteristics Card`:**
   - Provide `Coe Characteristics CardFace := ⟨CardFace.mk⟩` and
     `Coe Characteristics Card := ⟨singleFacedCard⟩` so single-faced cards are written directly as
     `card := { name := "...", ... }` without `{ body := ((singleFacedCard ...)) }` or
     `{ characteristics := { ... } }`.
   - Expose each multi-faced or multi-part `Card` constructor (`transforming`, `modalDfc`, `split`,
     `sharedLineSplit`, `adventurer`, `flip`, `leveler`, `prototype` [CR#709.1,710.1,711.1,712.1,715.1,718.1])
     as a primitive/template taking its fields as a braced record (`ctor { field := value, ... }`
     elaborating to `ctor (field := value) ...` in Lean and lowering in `deckmaste_semantics_v3`):
     - `card := modalDfc { front := { ... }, back := { ... } }`
     - `card := transforming { front := { ... }, back := { ... } }`
     - `card := adventurer { normal := { ... }, adventure := { ... } }`
     - `card := split { left := { ... }, right := { ... } }`
     - `card := sharedLineSplit { shared := { ... }, left := { ... }, right := { ... } }`
     - `card := flip { normal := { ... }, alternative := { ... } }`
     - `card := leveler { inner := { ... }, bands := [ ... ] }`
     - `card := prototype { inner := { ... }, alternative := { ... } }`
   - Remove the `body` field from `card` declarations in `Authoring.lean` and `reader.rs`.

4. **Missing coercions, defaults, and bare word primitives:**
   - Add `OfNat PrintedStat n` (and `OfNat (Option PrintedStat) n` / `Coe PrintedStat (Option PrintedStat)`),
     `OfNat (Option Amount) n`, and `Coe Amount (Option Amount)` in `lean/Semantics/Primitives.lean`
     and `crates/deckmaste_semantics_v3/src/lower/` so `power := 3, toughness := 2, loyalty := 3`
     write bare numerals instead of `stat 3`.
   - Default `copyCharacteristics`'s `typesAdded : Bool := false` parameter.
   - Ensure bare primitives/templates exist in `plugins_v3/builtin/` (and are used instead of raw
     dot-constructors `.creature`, `.white`, `.legendary`, `.atLeast`, `.upkeep`, `.endStep`, `.x`,
     `.of .blue`, etc.) for card types, colors, supertypes, comparators, turn parts, roundings, and
     quality sorts.

5. **Rewrite all 908 `plugins_v3/canon/cards/*.lean` files to house style and strip bare-name comments:**
   - Omit `: Spelled` and `body :=`.
   - Use coercions and top-down list elaborators (`cost := [2, black]`, `types := [instant]`,
     `supertypes := [legendary]`, `power := 3`, `toughness := 2`, `manaOf [[blue], [colorless, blue]]`,
     bare `StaticSpec` in `text := [...]` without `static (...)`, `[a, b]` for `sequentially [a, b]`
     in `Action` position and `and [a, b]` in `Predicate`/`Condition` position, and bare card-type /
     noun predicates in `target creature`, `target [permanent, non land, hasManaValue atMost 3]`, etc.).
   - Strip the ~420 redundant bare card-name doc comments (`/-- Doubling Season -/`,
     `/-- Mourning Patrol // Morning Apparition -/`, etc.). Card files do not need a comment
     repeating their name or a generic CR citation; retain (or carry over from `plugins/canon/cards/`)
     only substantive comments on cards that were canonized to illustrate or pin a particular rule
     or interaction (e.g. `adaptiveAutomaton.lean`, `hitRun.lean`).
   - Format multi-line records and ability lists consistently across all 908 card files.

## Examples

```lean
-- plugins_v3/canon/cards/Saw in Half.lean
card := {
  name := "Saw in Half",
  cost := [2, black],
  types := [instant],
  text := [
    spell [
      destroy (target creature),
      doIf (did (that creature) die thisWay)
        (instruct (controllerOf it)
          (create 2
            (copyOf (that creature)
              (copyCharacteristics {
                power := half up (powerOf (that creature)),
                toughness := half up (toughnessOf (that creature))
              }))))
    ]
  ]
}

-- plugins_v3/canon/cards/Academy Journeymage.lean
card := {
  name := "Academy Journeymage",
  cost := [4, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    onlyIfSo (costShift this (less 1)) (exists_ [wizard, youControl]),
    when (does thisCreature enter)
      (returnToHand (target [creature, anOpponentControls]))
  ],
  power := 3,
  toughness := 2
}

-- plugins_v3/canon/cards/Branchloft Pathway {slash}{slash} Boulderloft Pathway.lean
card := modalDfc {
  front := {
    name := "Branchloft Pathway",
    types := [land],
    text := [activated tapSymbol (addMana 1 (manaOf [[green]]))]
  },
  back := {
    name := "Boulderloft Pathway",
    types := [land],
    text := [activated tapSymbol (addMana 1 (manaOf [[white]]))]
  }
}
```

## Acceptance

- Every file in `plugins_v3/canon/cards/` is named `<card_filename(name)>.lean` and begins with
  `card :=` (after any rule-explaining doc comment).
- Zero bare card-name stub comments (`/-- <Card Name> -/`) in `plugins_v3/canon/cards/`; only cards
  canonized to illustrate a specific rule or interaction carry a doc comment.
- Zero occurrences of `: Spelled`, `body :=`, `singleFacedCard`, `{ characteristics :=`, `generic `,
  `pip .`, `stat `, or raw dot-constructors (`.creature`, `.white`, `.of .`, etc.) in
  `plugins_v3/canon/cards/`.
- `cargo test -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask`,
  `cargo clippy -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask --tests -- -D warnings`,
  `cargo xtask lean check`, and the CR citation gates pass.
