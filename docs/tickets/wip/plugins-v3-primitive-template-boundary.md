---
needs: [plugins-v3-builtin-cleanup]
---
# Settle the primitive/template boundary in `plugins_v3/builtin/primitives`

Standard constraints apply. The owner settled the boundary on 2026-10-28;
`docs/decisions/semantics-v3.md` §4.5 records the ruling.

## Settled transformation

Apply the verified 622-file partition: 80 bare-name renames, 17 duplicate removals,
137 moves to Templates, six widened duplicate counterparts, three widened fixed-argument
primitives, and 379 retained primitives. Within the retained group, `conclude` and `doIf`
expose their previously omitted default fields. Promote the six deontic/lookback Templates
and add six missing primitives. This leaves 480 files in `primitives/`.

A primitive body directly applies one `Semantics.Primitives.<Type>.<ctor>` to its parameters;
defaults are declared on binders. Fixed-argument expression specializations are Templates.
The promoted `can`, `cant`, `must`, `gatedBy` and both `did` bindings directly assemble their
non-expression `DeonticMods`/`DidClause` data records with the existing field defaults.
`when`, `whenever` and `at_` remain separate spelling attachment points over `triggered`.
The 11 remaining prefixed two-layer pairs are tracked by
`plugins-v3-shadowed-primitive-names`.

`doForEachKind` takes `axis`, `domain := none`, `sort`, then `body`. Its callers must preserve
the domain-before-body order used by mention assignment and checking. Both loaders allow a
non-primitive to depend on a distinct same-bare-name primitive, and ignore leading-dot enum
constructor names when collecting bundle dependencies. Primitive dependencies conservatively retain
matching bound names to make shadowed functions available before elaboration. Expected-type dispatch
tries visible authored alternatives for coercions when no exact result-sort entry matches, unwraps optional
result sorts and distinguishes callable local variables from auxiliary or non-callable locals.

## Checkable acceptance

- Enumerate `.lean` files in `primitives/`: exactly 480, each a single `primitive` declaration
  whose body begins with the direct `Semantics.Primitives` constructor application. Inspect its
  argument syntax against its binders and constructor fields; only the six named record bindings
  above assemble non-expression records. No expression-packing helper or fixed expression argument
  is admitted as a primitive body.
- For all 137 manifest moves, the old path is absent, the new declaration is a `template`, and
  the whitespace-normalized body equals the manifest's `new_body`. The 80 rename pairs are unique
  and update all builtin, canon and Lean callers. The renamed `MoveRider.not` file is in `move/`.
- The 17 deleted keys are absent; the six promoted declarations retain their keys; the six added
  keys are present. Remaining parameter and result types in the 80 renamed and 11 retained
  prefixed bindings omit redundant `Semantics.` prefixes.
- The expansion comparison accepts exactly the 80 renames and 35 expected differences:
  17 deleted duplicates, six new primitives, 11 widened primitives, and the explicitly ordered
  `doForEachKind : Action` signature. No caller or card differences are accepted, including
  `doubleCounters : Action` and `vorelOfTheHullClade : Spelled`.
- Bundle ordering, optional-result dispatch, non-callable local shadowing and callable local
  and ordinary namespace function precedence have regression checks. All affected Rust tests and clippy, the Lean check and
  citation gates pass. Existing proof assertions retain their semantic claims; call-name pins
  follow the generated sort-qualified names.

## Expansion evidence

Keep `docs/evidence/plugins-v3-expansions/plugins-v3-primitive-template-boundary/before/snapshot.json`
unchanged. It contains 2992 declarations and 80536 samples. `renames.json` contains the 80
`[old_key, new_key]` pairs; `expected.txt` contains the 35 exact declaration keys above. These
are ignored evidence, never tracked gate authority.

Use `cargo xtask expansions --compare <before> --rename-file <renames.json>`, adding one
`--expect '<name> : <Type>'` for each expected key. `--out <after>` optionally saves a separate
snapshot. The comparison must exit successfully, reporting `35 differences, 80 renames` with
zero unexpected differences and no failed samples. The sampler is bounded: inspect its omitted
constructors and capped products rather than treating sampling as an exhaustive proof.
