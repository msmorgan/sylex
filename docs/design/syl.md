# The `.syl` Data Language

## Status: DRAFT

This document is directional. It describes the `.syl` language the project is aiming at, and
the design is not settled: read it for the intended shape, not as a contract the code keeps.
`sylex_syl` parses and formats the syntax and checks the declaration rules today; resolution by
sort and expansion exist only as throwaway tools in `xtask` (`dsl check`, `dsl expand`).

- **Companion:** [`semantics.md`](./semantics.md), the semantic model `.syl` terms are read in.

## Overview

`.syl` is the language in which the vocabulary and cards under `plugins/` are written. This document
says how a term is written, named and expanded; what a term may say is
[`surface.md`](./surface.md); what the term means, and how its references
resolve, is [`semantics.md`](./semantics.md). A file is parsed (§2), each name is resolved by the
sort expected where it stands (§3), and Templates and Definitions are expanded (§4). The expanded
term is then read in printed order (§5) by semantics ([`semantics.md`](./semantics.md)
§2, §4).

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
  letter for letter as the Rust AST's variant name: `primitive activated(cost: Cost, action: VerbPhrase):
  Ability = Activated { … }`. The clause is required even when it only capitalizes the
  primitive's own name (`primitive power: Stat = Power`). Every constructor and enum word a
  file uses has its primitive, and each constructor has exactly one, at its result sort; a
  primitive with an `at` list (§2) builds its constructor at each of its sorts. No
  primitive builds a constructor that only expansion writes (§4). Fixed
  arguments and helpers belong in Templates over it.
  Why: without primitives, there is nowhere to attach a constructor's spelling. The wording would
  have to live somewhere other than the declaration that binds the word, which makes a second
  source of truth for spelling. A primitive is the one explicit mapping from a word to the AST, so
  every word can be traced to what it builds, and the AST's names appear nowhere else.
  `sylex_syl::check` refuses a primitive without a constructor clause, a constructor clause on
  any other declaration, and two primitives for one constructor at one sort.
- **A Template** substitutes its parameters (§4). What that means, and why it adds no meaning, is
  [`surface.md`](./surface.md) §6.
- **A Definition** wraps its parameters and its body (§4). What it carries, a Deed and its own
  Reference Scope, is [`surface.md`](./surface.md) §6.

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
- **No conversions besides `Option`.** A value stands at its own sort or at a sort its sort is a
  member of (§3), written as itself, and every wrap into another sort is a primitive or a Template
  written at the call site, `actor(…)` included. Why: a conversion hides which word was meant.
- **A sort may have member sorts.** `member(Color): AdjectivePhrase` declares that every value of
  `Color` is a value of `AdjectivePhrase`, written as itself: a membership has no name and no
  body, and builds nothing. It is transitive. The memberships are in `plugins/builtin/members/`:

  | Sort | Members |
  |---|---|
  | `PreModifier` | `AdjectivePhrase`, `Nominal`, `Participle` |
  | `PostModifier` | `PrepositionPhrase`, `RelativeClause`, `AdjectivePhrase`, `Participle` |
  | `Predicative` | `AdjectivePhrase`, `PrepositionPhrase`, `NounPhrase` |
  | `AdjectivePhrase` | `Color`, `Supertype` |
  | `Nominal` | `CardType`, `Subtype` |
  | `Amount` | `Int` |

  So the bare word, as printed (`creature`, `goblin`, `basic`, `white`, `3`), is the one written
  form of each such term wherever it stands. Under each sort its members form a tree: load refuses
  a cycle, and a sort that reaches another by two paths. `Status` is a member of nothing: it is the
  sort of the game's statuses, which `setStatus` and the test `hasStatus` take, and each wording
  ("tapped", "face down", "face-down") is a Template at its kind of phrase.
  `DesignationLabel` is a member of nothing either, since labels such as `monarch` are no
  describing phrases. A `ColorOrColorless` or a mana symbol wraps a `Color`, and a mana symbol is
  always written (`pip`, `colorlessPip`, `generic`), since a color word is not a mana symbol.
- **A kind of describing phrase denotes a test.** `denotes(AdjectivePhrase): Test` declares that a
  body may write a `Test` where an `AdjectivePhrase` is expected and an `AdjectivePhrase` where a
  `Test` is (§3). `AdjectivePhrase`, `PrepositionPhrase`, `RelativeClause` and `Participle` denote
  `Test`; `Nominal`, `NounPhrase` and `VerbPhrase` denote nothing.
- **A declaration may stand at several sorts.** Its block lists them, `at = [AdjectivePhrase,
  Nominal, Participle]`, and its signature writes `Self` for the sort it stands at:
  `template or(items: List(Self)): Self`. It is one declaration, checked once for each sort of
  the list with `Self` that sort. The coordinators (`or`, `and`, `andOr`, `both`, `coord`) and the
  negations (`non`, `not`) of describing phrases, nominals and noun phrases are declared so; §3
  says where a call of one stands.
- **Reads are marked.** `$name` reads a parameter; a bare word never does. Why: what a word refers
  to can be read from its spelling, and a parameter can never shadow a word.
- **Each argument is bound one way.** A required argument is positional and binds the next required
  parameter not yet bound, in declaration order, even past an optional parameter declared before it.
  A defaulted argument is passed by name, and never with the value its parameter declares as its
  default. Arguments
  are written in declaration order, which is printed order (§5), a named argument in its declared
  place among the positional ones: `fromEnd(bottom, ordinal = nth($n), $loc)`. `cargo xtask dsl
  check` refuses a required argument passed by name, a defaulted one passed positionally, a named
  argument equal to its default, and arguments out of declaration order. Why: a reader can tell
  which parameter each argument fills, and a call's arguments have one binding, naming and order.
- **Notation.** `=` binds and `:` means "has sort". A capitalized word is a sort in a signature
  (`other: NounPhrase`, `Option(List(Cost))`), in a membership or in an `at` list, or a
  constructor in a primitive's constructor clause, and nothing else; `sylex_syl`'s parser refuses one anywhere else. The rule does not
  apply inside `lexical` and `spelling`, which are metadata. Why: a capitalized word is the AST's
  own name, so where one stands, the AST is named, and nowhere else is.
- **One block per declaration** holds three layers: `lexical`, `spelling` and the meaning (`body`,
  `confers`, `rules`). Why: a word's frames, spelling and meaning are one fact, so they have one
  source. Keyword `parameters` are spelling frames and live in `lexical`. A Primitive Keyword
  Ability is the exception: its `lexical` is on its label primitive at `KeywordLabel`, and its
  `spelling` and meaning (`primitiveKeyword(firstStrike)`) are on its Template at `Ability` (§3).
- **A string has three escapes**: `\"` for a quotation mark, `\\` for a backslash and `\n` for a
  line break. Any other is refused. The lexer also reads a line break typed in a string, and the
  printer writes every line break as `\n`, so a string has one written form once formatted,
  including the line-break join that puts each mode of a `spelling` on its own line. Why: a line
  break typed inside a string is ugly and easy to miss.
- **A `spelling` is written in the notation of [`spelling.md`](./spelling.md) §7**, where a Hole
  is `$name`, the sigil that reads the same parameter in `body`.
- **Citations live in comments.** There is no `rule` field; `cargo xtask cite` checks the comments.
- **Paths are for people.** Folder and file names never carry meaning.

## 3. Name Resolution by Expected Sort

One printed word may build different sorts (`and`, `target`, `did`), so names are keyed by
`(name, sort)` instead of being qualified. No two declarations share a `(name, sort)`.

**A term is resolved in one of two modes.**

- **Surface mode** resolves a `card`'s term. At expected sort `T` a value stands at `T` or at a
  sort that reaches `T` through membership (§2), and nowhere else.
- **Body mode** resolves everything else: the body of a Template, a Definition or a rule, and a
  parameter's default. It adds `denotes` (§2): a slot typed by a kind that denotes `Test`, or by a
  position sort ([`surface.md`](./surface.md) §1), also admits a `Test`, and a slot typed `Test`
  also admits a phrase of such a kind. So `colorless`, an `AdjectivePhrase`, has the body
  `colorCount(equal, 0)`, and a rule body writes `post = [hasPossessor(controller, $player)]`.

Reaches(`T`) is the set of sorts a value expected at `T` may stand at. In surface mode it is `T`
and every sort that reaches it through `member`. In body mode, Reaches(`Test`) is `Test` and the
surface Reaches of each kind that denotes it; Reaches of such a kind, or of a position sort, is
its surface Reaches and `Test`; Reaches of any other sort is its surface Reaches. In either mode
it is a tree under `T`.

At expected sort `T`, a value resolves in this order, with no backtracking:

1. `none`, if `T` is optional;
2. what stands at `T` itself: for `$name`, a parameter of the enclosing declaration whose sort is
   `T`; for a word, the declaration `(name, T)`; for a literal, `T` if it is the literal's own
   sort;
3. if `T` is `Option(U)`, the value at `U`, by steps 2 and 4;
4. at the one sort in the mode's Reaches(`T`) that the value stands at: a word's declaration, a
   parameter's sort, a literal's own sort.

A word that stands at no sort of Reaches(`T`) is refused; no word reaches a constructor except
through its primitive (§1). A declaration's kind may declare its
name at a second sort: an `ability` declaration is also its keyword's `KeywordLabel`, so a keyword
is named bare where a label is expected (`byKeyword(kicker)`). That is a second declaration, not a
conversion; the keyword's term is written `theKeyword(kicker)`. A Primitive Keyword Ability has no
`ability` declaration ([`surface.md`](./surface.md) §6): its label is a primitive at
`KeywordLabel` with no parameters that builds the constructor its own name capitalizes
(`primitive firstStrike: KeywordLabel = FirstStrike`), the only `KeywordLabel` constructor a
primitive may build, and a Template at `Ability` of the same name wraps the label
(`primitiveKeyword(firstStrike)`). `sylex_syl::check` refuses a word that names no declaration at
any sort; the resolver refuses one that stands nowhere in Reaches(`T`).

**Optional positions.** A value at an `Option(T)` position is written as a `T`, `some` is never
written, and an omitted argument or field takes its declared default. Membership applies there as
at `T`: `power = 2` at `Option(Amount)`.

**One word, one sort in a Reaches set.** Two declarations of one word at two sorts of one Reaches
set, in either mode, are refused: at both `CardType` and `Subtype`, or at `Nominal` beside a card
type of the same name, or at a kind and at `Test`. For the same reason a word declared at
`Option(T)` and at a sort that reaches `T` is refused, since at an `Option(T)` position the first
is found before membership is tried. A literal that stands at two sorts of a Reaches set is
refused likewise: an integer when `Int` and `Nat` both reach `T`, a record when two record sorts
do, a list when two list sorts do. One declaration with an `at` list (§2) is one declaration at
all its sorts and is no such pair. These checks run at load, across the whole registry, so a
latent collision fails before any call site writes the word. A word at two sorts no Reaches set
joins, such as `snow` at `Supertype` and at `ManaSymbol`, `tapped` at `Status` and at
`AdjectivePhrase`, `faceDown` at `PreModifier` ("face-down") and at `PrepositionPhrase` ("face
down"), or a `List(CardType)` Template named `creature`, is no collision.

**A call of a word with an `at` list stands at the lowest of its sorts that holds every arm.**
An arm is an argument of a parameter typed by `Self`, or an item of the list written for a
parameter typed `List(Self)`. Each arm has one sort: its head word's, which is unique in
Reaches(`T`); a parameter's declared sort, with a list parameter passed whole giving its element
sort; or, for a nested call of a word with an `at` list, the sort that call stands at.
Reaches(`T`) is a tree, so the arms' sorts have one lowest common sort, and the call stands at
the lowest sort of the word's `at` list that is at or above it. Its arms are then resolved at that
sort. In body mode, when the list has no such sort and the word stands at `Test`, the call stands
at `Test` and its arms resolve at expected sort `Test`: `youDontControl`'s `not(controls(you))`,
since `not` does not stand at `RelativeClause`. A call with no arm, and one whose arms no sort of
the list holds, are refused.

- `or([white, blue])` in `pre` or after "that's": both arms are colors, so it stands at
  `AdjectivePhrase`, where the English grammar parses "white or blue" in both places (*Fry*).
- `or([knight, aura, equipment, nominal(pre = [legendary], artifact)])` in `pre`: the arms are
  subtypes and a nominal, so it stands at `Nominal` (*Acclaimed Contender*).
- `or([black, artifact])` in `pre` stands at `PreModifier`, where the grammar puts a coordination
  of unlike kinds (*Soldevi Adnate*).

This is the one place resolution reads a call's arguments. It stays one pass, because an arm's
sort never depends on the coordinator's.

**A test in a card.** No surface sort reaches `Test`, so a card cannot write a test, and nothing
at `Test` has a `spelling` (load refuses one, apart from a word that stands at `Test` as one sort
of its `at` list). Where a card's term calls a test because no declaration yet spells the phrase
the card prints, `cargo xtask dsl check` names the card and the test as an exception
(`schema::TEST_EXCEPTIONS`), each until `syl-spelled-test-phrases` writes its phrase. An exception
covers the smallest argument of a slot typed by a position sort that holds the call, and that
argument resolves in body mode: *Glaring Spotlight*'s `is(…, not(hasKeyword(…)))` keeps its `not`
at `Test`.

**Literals** stand only at their own sorts: a string at `String`, an integer at the integer sorts
(`Int`, and `Nat` when it is not negative). There are no direct sort atoms: a literal or a
declared word stands at another sort only through membership, or wrapped by a written primitive
or Template (§2).

## 4. Expansion

- Every parameter is a binding, a Template's or a Definition's, at any sort. Its argument is read
  once, at the parameter's first use in read order; each later use in the body re-mentions that
  read and never reads the argument again. The binding is generated, never hand-written, so a
  value the source writes once is one read in the expanded tree, and targets are not duplicated
  ([CR#115.1]).
- Templates substitute their parameters. A Template adds no Reference Scope and no Deed, so its
  body's references reach the referents at its call site. A body is resolved in body mode (§3),
  whatever its declaration's kind, so after expansion a describing phrase is a `Test` or a member
  word standing as itself (a color, a supertype, a type word).
- A Definition wraps every parameter for caller-scope reading, with no opt-out. Every `action`
  Definition has an implicit parameter `$actor` at `NounPhrase`, bound once at the call to the
  call's performer; a Definition's body reads its call's performer through `$actor`, whereas the
  `actor` word reads the performer of the verb phrase it stands in and rebinds under a Handoff
  inside the body. A Definition wraps its body in its Reference Scope under its Deed
  ([`semantics.md`](./semantics.md) §2.3).
- So Templates and Definitions bind parameters alike; in expansion they differ only in Reference
  Scope and Deed.
- `.syl` cannot write a scope, a mention, a raw Deed or a keyword wrapper.
- Expansion keeps every call and parameter boundary and marks no use as first or later. Which use
  of a parameter is its first mention is decided by reference resolution, in read order (§5). Why:
  the first mention is the one that introduces referents, so it must be the use actually read
  first. It is never decided by written argument order. For example, a passive `by(np)` reads the
  patient before `np` ([`surface.md`](./surface.md) §7.2).
- Sorts are checked per slot, before expansion: `cargo xtask dsl check` checks each argument
  against its parameter's declared sort at the call, and each `$name` in a declaration against the
  sort of the slot it stands in, which must be the parameter's sort, `Option` of it, or a sort the
  parameter's sort reaches (§3).

## 5. Read Order

Read order is printed order. A declaration's parameters are read in the order its spelling prints
them, with no per-declaration exception: where the printed order differs ("When $event, if
$condition, $action"), the declaration's parameters are reordered to match. Why: reference
resolution sees what the text says first as first, spelling renders a declaration in its parameter
order, and a reference oracle keyed to printed text lines up with the term only if the two orders
agree.

The elements of a list are read in printed order too: a nominal's modifier lists and a sequence's
steps stand in the order the text prints them, so "other nonland permanents" has
`[other, non(land)]` in `pre`, and "basic land" is `nominal(pre = [basic], land)`.
