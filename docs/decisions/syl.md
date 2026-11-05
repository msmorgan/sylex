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
  no coercions, no type inference (§2).
- **A name is found by its expected sort**, not by qualification (§3).
- **Expansion, not the author, supplies scopes, mentions and bindings** (§4).
- **Read order is printed order** (§5).

## 1. Three Kinds of Declaration

The language has three kinds of named word, because there are three different jobs: attaching a
constructor's spelling, adding printed wording, and adding vocabulary the rules define.

- **A primitive** is where a model constructor's spelling is attached. It is the one
  declaration that binds a word to a constructor, and it does the wrapping: it names the
  constructor the word builds. Fixed arguments and helpers belong in Templates over it.
  Why: without primitives, there is nowhere to attach a constructor's spelling. The wording would
  have to live somewhere other than the declaration that binds the word, which makes a second
  source of truth for spelling.
- **A Template** substitutes its parameters (§4). What that means, and why it adds no meaning, is
  [`semantics.md`](./semantics.md) §3.
- **A Definition** wraps its parameters and its body (§4). What it carries, a Deed and its own
  Reference Scope, is [`semantics.md`](./semantics.md) §3.

A declaration's kind decides which it is; its file path never does. Why: a role inferred from
location is not stated by the declaration, and it can disagree with what the declaration is.

## 2. Files and Syntax

- **First-order data.** Every declaration body is a tree of calls. There is no evaluation, there
  are no local bindings or higher-order parameters, and there is no type inference. Why: every
  tool (the Rust pipeline, the Lean generator, the formatter, the citation checker) can read a file
  without an evaluator, and what a file says is what it means.
- **No coercions.** Only the direct sort atoms of §3 resolve implicitly. Every wrapper is written,
  `actor(…)` included. Why: a coercion hides which word was meant. Uglier but simpler is the
  point; coercions may return deliberately once the language is simple enough to refactor.
- **Reads are marked.** `$name` reads a parameter; a bare word never does. Why: what a word refers
  to can be read from its spelling, and a parameter can never shadow a word.
- **Argument order carries no meaning.** Named arguments may stand anywhere, so a call can follow
  printed order.
- **Notation.** `=` binds and `:` means "has sort". A leading `.` appears only where a primitive
  names its constructor.
- **One block per declaration** holds three layers: `lexical`, `spelling` and the meaning (`body`,
  `confers`, `rules`). Why: a word's frames, spelling and meaning are one fact, so they have one
  source. Keyword `parameters` are spelling frames and live in `lexical`.
- **Citations live in comments.** There is no `rule` field; `cargo xtask cite` checks the comments.
- **Paths are for people.** Folder and file names never carry meaning.

## 3. Name Resolution by Expected Sort

One printed word may build different sorts (`and`, `target`, `did`), so names are keyed by
`(name, sort)` instead of being qualified. No two declarations share a `(name, sort)`. At expected
sort `T`, a word resolves in constant time, with no backtracking, in this order:

1. `$name`: a parameter of the enclosing declaration;
2. `none`, if `T` is optional;
3. the declaration `(name, T)`;
4. a constructor or enum word of `T`.

Declarations therefore shadow constructors.

**Optional positions.** A value at an `Option(T)` position is written as a `T`, `some` is never
written, and an omitted argument takes its declared default. This is the one implicit wrap the
language keeps.

**Direct sort atoms** are integers; color words; and keyword names at `Ability`, `KeywordLabel`,
`KeywordTerm`, `Action` and `Deed`.

## 4. Expansion

- Templates substitute their parameters. A Template that mentions a NounPhrase parameter more than
  once binds it at the first occurrence. The binding is generated, never hand-written, so targets
  are not duplicated ([CR#115.1]).
- A Definition wraps every parameter for caller-scope reading, with no opt-out. It wraps its body
  in its Reference Scope under its Deed ([`semantics.md`](./semantics.md) §2.3).
- `.syl` cannot write a scope, a mention, a raw Deed or a keyword wrapper.
- Which occurrence of a parameter is its first mention is decided after expansion, in the order
  the checker reads. Why: the first mention is the one that introduces referents, so it must be
  the occurrence actually read first. It is never decided by written argument order. For example,
  a passive `by(np)` reads the patient before `np` ([`semantics.md`](./semantics.md) §4.2).

## 5. Read Order

Read order is printed order. A declaration's parameters are read in the order its spelling prints
them, with no per-declaration exception: where the printed order differs ("When <event>, if
<condition>, <action>"), the declaration's parameters are reordered to match. Why: reference
resolution sees what the text says first as first, spelling renders a declaration in its parameter
order, and a reference oracle keyed to printed text lines up with the term only if the two orders
agree.
