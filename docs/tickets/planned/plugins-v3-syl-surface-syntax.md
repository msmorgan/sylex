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

## Questions for the design dialogue

1. **Call and argument syntax.** Curried juxtaposition (`can (actor you) (cast this (from :=
   yourGraveyard))`) requires parentheses around every non-atomic argument. Should `.syl` keep
   juxtaposition or switch to explicit call parentheses/commas (`can(actor(you), cast(this, from:
   yourGraveyard))`) or another concrete form?
2. **Declaration headers and `{ body := … }` wrapper.** How are declaration headers, parameter
   binders with defaults, return sorts, and the metadata/meaning sections (`lexical`, `spelling`,
   `body`/`confers`/`rules`) written?
3. **Binding punctuation.** What token binds record fields, named arguments, and parameter defaults
   (`:=`, `:`, or `=`)?
4. **Unescaping Lean reserved words.** `in_`, `from_`, `by_`, `or_`, `this_`, `exists_`, `where_`,
   and `«variable»` only carry `_` or `«…»` because of Lean 4's keyword table. Confirm that `.syl`
   uses the plain words (`in`, `from`, `by`, `or`, `this`, `exists`, `where`, `variable`), with the
   Lake generator quoting any word that collides with Lean when emitting `.lean` modules.
5. **Comments and CR citations.** Replace Lean's `/-- … -/` and `--` or keep them? Update
   `cargo xtask cite` so `.syl` comments are scanned and audited like `.lean` and `.rs`.
