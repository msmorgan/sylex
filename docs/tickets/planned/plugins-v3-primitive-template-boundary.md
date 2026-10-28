---
needs: [plugins-v3-builtin-cleanup]
---
# [design] Settle the primitive/template boundary in `plugins_v3/builtin/primitives`

Standard constraints apply. This ticket needs a design dialogue before implementation.
`docs/decisions/semantics-v3.md` §4.5 says `primitives/` holds "1-to-1 bindings over
`Semantics.Primitives.<Type>.<ctor>`" and `templates/` holds composite phrasings. It does not
say which side a binding with fixed arguments belongs on. The extraction
(`uvnrnvqq`) left two families of files that depend on that answer.

## What is there

- **About 110 primitives with a sort prefix**, such as `abilityActivated`,
  `abilityTriggered`, `actionMove`, `actionChoose`, `staticSpecCant`,
  `staticSpecConjunction`, `conditionDid`, `predicateDid` and `nounPhrasePro`.
  - These are the full-arity bindings, and usually the only binding for their constructor.
  - The unprefixed names next to them are narrower. `activated` takes `(cost, action)` and fixes
    four `none`s, while `abilityActivated` takes all six. `choose` takes 2 of the 4 arguments.
    `conjunction` swaps the argument order.
  - `move`, `did` and `pro` have no unprefixed primitive at all.
  - Prefixing was most likely how the extraction avoided collisions. Renaming to the bare
    constructor name collides on `(name, return_type)` with existing templates
    (`conditionDid`/`predicateDid` against `template did : Condition/Predicate`,
    `staticSpecCant` against `template cant : StaticSpec`, whose body calls it).
  - The prefix pattern also catches some names whose constructor is genuinely sort-named
    (`doorOf`, `abilityWord`, `keywordParamCost`, `detPhraseA`).
- **About 149 primitives that apply one constructor with some arguments fixed:**
  - `graveyard` is `Location.zone .graveyard none` and `oncePerTurn` is `UsageLimit.mk 1 .turn`.
  - `when`, `whenever` and `at_` have identical `Ability.triggered` bodies.
  - The `activatedOnly*` family fixes slots of `Ability.activated`.
  - Others include `isTapped`, `it`, `topOf` and `controllerOf`.

## Questions to settle

1. Is a binding with fixed arguments a primitive or a template? If it is a template, every
   constructor needs exactly one full-arity primitive, which probably means un-prefixing the
   wrappers.
2. What bare name does a full-arity primitive take when a template already holds
   `(name, return_type)`? Options:
   - Rename the template (for example, `cant` is the phrasing and the primitive is
     `staticCant`).
   - Let primitives live in a separate key space.
   - Accept the prefixes as policy, and then write the rule down.
3. `when`, `whenever` and `at_` give one meaning three names. Keep them as spelling attachment
   points (templates), or collapse them?

## Once settled

- Record the ruling in `semantics-v3.md` §4.5.
- Apply it with a script, checked against the expansion baseline (meaning preserved; keys
  renamed per the ruling).
- Move the files that change side between `primitives/` and `templates/`.
- Keep or add doc comments on the files this ticket touches (`plugins-v3-builtin-doc-comments`
  runs after it).

## Acceptance

The acceptance criteria are written down as part of the ruling. Whatever they are, they must
be checkable by a script over declaration headers and bodies, not by "etc."

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-primitive-template-boundary/before
```

Once the ruling is settled, the rewrite script writes `docs/evidence/plugins-v3-expansions/plugins-v3-primitive-template-boundary/renames.json`
as a JSON array of `[old_key, new_key]` pairs for every changed declaration key.
Use `[]` if no keys change. Compare after the rewrite:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-primitive-template-boundary/before \
  --rename-file docs/evidence/plugins-v3-expansions/plugins-v3-primitive-template-boundary/renames.json
```

Require zero differences; accepted renames have their own count. Any proposed
meaning change needs the design ruling before an explicit `--expect` allowance.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.

Declaration rename maps also normalize `<decl>.<param>` and `<decl>.actor` mention
IDs in callers, simultaneously, so swaps and chains work. Renaming a repeated
`NounPhrase` parameter changes `<decl>.<param>` IDs in callers; those changes
remain visible and require review and explicit expectations where intended.
