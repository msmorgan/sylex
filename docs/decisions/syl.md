# ADR: The `.syl` Data Language

- **Companion:** [`semantics.md`](./semantics.md), the semantic model `.syl` terms are read in.

## Overview

`.syl` is the language in which the vocabulary and cards under `plugins/` are written. This ADR
says how a term is written, named and expanded; what the term means, and how its references
resolve, is [`semantics.md`](./semantics.md). A file is parsed (§2), each name is resolved by the
sort expected where it stands (§3), and Templates and Definitions are expanded (§4). The expanded
term is then read in printed order (§5) by the discourse reading ([`semantics.md`](./semantics.md)
§2).

What sets the language apart, each in its own section:

- **Each word's spelling and meaning have one source.** A primitive is a constructor's only
  binding (§1), and one block per declaration holds its frames, spelling and meaning (§2).
- **What a file says is what it means.** It is first-order data: no evaluation, no local bindings,
  no conversions besides `Option`, no type inference (§2).
- **A name is found by its expected sort**, not by qualification (§3).
- **Expansion, not the author, supplies scopes, mentions and bindings** (§4).
- **Read order is printed order** (§5).

## 1. Three Kinds of Declaration

The language has three kinds of named word, because there are three different jobs: attaching a
constructor's spelling, adding printed wording, and adding vocabulary the rules define.

- **A primitive** is where a model constructor's spelling is attached. It is the one
  declaration that binds a word to a constructor, and it does the wrapping: its constructor
  clause, after its signature, names the constructor the word builds, capitalized and spelled
  letter for letter as the Rust AST's variant name: `primitive activated(cost: Cost, action: Action):
  Ability = Activated { … }`. The clause is required even when it only capitalizes the
  primitive's own name (`primitive power: Stat = Power`). Every constructor and enum word a
  file uses has its primitive, and each constructor has exactly one, at its result sort. No
  primitive builds a constructor that only expansion writes (§4). Fixed
  arguments and helpers belong in Templates over it.
  Why: without primitives, there is nowhere to attach a constructor's spelling. The wording would
  have to live somewhere other than the declaration that binds the word, which makes a second
  source of truth for spelling. A primitive is the one explicit mapping from a word to the AST, so
  every word can be traced to what it builds, and the AST's names appear nowhere else.
  `sylex_syl::check` refuses a primitive without a constructor clause, a constructor clause on
  any other declaration, and two primitives for one constructor.
- **A Template** substitutes its parameters (§4). What that means, and why it adds no meaning, is
  [`semantics.md`](./semantics.md) §3.
- **A Definition** wraps its parameters and its body (§4). What it carries, a Deed and its own
  Reference Scope, is [`semantics.md`](./semantics.md) §3.

A declaration's kind decides which it is; its file path never does. Why: a role inferred from
location is not stated by the declaration, and it can disagree with what the declaration is.

## 2. Files and Syntax

- **First-order data.** Every declaration body is a tree of calls. There is no evaluation, there
  are no local bindings or higher-order parameters, and there is no type inference. Why: every
  tool (the Rust pipeline, the formatter, the citation checker) can read a file
  without an evaluator, and what a file says is what it means.
- **A value is written once per printed mention.** A value the text prints once is written once,
  through a Template whose wording prints it once, so a Template parameter is the binding:
  "gets +1/+1 for each Forest you control" is `getsBoth`'s one delta, not `getsPt`'s two copies.
  A value the text prints twice is written twice. Why: without local bindings, a copy the text
  does not print is a second source for one value, and the two can drift apart.
- **No conversions besides `Option`.** A value stands only at its own sort (§3), and every wrap
  into another sort is a primitive or a Template written at the call site, `actor(…)` included: a
  color word is a `Color`, and a `ColorOrColorless` or a mana symbol wraps it; an integer literal
  is an integer, and an `Amount` or a generic mana symbol wraps it. Mana-symbol wrappers are
  Templates over primitive chains. Why: a conversion hides which word was meant.
  Uglier but simpler is the point; conversions may return deliberately once the language is
  simple enough to refactor.
- **Reads are marked.** `$name` reads a parameter; a bare word never does. Why: what a word refers
  to can be read from its spelling, and a parameter can never shadow a word.
- **Argument order carries no meaning.** Named arguments may stand anywhere, so a call can follow
  printed order.
- **Notation.** `=` binds and `:` means "has sort". A capitalized word is a sort in a signature
  (`other: NounPhrase`, `Option(List(Cost))`) or a constructor in a primitive's constructor
  clause, and nothing else; `sylex_syl`'s parser refuses one anywhere else. The rule does not
  apply inside `lexical` and `spelling`, which are metadata. Why: a capitalized word is the AST's
  own name, so where one stands, the AST is named, and nowhere else is.
- **One block per declaration** holds three layers: `lexical`, `spelling` and the meaning (`body`,
  `confers`, `rules`). Why: a word's frames, spelling and meaning are one fact, so they have one
  source. Keyword `parameters` are spelling frames and live in `lexical`. A Primitive Keyword
  Ability is the exception: its `lexical` is on its label primitive at `KeywordLabel`, and its
  `spelling` and meaning (`primitiveKeyword(<label>)`) are on its Template at `Ability` (§3).
- **Citations live in comments.** There is no `rule` field; `cargo xtask cite` checks the comments.
- **Paths are for people.** Folder and file names never carry meaning.

## 3. Name Resolution by Expected Sort

One printed word may build different sorts (`and`, `target`, `did`), so names are keyed by
`(name, sort)` instead of being qualified. No two declarations share a `(name, sort)`. At expected
sort `T`, a word resolves in constant time, with no backtracking, in this order:

1. `$name`: a parameter of the enclosing declaration;
2. `none`, if `T` is optional;
3. the declaration `(name, T)`.

A word that names no declaration at `T` is refused; no word reaches a constructor except through
its primitive (§1). A declaration's kind may declare its name at a second sort: an `ability`
declaration is also its keyword's `KeywordLabel`, so a keyword is named bare where a label is
expected (`byKeyword(kicker)`). That is a second declaration, not a conversion; the keyword's
term is written `theKeyword(kicker)`. A Primitive Keyword Ability has no `ability` declaration
([`semantics.md`](./semantics.md) §3): its label is a primitive at `KeywordLabel` with no
parameters that builds the constructor its own name capitalizes (`primitive firstStrike:
KeywordLabel = FirstStrike`), the only `KeywordLabel` constructor a primitive may build, and a
Template at `Ability` of the same name wraps the label (`primitiveKeyword(firstStrike)`).
`sylex_syl::check` refuses a word that names no declaration at any sort; the resolver refuses one
that names none at `T`.

**Optional positions.** A value at an `Option(T)` position is written as a `T`, `some` is never
written, and an omitted argument or field takes its declared default. This is the one implicit
wrap the language keeps.

**Literals** stand only at their own sorts: a string at `String`, an integer at the integer sorts
(`Int`, and `Nat` when it is not negative). There are no direct sort atoms: a literal or a
declared word at any other sort is wrapped by a written primitive or Template (§2).

## 4. Expansion

- Every parameter is a binding, a Template's or a Definition's, at any sort. Its argument is read
  once, at the parameter's first use in read order; each later use in the body re-mentions that
  read and never reads the argument again. The binding is generated, never hand-written, so a
  value the source writes once is one read in the expanded tree, and targets are not duplicated
  ([CR#115.1]).
- Templates substitute their parameters. A Template adds no Reference Scope and no Deed, so its
  body's references reach the referents at its call site.
- A Definition wraps every parameter for caller-scope reading, with no opt-out. It wraps its body
  in its Reference Scope under its Deed ([`semantics.md`](./semantics.md) §2.3).
- So Templates and Definitions bind parameters alike; in expansion they differ only in Reference
  Scope and Deed.
- `.syl` cannot write a scope, a mention, a raw Deed or a keyword wrapper.
- Expansion keeps every call and parameter boundary and marks no use as first or later. Which use
  of a parameter is its first mention is decided by reference resolution, in read order (§5). Why:
  the first mention is the one that introduces referents, so it must be the use actually read
  first. It is never decided by written argument order. For example, a passive `by(np)` reads the
  patient before `np` ([`semantics.md`](./semantics.md) §4.2).
- Sorts are checked per slot, before expansion: `cargo xtask dsl check` checks each argument
  against its parameter's declared sort at the call, and each `$name` in a declaration against the
  sort of the slot it stands in, which must be the parameter's sort or `Option` of it (§3).

## 5. Read Order

Read order is printed order. A declaration's parameters are read in the order its spelling prints
them, with no per-declaration exception: where the printed order differs ("When <event>, if
<condition>, <action>"), the declaration's parameters are reordered to match. Why: reference
resolution sees what the text says first as first, spelling renders a declaration in its parameter
order, and a reference oracle keyed to printed text lines up with the term only if the two orders
agree.
