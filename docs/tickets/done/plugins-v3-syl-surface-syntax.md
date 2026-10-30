---
needs: []
---
# [design] `.syl` surface syntax for `plugins_v3`

Standard constraints apply. This ticket settles the concrete surface syntax of `.syl` files before
`plugins-v3-data-language` runs the conversion script, and amends `docs/decisions/semantics-v3.md`
§4.4 with the concrete grammar productions.

## Why

`plugins-v3-data-language` settled the abstract grammar, dispatch, expansion and Lake codegen for
`plugins_v3/**/*.syl`: files are standalone `.syl` data files with 5 expression forms (literals,
list literals, record blocks, calls with positional/named arguments, and bare words), no coercions,
no domain-list sugar, no leading dots, and no Lean term elaboration.

The current files still wear Lean's surface costume (`:=`, curried juxtaposition requiring
parentheses around every nested call, `{ body := … }` wrappers, `/-- … -/` and `--` comments, and
`in_`/`from_`/`this_`/`«variable»` escapes for Lean 4 reserved words). Because
`plugins-v3-data-language` converts every file by script, settling the concrete `.syl` surface form
first lets the conversion happen in a single pass.

## Settled (owner, 2026-10-30)

Recorded in `docs/decisions/semantics-v3.md` §4.4 ("Concrete surface syntax"):

- calls are `head(arg, name = arg)`; `=` binds everywhere (fields, named arguments, defaults) and
  `:` means only "has sort", in headers;
- headers are `kind name(param: Sort = default): Sort { fields }`; only `primitive` and `template`
  write a result sort, after the parameters, and there is no `->`; the `keyword` kind is renamed
  `ability` (`action` and `ability` declare Keyword Actions and Keyword Abilities);
- `lexical`, `spelling`, `body`, `confers` and `rules` are record fields;
- reserved words are written plain (`in`, `from`, `this`, ...) and the Lake generator quotes
  collisions; `none`, `true` and `false` are reserved and resolve before enum words;
- comments are `#` to end of line, doc comments a leading `#` block, with bracketed CR citations;
- no parameter opt-out marker exists (the ADR's `inCaller` ruling).

## Acceptance

- `cargo xtask cite` scans `.syl` `#` comments before the conversion lands.
- The conversion preserves the Lean dump oracle (`lean-elaborated-value-dump`): it changes syntax
  only.
