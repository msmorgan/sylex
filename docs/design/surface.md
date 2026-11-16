# Surface

## Status: DRAFT

This document is directional. It describes what a surface term may say: the stored `.syl` term
of a card, which follows the card's English and does not say which of its clauses is an event, a
condition, a static statement or a replacement. The design is not settled, and
[`semantics.md`](./semantics.md) §6 lists what is open.

A surface term has a meaning, and has it indirectly. The Comprehensive Rules are transcribed
into surface form the same way a card's text is: a keyword's Definition is the rule that defines
the keyword, written as a surface term. Every word that can be defined in terms of other words
is, by a Template or a Definition, down to the primitives. Semantics takes it from there: it
reads the expanded term and gives the primitives and the places a clause stands their meaning
([`semantics.md`](./semantics.md) §4).

- **Companions:** [`syl.md`](./syl.md), the language a surface term is written in, and
  [`semantics.md`](./semantics.md), how a surface term is read into its meaning.

The words a surface term is built from are designed in `docs/specs/voice-and-object-phrases.md`
(the clause, the verb phrase and its operators, the object phrases) and in the planned tickets
that build them. §6 and §7 say what Definitions, Templates and the action-clause spine are in
today's term language, where the sorts that say what a clause means are still in it (§2). They
were written when the term classified its own clauses, so they still say what some statements
mean beside how they are written, and the tickets that build the clause and voice words
rewrite them.

## 1. A Sort Names a Kind of Phrase

**A sort names a kind of phrase.** `Clause`, `VerbPhrase`, `NounPhrase`, `Nominal` and its
modifier sorts, `Amount`, a determiner, a duration phrase, a Turn Point are kinds of phrase. So
are the sorts of words and symbols a card prints: a card type, a color, a mana symbol, a counter
kind. A sort that says what a phrase does in the game is no kind of phrase and is not a sort of
the term language.

The test is put to a pair of sorts: where two terms differ only in which of the two they stand
at and print the same words in the same structure, the sort that names a role in the game is the
one that goes. The sorts of the words a coercion wraps (`CardType` under `Nominal`) name kinds
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
types of the meaning ([`semantics.md`](./semantics.md) §4).

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

**The rules are transcribed as cards are.** A Definition's body, a state-based action and a
conferral state a rule of the Comprehensive Rules in surface form. Today a state-based action and a
conferral are records (an `SbaRule`'s `scope`, `when` and `then`; a `ConferralRule`'s `scope` and
`confer`), a declaration's `confers` field holds `Conferral` values, and some bodies use words no
card prints (`asEvent` in `lifelink`, `stateBasedActionsChecked`). A fair amount of that becomes
clauses as the rule states them, and a fair few of those sorts stay, as the frame a rule is written
in: the frames of the kinds of ability (`static`, `triggered`, `activated`, `spell`) are frames of
that kind already. Which declaration goes which way is not settled
([`semantics.md`](./semantics.md) §6). Two things keep a card from calling a word only a
rule may use, and neither is a sort: the word has no route through spelling into a term read
from a card's text, and semantics refuses a card's term that calls it
([`semantics.md`](./semantics.md) §4).

## 5. What Is Checked at the Surface

**`cargo xtask dsl check` checks surface form only**: that names resolve at their expected sorts,
that arguments are bound as [`syl.md`](./syl.md) §2 says, and the structural refusals of
`voice-and-object-phrases.md` §9 that concern silent words (`subject` under `done`, `_` under
`shared`, `$actor` in an `action` Definition). The event gate and "a replacement's event is a
`would` clause" are not surface form. Semantics refuses them ([`semantics.md`](./semantics.md) §3.2
and §4). Until it exists, the sorts that say what a clause means and the checks the planned tickets
add to `cargo xtask dsl check` stand in for it.

## 6. Definitions and Templates

A named word is a primitive, a Template or a Definition; why the language has the three is a
language matter ([`syl.md`](./syl.md) §1). A primitive only names a model constructor. The other
two differ in what they mean:

- **A Definition** is vocabulary the rules define: a keyword action or ability ([CR#701.1,702.1]),
  a Counter, a Designation, a Defined Token ([CR#111.10,701.71a]). It carries rules meaning and
  its own Reference Scope ([`semantics.md`](./semantics.md) §2.3).
  - A keyword action's Definition carries a Deed that triggers, lookbacks and prohibitions can name
    ("whenever you scry", "the card exiled this way", "can't be regenerated"). "This way" reads the
    current ability's stamps ([`semantics.md`](./semantics.md) §2.4), and the stamp a keyword
    action leaves is its Definition's Deed. That is why each keyword action has exactly one Deed
    (§6.1): a variant with its own Deed would make "exiled this way" miss it.
  - A keyword ability's Definition carries a Deed only where the rules let text ask who did it:
    a creature tapped to pay a crew cost "crews" the Vehicle, and the Vehicle is "crewed by" it
    ([CR#702.122b,702.122c]), so Crew's Definition carries the Deed paying that cost stamps.
  - A Primitive Keyword Ability is the exception: a keyword ability defined by the engine rather
    than by a Definition (first strike, double strike, trample and banding). It is a primitive,
    with no body and no Reference Scope ([`syl.md`](./syl.md) §3).
  - A Counter's, a Designation's or a Defined Token's Definition carries none.
  - A Defined Token's Definition takes no arguments, so its Reference Scope holds nothing of the
    caller's; the ability text in its characteristics is embedded ability text
    ([`semantics.md`](./semantics.md) §2.3).
- **A Template** is printed wording only. It expands transparently into the expressions its parts
  would be written as, and it adds no meaning: it has no Reference Scope and no Deed, so its
  body's references reach the referents at its call site.

Templates and Definitions bind parameters alike: each parameter's argument is read once, at its
first use, and later uses re-mention it ([`syl.md`](./syl.md) §4). In expansion they differ only in
Reference Scope and Deed.

They are kept apart because conflating them goes wrong both ways. A phrasing given a Deed or a
scope invents a rules distinction the CR does not make: an event then matches it, and referents it
should see are hidden from it. A rules-defined keyword treated as a phrasing loses its identity, so
"can't X" and "whenever X" can no longer name it. So the kind a declaration states decides which it
is ([`syl.md`](./syl.md) §1). That a Definition's body is the rules' meaning of its keyword is
trusted, vouched for by its CR citation like any rule declaration ([`semantics.md`](./semantics.md)
§1 item 4). Its sorts are checked once, at its declaration ([`syl.md`](./syl.md) §4); its reference
resolution is checked in each context that expands it.

### 6.1 Keyword Actions and Deeds

- Every keyword action is a Definition with exactly one Deed ([CR#701.1]). The exception is
  an action the rules define as another action: `convert` is a Template over `transform`
  ([CR#701.28a]).
- A keyword ability's verb that is done by doing another deed is a Definition over that deed:
  `devour` over `sacrifice` ([CR#702.82b]).
- With no Handoff to a Permanent, a deed a permanent performs is performed by the Source
  ([CR#701.14a,701.44a]). The checker never guesses a deed from a body's shape.

## 7. The Action-Clause Spine

### 7.1 One Action Per Verb

Every verb is one Action, its bare infinitive verb phrase. Why: otherwise each verb is encoded once
per clause mood (as an instruction, an event, a prohibition and a lookback), each copy with its own
subject and patient plumbing, and the copies drift apart. With one Action, "sacrifice" means the
same thing in "sacrifice a creature", "whenever you sacrifice", "can't be sacrificed" and "if you
sacrificed". Eight contexts embed that Action directly, with no double wrapping:

| Context | Form |
|---|---|
| Imperative | `action` |
| Handoff | `instruct(subj, action)` |
| Cost | `perform(action)` |
| Trigger | `does(subj, action)` |
| Replacement | `replace(would(subj, action), repl)` |
| Deontic | `cant`, `must`, `can`, `gatedBy` |
| Lookback | `did(subj, action, within)` |
| Negative declarative | `doesnt(subj, action)` |

Triggers, replacements, deontic rules and lookbacks share one head gate
([`semantics.md`](./semantics.md) §3.2), and every context shares one reading of subject, action
and agent. A context adds only what is its own.

### 7.2 Voice

Voice belongs to the Subject, not to the Action, so the passive needs no separate event. The
passive matters only because it promotes the patient to grammatical subject. The patient is then
read first, ahead of any agent, and that decides which occurrence of a parameter is its first
mention ([`syl.md`](./syl.md) §4). A Subject is one of three:

- `actor(np)`: active. `np` is read first, with role subject.
- `any`: passive with no agent. The patient is promoted to subject.
- `by(np)`: passive with an agent. The patient is promoted and read first, then `np` with role
  other.

The noun phrase of `actor(np)` or `by(np)` names the Actor and is an ordinary referent, not the
Actor itself ([`semantics.md`](./semantics.md) §2.1). What an active Subject names need not
be a player or a permanent: a spell or ability ("<ability> triggers") or an effect
(`actor(thisEffect)`, §7.6 and [`semantics.md`](./semantics.md) §3.3) can be one.

"Becomes blocked" is therefore not the agentless passive of `block`. It is its own event head,
`becomeBlocked`: the Blocked status beginning, once per combat, even when an effect blocks the
creature with no blocker ([CR#509.1h,509.3c]). "Becomes blocked by <B>" is `by(B)` over `block`,
one event per blocker ([CR#509.3d]).

### 7.3 `any`

There is one wildcard word, `any`, written at every sort that needs one. No sort has a wildcard
word of its own.

### 7.4 Patients and Relative Clauses

- A transitive Action always keeps its patient inside it. The Subject supplies only the performer.
- A relative clause filtering by an event is a lookback with `which` in the relativized slot.
- "The same is true for …" is one frame with a `thatKeyword` placeholder, checked per keyword
  ([CR#702.1c]).

### 7.5 Events as Written

Which events a pattern may name, and from which side of an event its descriptions are read, is
[`semantics.md`](./semantics.md) §3.1 and §3.2.

- **A move is the one Zone Change event.** `die`, `enter` and `leave` are Templates over it
  ([CR#700.4]). A move carries no cause; a caused move is wrapped in `causes` or `causedBy`.
- **Pattern-only heads.** `gainControlOf` is a pattern head only; performed control change is
  `establish(controlGrant(…))` ([CR#613.1b]). `anyCost` and `anyBallot` likewise appear only in
  patterns.
- **Paying life** is a cost payment of `loseLife`; there is no life Cost ([CR#119.4,119.8]).
- **Alternative events are an `or` over events**: one trigger condition, met when a game event
  matches any arm ([CR#603.2]). Several trigger conditions are the disjunction at
  `TriggerCondition` instead ([CR#603.1b]). Each arm is a whole event with its own subject:
  "attacks or blocks" is `or([does(actor(x), attack), does(actor(x), block)])`, and "blocks or
  becomes blocked" joins two events whose subjects differ. An `or` over Actions would be a
  compound action, which the gate refuses.
- **"At" names a Turn Point.** A phase or step beginning is a trigger event [CR#603.2b], and
  "at end of combat" triggers as the end of combat step begins [CR#511.2], so there is one
  triggered-ability constructor and `at` builds it too. What tells the trigger words apart is
  sort: `at` takes a `TurnPoint` (`beginningOf`, an `nthOccurrence` of one), and the event it
  triggers on is the game reaching it (`reached`); `when` and `whenever` take a `GameEvent`,
  which no Turn Point is. A delayed "at" trigger (`atNext`) takes a Turn Point the same way.
  Any other event-taking position that names a beginning writes `reached` ("until your next
  turn", "continuously since the beginning of your last upkeep"). The end of a phase, step or
  turn (`endOf`) is no Turn Point: "at end of combat" is a beginning. `reached(point)` is an
  ordinary `GameEvent`, which durations, `heldSince` and `delay` need, so `when` and `whenever`
  refuse a Turn Point written bare but not one wrapped in `reached`; the trigger word for a Turn
  Point is `at`.
- **A trigger condition prints its trigger word.** `triggered` takes one `TriggerCondition`:
  `when`, `whenever` or `at` over its event ([CR#603.1]), so a trigger word stands only where a
  trigger condition does. Several trigger conditions of one ability ([CR#603.1b]) are one
  disjunction of them, each with its own word: "When this creature enters and whenever it
  attacks while saddled" (*Autarch Mammoth*). The ability triggers on any of them. The primitive is
  `or`, named for that disjunction and spelled with "or" (*Tenuous Truce*); cards that print "and"
  between their trigger conditions write the Template `and` over `or`. A concurrent condition or timing window ("while
  saddled", "during your turn") restricts the event it follows (`timed`), so it stands inside the
  one trigger condition it qualifies.

### 7.6 Static Statements as Written

How an Unless Condition is read, what a cast permission widens and what makes something an
effect are [`semantics.md`](./semantics.md) §3.3.

- **Abilities granted, lost or listed together are one list**, in printed order, at the parameter
  of the word that grants, removes or lists them: "gains hexproof and indestructible" (`gain`),
  "has double strike and trample" (`abilityGrant`), "loses hexproof and indestructible"
  (`abilityLoss`), "except it has haste and dethrone" (`alsoHas`), "creature tokens with flying
  and vigilance" (`creatureToken`). Each element is an ability of its own. An object may have
  several abilities, and a spell or ability may grant it more ([CR#113.2c]): granted and lost
  abilities are each added or removed in layer 6 ([CR#613.1f]), the abilities a copy effect's
  exception lists each become part of the copy's copiable values ([CR#707.9b]), and the abilities
  listed for a token are each an ability the effect that creates it gives it ([CR#113.2c,111.3]).
  So no Ability stands for several, and `Ability` has no conjunction. A grant of one ability is a
  list of one.
- **A deontic rule** is a Subject, an Action and optional modifiers: Timing, as-though, a Deontic
  Exception, an Unless Condition (`cant` and `must`), a cause (`cant` only), and a Permission
  Rider (`can` only) ([CR#113.2c]). "Can't A or B" is a conjunction of prohibitions.
- **An alternative or additional cost stands where its sentence prints it.** "You may cast X by
  paying C …" or "… without paying its mana cost", in a permission or an instructed cast, fills that
  `cast`'s `altCost` (or, "as an additional cost", its `addedCost`). "You may pay C rather than pay
  …" is the static `altCost`, and "As an additional cost to cast …" the static `addedCost`
  ([CR#118.8,118.9]).
- **A replacement's `repl`** says what happens:
  - `instead`: in the event's place.
  - `before`: before the event, which still happens; the choices "as it enters" requires
    ([CR#614.12a]).
  - `alongside`: as part of the event; "enters with" ([CR#603.6d]).
  - `withRiders`: the entering happens with Entry Riders.
- **A negative declarative** `doesnt(subj, action)` says what its Subject does not do. It is no
  deontic rule and takes no modifiers. "This effect doesn't remove this Aura" is
  `doesnt(actor(thisEffect), remove(thisAura))`, written as the card prints it: its own Static Spec
  beside the spec it talks about, the two joined by `conjunction` in one static ability. Which
  consequence of the effect it exempts, an Aura put into its owner's graveyard or Equipment
  unattached ([CR#704.5m,704.5n]), is the engine's ([CR#702.16n]). `remove` is written only here:
  it is no event head ([`semantics.md`](./semantics.md) §3.2).

### 7.7 What a Term Holds for Spelling

- `getsPt` carries both printed deltas ("+X/+0"), because spelling must render them back
  ([CR#613.4c]).
- A word replacement accepts only the replaceable word sorts, so a card name is unrepresentable
  ([CR#612.2a]).
