# Surface

## Status: DRAFT

This document is directional. It describes what a surface term may say: the stored `.syl` term
of a card, which follows the card's English and says nothing about what its words do in the
game. The design is not settled, and [`semantics.md`](./semantics.md) §8 lists what is open.

- **Companions:** [`syl.md`](./syl.md), the language a surface term is written in, and
  [`semantics.md`](./semantics.md), how a surface term is read into its meaning.

The words a surface term is built from are designed in `docs/specs/voice-and-object-phrases.md`
(the clause, the verb phrase and its operators, the object phrases) and in the planned tickets
that build them. What [`semantics.md`](./semantics.md) §3 and §4 say of Definitions, Templates
and the action-clause spine is surface design too, and moves here
(`syl-surface-and-semantics-design-docs`).

## 1. A Sort Names a Kind of Phrase

**A sort names a kind of phrase.** `Clause`, `VerbPhrase`, `NounPhrase`, `Nominal` and its
modifier sorts, `Amount`, a determiner, a duration phrase, a Turn Point are kinds of phrase. So
are the sorts of words and symbols a card prints: a card type, a color, a mana symbol, a counter
kind. A sort that says what a phrase does in the game is no kind of phrase and is not a sort of
the term language.

The test is put to a pair of sorts: where two terms differ only in which of the two they stand
at and print the same words in the same structure, the sort that names a role in the game is the
one that goes. The sorts of the words a coercion wraps (`CardType` under `NounWord`) name kinds
of word and are not such a pair. `clause(you, did(attack, within = thisTurn))` prints "you
attacked this turn" whether it stands where a condition or an event is wanted, so `Condition`
and `GameEvent` go and `Clause` stays. `VerbPhrase` and `Clause` are no such pair: one prints a
subject and the other does not.

## 2. A Clause Stands Bare

**A clause stands bare wherever the text prints one.** In the direction, the words that make an
event, a condition or a static statement out of a clause go, and the word that prints the
surrounding construction takes the clause:

| Card | Text | As `voice-and-object-phrases.md` builds it | In the direction |
|---|---|---|---|
| *Grollub* | "Whenever this creature is dealt damage, …" | `whenever(does(thisCreature, be(done(…))), …)` | `whenever(clause(thisCreature, be(done(…))), …)` |
| *Mardu Hordechief* | "if you attacked this turn" | `holds(you, did(attack, within = thisTurn))` | `clause(you, did(attack, within = thisTurn))` |
| *Glorious Anthem* | "Creatures you control get +1/+1." | `static(clause(…, getsPt(…)))`, through a coercion from `Clause` to `StaticSpec` | `static(clause(…, getsPt(…)))`, `static` taking a `Clause` |

So `does`, `holds` and the coercion from `Clause` to `StaticSpec` are scaffolding: the planned
tickets build them as `voice-and-object-phrases.md` states, and they come out when semantics
exists. `StaticSpec`, `GameEvent`, `Condition` and `Replacement` are
types of the meaning ([`semantics.md`](./semantics.md) §6).

## 3. More Than a Reading

**A surface term still says more than a Reading.** It is not the parse written another way:

- It names which word of the game each printed word is. A Reading has the verb "counter" with a
  frame; the term has the keyword action `counter`, and a keyword's Definition where the card prints
  the keyword.
- Its arguments are named roles, not complement positions, and the same roles in every voice and
  verb form (`voice-and-object-phrases.md` §3).
- It holds no inflection, agreement, case or punctuation, which the English layer derives
  (`spelling.md` §1, `english.md` §7).
- A fixed wording is one Template call, and its body is the surface term the wording abbreviates.
- Attachment and bracketing are explicit, so two Readings of one sentence are two terms.

## 4. Templates, Definitions and Rules Data

**Templates and Definitions are surface to surface.** A body is a surface term over the
declaration's parameters. Expansion is unchanged: it substitutes a Template's parameters, wraps
a Definition's body in its Reference Scope under its Deed, keeps every call and parameter
boundary, and reads each argument once, in printed order ([`syl.md`](./syl.md) §4, §5).

**Rules data is not text a card prints.** A state-based action or a conferral is a record (an
`SbaRule`'s `scope`, `when` and `then`; a `ConferralRule`'s `scope` and `confer`), a declaration's
`confers` field holds `Conferral` values, and some Definition bodies use words no card prints
(`asEvent` in `lifelink`, `stateBasedActionsChecked`). Their sorts say what the data does in the
game and stay. Where the line between rules data and printed text runs, what keeps a card from
calling a rules-only word, and what the clauses inside rules data are written as are open
([`semantics.md`](./semantics.md) §8).

## 5. What Is Checked at the Surface

**`cargo xtask dsl check` checks surface form only**: that names resolve at their expected sorts,
that arguments are bound as [`syl.md`](./syl.md) §2 says, and the structural refusals of
`voice-and-object-phrases.md` §9 that concern silent words (`subject` under `done`, `_` under
`shared`, `$actor` in an `action` Definition). The event gate and "a replacement's event is a
`would` clause" are not surface form. Semantics refuses them ([`semantics.md`](./semantics.md) §6).
Until it exists, the sorts that say what a clause means and the checks the planned tickets add to
`cargo xtask dsl check` stand in for it.
