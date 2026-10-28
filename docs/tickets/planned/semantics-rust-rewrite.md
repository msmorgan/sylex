---
needs: [plugins-v3-data-language, lean-v3-active-player-term, lean-v3-binding-across-or-arms, lean-v3-caller-argument-refusal-multiplicity, lean-v3-card-head-noun-sweep, lean-v3-cast-spell-anaphora-and-noted-mana, lean-v3-clash, lean-v3-colored-mana-spent-count, lean-v3-combat-damage-rule-rows, lean-v3-companion-pregame-rule, lean-v3-counted-pair-each-other, lean-v3-craft-materials, lean-v3-crewed-and-saddled-lookbacks, lean-v3-cross-ability-self-replacement, lean-v3-enlist-optional-attack-cost, lean-v3-hand-written-own-scope-bodies, lean-v3-keyword-body-spelling-gaps, lean-v3-later-ability-acts-before-earlier, lean-v3-literal-referent-id-pins, lean-v3-lose-control-event, lean-v3-owner-zone-introduces-owner, lean-v3-phasing-direct-phase-out, lean-v3-player-speed-rules, lean-v3-preposed-static-condition-ordering, lean-v3-regenerate-target-readback, lean-v3-same-color-mana-spent-count, lean-v3-single-use-as-though, lean-v3-turn-face-up-spend-cost, lean-v3-venture-dungeon, lean-defense-projection, lean-marked-damage-projection]
---
# [design] Rewrite the Rust semantics pipeline: `semantics`, `discourse`, `lowering`

Standard constraints apply. This is an epic, and needs a design dialogue before it is split into
tickets.

Delete `deckmaste_semantics_v3` and `deckmaste_lowering_v3`. Their only outside user is
`xtask expansions`, which goes too; `lean-elaborated-value-dump` replaces it. Rebuild as three
crates:

- **`semantics`** parses the `plugins-v3-data-language` grammar into its AST and expands
  Templates and Definitions. It does no reference resolution.
- **`discourse`** implements the DRT referent resolution of `docs/decisions/semantics-v3.md` §1
  and §3 in Rust. Lean is never in the hot path of card compilation, so this is a
  reimplementation, not a reader of Lean output. `deckmaste_lowering_v3` already has part of it.
- **`lowering`** translates the resolved AST to the `deckmaste_core` representation. Its seams
  are as already designed in `lowering-v3`: regions, `ownScope` to `Action::Composite`, the
  action-clause spine, and so on.

## Sequencing (owner, 2026-10-28)

Start this only after:

- `lean-build-speed`;
- `plugins-v3-data-language`;
- the remaining `lean-v3-*` tickets, so that the Lean workbench's semantics are settled before
  Rust reimplements them.

## Verification

The Lean checker is the oracle for `discourse`. Per card, Rust's resolved bindings (first and
later mentions, `D′`, `AC>R1` choices) must match the bindings Lean's checker produces. Lean can
export them as a development tool, outside the compile path. The design dialogue settles that
export and how per-card agreement is gated.
