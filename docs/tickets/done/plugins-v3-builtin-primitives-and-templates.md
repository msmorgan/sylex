---
needs: [plugins-v3-builtin-actions-and-definitions]
---
# Migrate `Templates.lean` into `plugins_v3/builtin/{primitives,templates}/`

**Split `lean/Semantics/Templates.lean` (816 declarations) into individual `.lean` files under
`plugins_v3/builtin/primitives/` (1-to-1 `Semantics.Primitives.*` bindings) and
`plugins_v3/builtin/templates/` (composite phrasings), and enforce that only `primitive`
declarations reference raw `Semantics.Primitives.*` constructors.** Fourth of four atomic tickets
migrating Lean declarations into `plugins_v3/builtin/`
([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5). Standard
constraints apply.

## Scope

1. **Extract 1-to-1 AST constructor bindings into `plugins_v3/builtin/primitives/`:**
   - Move direct surface wrappers over a single `Semantics.Primitives.<Type>.<ctor>` constructor into
     `plugins_v3/builtin/primitives/{action,ability,condition,cost,determiner,event,mana,noun,pile,predicate,pronoun,quantity,static,timing,zone}/<name>.lean`
     using the `primitive` command macro.

2. **Extract composite phrasings into `plugins_v3/builtin/templates/`:**
   - Move all composite phrasing helpers (`gainLife`, `loseLife`, `die`, `enter`, `leave`,
     `entersTapped`, `duringNextTurnOf`, `lookAndSortSlice`, etc.) into
     `plugins_v3/builtin/templates/{ability,ability_word,action,amount,condition,cost,determiner,duration,event,mana,noun,pile,predicate,pronoun,quantity,timing,zone}/<name>.lean`
     using the `template` command macro.
   - Omit manual `firstMention` / `laterMention` in `duringNextTurnOf` and `lookAndSortSlice`
     (handled automatically by `template` on multi-mentioned `NounPhrase` parameters).

3. **Enforce the `primitive` constructor boundary and delete `Templates.lean`:**
   - Enforce in `Authoring.lean` that non-`primitive` declarations (`template`, `action`, `keyword`,
     `counter`, `subtype`, `designation`, `card_type`, `relation`, `token`, `sba`, `conferral`,
     `card`) do not directly reference `Semantics.Primitives.*` constructors.
   - Delete `lean/Semantics/Templates.lean`; `lean/Semantics/Definitions.lean` now loads the entire
     builtin vocabulary via `load_bundle "../plugins_v3/builtin"`.

## Proof

- `lean/Semantics/Templates.lean` and all `lean/Semantics/Definitions/*.lean` submodules are deleted;
  every builtin `primitive`, `template`, `definition`, and rule row lives as an individual `.lean`
  file under `plugins_v3/builtin/{primitives,definitions,templates,rules}/`.
- Non-`primitive` declarations are gated against referencing raw `Semantics.Primitives.*` constructors.
- `cargo xtask lean check` (`lake build --wfail`) exits 0 with zero errors and zero warnings across
  `Semantics`, `SemanticsCards`, and `SemanticsProofs`.

## Implemented inventory

The source at migration time contained 816 `semantic_template` declarations, including the
private `zoneChange` helper; current declarations take precedence over earlier example lists.
The migration preserves that inventory and adds 109 constructor bindings needed by the existing
builtin definitions to cross the primitive boundary. Core coercions, keyword/deed elaborators,
and `Cost.asPaid` remain Lean machinery in `Semantics.Primitives`, with the authored vocabulary
loaded into `Semantics.Templates` by `Semantics.Definitions`.

New files by category (existing bundle files are excluded):

| Category | Primitives | Templates |
| --- | ---: | ---: |
| ability | 25 | 60 |
| action | 133 | 74 |
| amount | 0 | 11 |
| condition | 24 | 10 |
| cost | 23 | 6 |
| determiner | 11 | 17 |
| duration | 0 | 13 |
| event | 19 | 8 |
| mana | 27 | 11 |
| noun | 49 | 31 |
| pile | 5 | 3 |
| predicate | 84 | 35 |
| pronoun | 18 | 5 |
| quantity | 62 | 1 |
| static | 81 | 0 |
| timing | 28 | 11 |
| zone | 35 | 5 |

Each bundle file has one command and no module headers. The six status helpers carry nested
lexical metadata from `statuses.ron`; the source inventory contained no new ability-word catalog
or turn-part declarations. Fixed phrasings use angle-bracket parameters. Structural combinators,
optional clauses and inflected wording that plain string substitution cannot express retain
an explicit `TODO(spelling)` inside their declaration blocks.

The constructor-boundary proofs cover qualified primitive references, anonymous AST constructors,
action bodies, parameter defaults and constructor abbreviations, plus a primitive-to-template
success case and branch-local mention generation. Existing
`annotated`, `token`, `paymentArgument` and actor-scope fixtures use the corresponding primitive
helpers. The Static refusal fixture writes its subject explicitly to keep testing the
Action-versus-Replacement mismatch independently of coercion printing. No tests are retired.
