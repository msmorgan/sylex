# Surface and semantics: a term says what the card prints, and its meaning is read from it

A `.syl` term follows its card's English phrase by phrase, and the same sorts that make it do so
also say what each clause means: `GameEvent`, `Condition`, `StaticSpec`, `Replacement`. The two
jobs pull apart. A term built from words that follow the English can be built for a wrong reading
of the text as easily as for the right one, so something has to refuse wrong readings, and the
words that say what a clause means are where the term stops following the English
(`voice-and-object-phrases.md` §13 item 5).

This spec splits them. The stored term is a **surface term**: it says which words and
constructions the card prints and nothing about what they do in the game. **Semantics** reads a
surface term and decides what it means: where each indefinite is introduced, what each reference
reads, and which clause is an event, a condition, a static statement or a replacement. A reading
semantics cannot assemble is refused there, not by spelling and not by a sort.

The `.syl` below is a sketch of the shape, marked SKETCH where a form is not settled. None of it
has been run through `cargo xtask dsl check`.

## 1. The layers

```
Oracle text
    ⇅   English       parse / realize
Reading
    ⇅   spelling      recover / spell
surface term                               stored; keeps Template and Definition calls
    │   semantics     one way: boxes, references, assembly
    ▼
meaning                                    never stored; every reference resolved
    │   lowering      one way
    ▼
core
```

- **`sylex_syl`** parses and formats the `.syl` language and nothing else.
- **`sylex_surface`** holds the structures the primitives' constructors build, the registry of
  declarations, and name resolution by expected sort (`syl.md` §3). It is the Rust AST of the
  cleanup's step 7. A surface term is a tree of these structures, with its Template and
  Definition calls kept.
- **`sylex_spelling`** relates Readings to surface terms, in both directions. It parses the
  `spelling` notation, recovers candidate terms from a Packed Forest and spells a term as a
  Reading. It refuses nothing for what a term means.
- **`sylex_semantics`** reads a surface term and builds its meaning (§3). The structures that
  say what a clause means are its Rust types, built by assembly and written in no `.syl` file.
  It is the home of the Discourse Representation Structure and of reference resolution.
- **A pragmatics step** ranks the readings semantics leaves (§5).
- Lowering compiles a meaning to core, where `architecture.md` has it compile the checked
  expanded tree.

The **surface term** is what `architecture.md` calls the resolved term: it is what is stored,
recovered and rendered, and it is canonical. Nothing between it and core is serialized, as
today (`architecture.md`, "Rejected alternatives").

## 2. What a surface term says

**A sort names a kind of phrase.** `Clause`, `VerbPhrase`, `NounPhrase`, `Nominal` and its
modifier sorts, `Amount`, a determiner, a duration phrase, a Turn Point are kinds of phrase. So
are the sorts of words and symbols a card prints: a card type, a color, a mana symbol, a counter
kind. A sort that says what a phrase does in the game is no kind of phrase and is not a sort of
the term language.

The test for one sort: two terms that differ only in standing at it or at another sort print the
same words in the same structure. `clause(you, did(attack, within = thisTurn))` prints "you
attacked this turn" whether it stands where a condition or an event is wanted, so `Condition`
and `GameEvent` fail the test. `VerbPhrase` and `Clause` pass it: one prints a subject and the
other does not.

**A clause stands bare wherever the text prints one.** The words that today make an event, a
condition or a static statement out of a clause go, and the word that prints the surrounding
construction takes the clause:

| Card | Text | With the voice spec's stopgaps | Surface |
|---|---|---|---|
| *Grollub* | "Whenever this creature is dealt damage, …" | `whenever(does(thisCreature, be(done(…))), …)` | `whenever(clause(thisCreature, be(done(…))), …)` |
| *Mardu Hordechief* | "if you attacked this turn" | `holds(you, did(attack, within = thisTurn))` | `clause(you, did(attack, within = thisTurn))` |
| *Glorious Anthem* | "Creatures you control get +1/+1." | `static(clause(…, getsPt(…)))`, through a coercion from `Clause` to `StaticSpec` | `static(clause(…, getsPt(…)))`, `static` taking a `Clause` |

So these are not built: `does`, `holds`, the coercion from `Clause` to `StaticSpec`, and a
`replace` declared at two sorts. `StaticSpec`, `GameEvent`, `Condition` and `Replacement` are
types of the meaning (§3).

**A surface term still says more than a Reading.** It is not the parse written another way:

- It names which word of the game each printed word is. A Reading has the verb "counter" with a
  frame; the term has the primitive `counter`, and a keyword's Definition where the card prints
  the keyword.
- Its arguments are named roles, not complement positions, and the same roles in every voice and
  verb form (`voice-and-object-phrases.md` §3).
- It holds no inflection, agreement, case or punctuation, which the English layer derives
  (`spelling.md` §1).
- A fixed wording is one Template call, and its body is the surface term the wording abbreviates.
- Attachment and bracketing are explicit, so two Readings of one sentence are two terms.

**Templates and Definitions are surface to surface.** A body is a surface term over the
declaration's parameters. Expansion is unchanged: it substitutes a Template's parameters, wraps
a Definition's body in its Reference Scope under its Deed, keeps every call and parameter
boundary, and reads each argument once, in printed order (`syl.md` §4, §5).

**Rules rows are not text a card prints.** A row under `plugins/builtin/rules/` is a record
(`scope` and `confer`, or `when` and `then`) at sorts such as `Conferral` that say what the row
does in the game, and its body uses words no card prints (`asEvent`, `stateBasedActionsChecked`).
Those sorts and words are rules data and are not put to the test above. What the clauses inside
a row are written as is open (§7).

**`cargo xtask dsl check` checks surface form only**: that names resolve at their expected
sorts, that arguments are bound as `syl.md` §2 says, and the structural refusals of
`voice-and-object-phrases.md` §9 that concern silent words (`subject` under `done`, `_` under
`shared`, `$actor` in an `action` Definition). The event gate and "a replacement's event is a
`would` clause" are not surface form. Semantics refuses them (§3).

## 3. Reading a term: semantics

Semantics takes the expanded surface tree and gives a meaning or a refusal. It runs three passes
in one crate.

1. **Boxes.** Each operator and each construction that holds clauses says which boxes it opens
   and which of its parts stand in which (`semantics.md` §2.2): a trigger, a conditional and a
   replacement are duplex; `cant`, `must`, `can`, a negation and an "otherwise" are negated or
   modal; a distributive opens a loop. The rule that an indefinite subject of a modal clause is
   generic when the clause states what holds and a choice when it states a deed, and that a
   counting subject is read inside the operator's box (`voice-and-object-phrases.md` §5), is this
   pass's.
2. **References.** Every mention introduces a Referent or resolves to one, in printed order, by
   the accessibility rules of `semantics.md` §2.4. A Definition's body is read in its own
   Reference Scope and exports its outcomes to the caller (`semantics.md` §2.3), so *Cursed
   Windbreaker*'s "manifest dread, then attach this Equipment to that creature" reads the
   creature that the Definition of manifest dread puts onto the battlefield.
3. **Assembly.** Each clause is given its meaning from where it stands and what it holds:
   - the clause of a trigger is an event pattern, and it must name one event (the event gate of
     `semantics.md` §4.6, read through `done`, `be`, Templates and coordinations);
   - a clause under "if", "as long as" or "unless" is a condition, a state or a lookback;
   - a static ability's clause is a continuous effect, a deontic rule, a replacement effect, a
     cost statement or one of the other static statements, by its verb phrase;
   - a clause whose verb phrase is a `would`, with a clause marked "instead", is a replacement
     effect [CR#614.1a], and so is a clause whose subject carries a `would` relative
     (`voice-and-object-phrases.md` §8);
   - a `subject` under `done` becomes the role of the slot it stands in, and a `done` call's
     performer is its `by` or nobody.

   What assembly builds is today's expanded tree as Rust values: an event with its subject and
   Action, a Static Spec, a Replacement. Lowering reads those. Where the Comprehensive Rules
   distinguish two structures the meaning does too (`semantics.md` §1 item 5(a)); the surface
   term distinguishes them only as far as the printed words do.

**Why one layer.** Reference resolution needs the boxes, and where a box opens depends on what
an operator means. Assembly needs resolved references, since an event pattern's subject is a
Referent. A reference pass that ran before any meaning was known would have to restate what
each operator means.

**Each pass answers one of three ways** for a term or a part of one: it is accepted; it is
refused, with the rule that refuses it; or it is not understood, because semantics has no rule
for the construction yet. Only a refusal says a reading is wrong. A term that is not understood
is reported as that, and is never dropped as though it had been refused (§4).

## 4. From text to a stored term

1. **Recovery** gives the set of candidate surface terms whose wording matches a Reading in the
   card's Packed Forest. Candidates are not one per Reading: one Reading gives several where
   Constructions word alike, and several Readings give one term, which is one candidate.
2. **Semantics** reads each candidate. A candidate it refuses in any pass drops out. A candidate
   it does not understand stays, marked.
3. **Candidates with one meaning are one class.** Two surface terms that assemble to the same
   meaning differ in nothing the game can see.
4. **Pragmatics ranks the classes** when more than one is left (§5).
5. **The top class's term is stored** as the card's `.syl`. Within a class the term is chosen by
   a fixed rule, so the same text always stores the same term.

**The stored term is the decision, and rank only proposes it.** Reading a stored term runs
semantics and never pragmatics. A wrong choice is corrected by editing the file. What is checked
for every card is that its stored term is among the candidates recovery gives for its text, that
semantics accepts it, and that it spells back to the analyzed text (`architecture.md`, "Exact
round trip"); where it ranks is reported and gates nothing, as a Reading's rank gates nothing
(`english.md` §5).

**The report** for a card gives the number of candidates at each step, the rule that refused
each dropped candidate, each candidate that was not understood, and, where pragmatics chose,
the preference that decided. Two classes that pragmatics ranks equal are reported as a tie, and
no term is stored for that card until an author picks one.

Recovery enumerates candidates here. `spelling.md` §8 calls a Construction that makes recovery
enumerate trees a defect; that sentence is about cost, and whether the candidate sets are small
enough to enumerate is measured on the corpus before anything is packed.

## 5. Pragmatics

Some wordings leave more than one meaning that semantics accepts. *Broken Wings* prints "Destroy
target artifact, enchantment, or creature with flying." and *Troublemaker Ouphe* "exile target
artifact or enchantment an opponent controls". Each has a term in which the trailing modifier
describes the whole coordination and one in which it describes the last conjunct. Both are
well-formed readings of both cards, and nothing in the rules refuses an artifact with flying.
*Broken Wings* means the last conjunct and *Troublemaker Ouphe* the whole coordination.

Pragmatics ranks such classes by simple preferences, stated as data beside the declarations they
concern (SKETCH):

    keyword_ability flying = { pragmatics = { preferHostTypes = [creature] }, … }
    subtype vehicle        = { pragmatics = { mimicTypes = [creature] }, … }

The step's design is `docs/design/pragmatics.md`. A preference is an exception to a default. For a trailing modifier over a coordination the
default is the whole coordination, and `flying`'s preferred host narrows it on *Broken Wings*.
It is a step of its own, outside semantics, because it encodes what a card is likely to mean and
not what its words can mean.

## 6. What this changes

**In the design docs**, edited by the tickets that build each part:

| Where | Today | Becomes |
|---|---|---|
| `architecture.md`, "The layers" | "Semantics is the card language: `.syl` declarations"; the resolved term and the expanded tree as its two levels | §1's layers and crates |
| `architecture.md`, "Spelling is a construction grammar"; `spelling.md` §5 | "Meaning resolves the ambiguity, never rank or rule order"; "a card with more is reported, never stored as a choice" | §4: semantics prunes, pragmatics ranks, the stored term decides |
| `architecture.md`, design principle 4 | "the spelling boundary refuses a term that has no English" as what bounds overgeneration | that, and semantics refusing a term with no meaning |
| `semantics.md`, overview and §4.1 | the checker reads `.syl` terms built from `does`, `would`, `did` and the deontic words over a `Subject` | three passes over the expanded surface tree (§3); the table of contexts is assembly's |
| `semantics.md` §4.6 | the event gate as a rule on which heads a word admits | a refusal assembly makes |
| `syl.md` overview | resolution and expansion "exist only as throwaway tools in `xtask`" | `sylex_surface` resolves; expansion and reading are `sylex_semantics`'s |
| `spelling.md` §8 | "no spelling crate exists" | `sylex_spelling` and what it holds |

**In `voice-and-object-phrases.md`**, the clause, the passive, the operators, the object phrases
and the shared phrase stand as written. These parts of it are replaced by §2 and §3 here:

- §1's table of positions: a trigger, a condition and a static ability take a `Clause`; `does`
  and `holds` are not built, and neither is the coercion from `Clause` to `StaticSpec`. Its
  §13 item 3 (`static` taking a `Clause`) is part of this spec.
- §8's event gate and §9's `replace` refusal are semantics' refusals, not `cargo xtask dsl
  check`'s.
- §9's last paragraph: the candidates the term rules leave go through §4 and §5 here.

**In the planned tickets**, these are written against the stopgaps and are revised before any is
claimed: `syl-clause-and-imperative`, `syl-continuous-effect-subject`,
`syl-passive-participle-and-trigger-events`, `syl-would-operator-and-replacement-clauses`,
`syl-lookback-operator-and-holds`, `syl-deontic-verb-phrase-operators`, `syl-event-gate`,
`syl-replacement-stated-as-clause` and `english-syl-term-recovery`. That list is not complete:
every other planned ticket that writes a declaration or a card at `StaticSpec`, `Condition`,
`GameEvent` or `Replacement`, or names `does`, `did` or `replace` (`syl-spelling-forms-and-runs`
labels the verb forms of `does` and `did`), is read against this spec before it is claimed.
These do not depend on it: `syl-spelling-conventions`, `syl-nominal-descriptions`,
`syl-damage-and-life-noun-phrases`, `syl-counters-noun-phrase`, `syl-verb-phrase-sort-name`,
`syl-definition-actor-parameter` and `syl-misnamed-declarations`.

**In the glossaries**: **Static Spec** and the other entries for what a clause means describe
types of the meaning; new entries name the surface term and the three outcomes of §3.

## 7. Open

1. **Which sorts are kinds of phrase.** §2 gives the test and applies it to the four sorts the
   voice spec worked around. `plugins/builtin/` declares words at more than a hundred sorts, among
   them `Conferral`, `CharacteristicEdit`, `MoveRider`, `CostShift`, `Cost`, `Timing` and
   `TriggerCondition`, and none of those has been put to the test. Each is either a kind of
   phrase the text prints or a classification that moves into the meaning.
2. **How a replacement is written at the surface.** *Phytohydra* prints "If damage would be
   dealt to this creature, put that many +1/+1 counters on it instead.": a conditional sentence
   whose condition holds a "would" and whose consequent ends with "instead". Either the term is
   one conditional word over two clauses with "instead" a Template on the second, and assembly
   reads the replacement off the `would` and the "instead" [CR#614.1a]; or `replace` stays a
   word of its own for this wording. The same choice exists for an intervening "if"
   [CR#603.4], which is printed as a position after the trigger and not as a different word.
3. **The pragmatics defaults and the form of the data.** §5 names one default and two
   preferences. The list of ambiguities that reach pragmatics, the default for each, what a
   preference may say, and what `mimicTypes` does are not designed; the first run of §4 over
   the corpus gives the list.
4. **How candidates of one class are told apart for storing** (§4 step 5): the fixed rule is not
   chosen. The Reading's Ranking Key is one candidate for it.
5. **The names.** The glossary's **Realization** is "Producing the surface text of a Reading"
   (`docs/contexts/oracle-english/CONTEXT.md`), and a surface term is not that text. The crate names of
   §1, "surface term" against today's "resolved term", and a name for the meaning are to be
   settled against both glossaries.
6. **Sentences that depend on an earlier one.** "If the player does, untap that creature."
   (*Apathy*) and the other follow-ups of `voice-and-object-phrases.md` §13 items 6 and 7 are
   ordinary sentences at the surface, and what links each to the choice or deed it follows is
   reference resolution's. Which Referent an optional deed introduces for "does" and "do" to
   read is not designed.
7. **A Definition's meaning.** A Definition's sorts are checked once, at its declaration, and its
   references in each context that expands it (`semantics.md` §3). Whether assembly runs on a
   body once or at each call is not settled.
