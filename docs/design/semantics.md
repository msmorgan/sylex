# Semantics

## Status: DRAFT

This document is directional. It describes how a surface term is read into its meaning (what a
clause means, reference resolution over a Discourse Representation Structure, and the path from a
card's text to a stored term), and the design is not settled: read it for the intended shape, not
as a contract the code keeps.

- **Companions:** [`syl.md`](./syl.md), the `.syl` data language, and
  [`surface.md`](./surface.md), what a term is built from: Definitions, Templates and the
  action-clause spine.

## Overview

A card is written as a `.syl` term that follows its English phrase by phrase, and its meaning is
read from that term. The pipeline
has four stages:

1. Parse the `.syl` files ([`syl.md`](./syl.md) §2).
2. Resolve names by expected sort, and expand Templates and Definitions ([`syl.md`](./syl.md)
   §3–§4).
3. Read the expanded term: classify each clause, build a Discourse Representation Structure,
   resolve every reference against it (§2), and build the meaning (§4).
4. Lower the meaning to the engine's core.

The term says what the card prints and semantics decides what it means. That split is the
direction: today the term language's own sorts (`GameEvent`, `Condition`, `StaticSpec`,
`Replacement`) say what a clause means, and [`surface.md`](./surface.md) §7 describes the
action-clause spine in those terms. They are scaffolding until the reading of §4 exists
([`surface.md`](./surface.md) §2).

## 1. Principles

1. **Surface-faithful.** A construction stands where the English puts it. Nothing is hoisted:
   there are no prenex target lists and no positional slot reads. Lowering does all rearrangement
   ([`core.md`](./core.md)). Why: a term that mirrors the text
   can be checked against the text and spelled back into it. A hoisted term needs a second,
   hidden mapping to the wording, and that mapping is where readings go wrong.
2. **Printed reading order.** The term is read in the order the text prints it
   ([`syl.md`](./syl.md) §5). Reading order does not limit reference direction: an anaphor may
   resolve to a mention later in the text (§2.4). There is no out-of-band announcement channel.
   - Across an activated ability's colon, the effect reads every cost referent wherever the cost
     moved it ([CR#400.7j]).
   - A referent the text moves stays the same referent; the move is recorded on it (§2.1).
3. **The checker reads text, not game state.** It verifies that a term is a faithful reading of
   its English: references resolve, sorts are right, and frames are well-formed. Whether an ability
   is characteristic-defining is fixed by the textual criteria of [CR#604.3a], so the checker
   decides it. The checker is not a rules engine. These belong to the engine: runtime visibility,
   the availability of last known information ([CR#608.2h]), timing, and the order of state-based
   actions and replacements ([CR#704.3,616.1]). So reference resolution applies no public-zone
   filter: a well-formed textual antecedent is admitted. Why: what a text means is decidable from
   the text, but runtime state is not. A check of runtime state can only be satisfied by adding
   model structure that the text does not contain (item 5(c)).
4. **Rules are data.** Every decomposable game rule is a declaration, not engine code: damage
   results, state-based actions, turn-based actions, intrinsic type rules
   ([CR#702.15b,704.5q,703.1]). Their CR citations, not the checker, vouch for their correctness.
   Why: a rule written as data is written once and read by every tool. A rule in engine code is a
   second source of truth that the model cannot see.
5. **The CR decides structure.**
   - (a) Where the CR distinguishes two structures, the model does too: an intervening "if" from a
     plain condition ([CR#603.4]), a replacement from a trigger ([CR#614.1,603.1]), an additional
     cost from an alternative one ([CR#118.8,118.9]), a turn-based action from an effect
     ([CR#703.1]). A distinction is never dropped on the theory that no card notices it.
   - (b) Compose before minting: express a CR definition with existing primitives, putting the
     printed wording on Templates. For example, a planeswalker's or battle's entry with counters
     reads `statOf(stat(loyalty|defense), this)`, which off the battlefield is the printed number
     ([CR#306.5a,306.5b,310.4a,310.4b]).
   - (c) Add no node, field or variant to satisfy a check of runtime state; remove the check
     instead.

## 2. Discourse and Reference Resolution

Reference is modeled explicitly with Discourse Representation Theory (DRT). Why: anaphor resolution
(§2.4) needs stable referent identities, clause boundaries and nested boxes. Positional binding
stacks cannot supply these, because a binding's position shifts as others are pushed and removed.

### 2.1 The DRS

- Every mention that introduces an entity, quantity, choice or outcome creates a referent with a
  stable identity. A re-mention resolves to the existing referent and creates nothing.
- Each mention has a grammatical role in its clause: subject, object or other
  ([`surface.md`](./surface.md) §7.2).
- **A Subject is a referent; an Actor is not.** The noun phrase of a clause's Subject
  ([`surface.md`](./surface.md) §7.2) is an ordinary mention, which pronouns and demonstratives
  read like any other: "it" in "whenever a creature dies, … it" reads the Subject's "a creature".
  Inside a verb phrase, the `subject` word at `NounPhrase` reads what the phrase is said of as a
  participant of its event, printing no word. The Actor is the performer of an Action: in an
  instruction, the controller of the text's spell or ability unless a Handoff names another; in a
  clause, what its active Subject or its `by` agent names. A Handoff makes its Actor the innermost
  for the Action it contains. The Actor is a role that only the `actor` word at `NounPhrase` reads,
  not a discourse referent; pronouns, groups and counts never read it. The noun phrase naming it,
  in a Subject, an agent or a Handoff, is an ordinary referent.
- A disjunctive noun phrase exports only the referents common to every branch. A coordination bound
  to a parameter binds every member under the parameter's one Binding Identity.
- A zone change keeps the referent and records the move on it: its origin, its new zone and the
  deed. Carrier words ("that card" after "exile target creature", [CR#109.2]), the consuming
  verb's applicability (§2.4) and Participial Descriptions read those. The origin keys the reads
  that reach the object as it was before the move: a lookback's historical subject (§3.4), as in
  *Saw in Half*'s and *Kaervek's Purge*'s "If that creature dies this way" and *Tuktuk
  Scrapper*'s "If that artifact is put into a graveyard this way, … that artifact's controller",
  and last-known information once the move hid it (*Zoyowa's Justice*'s "its mana value" after
  "shuffles it into their library", [CR#608.2h]). That the moved card is a new object in the game
  ([CR#400.7]) is game identity, which lowering supplies.

### 2.2 Boxes

An indefinite's force and accessibility come from the box it is introduced in, not from its
determiner (Kamp & Reyle 1993). Targets survive every box closure, because they are chosen as the
spell or ability is put on the stack ([CR#601.2c,115.1]).

1. **Top level, and activated cost → effect.** An indefinite is an existential choice made at
   action time. It is accessible to the rest of the paragraph, across the colon too.
2. **Duplex** (a trigger, a replacement, a positive conditional). Antecedent referents are
   accessible in the consequent and closed after it. A `doIf` with no else branch lets its branch's
   output, such as a token it creates, continue to later text, while its condition's referents
   close. With an else branch, branch-local non-targets are closed. An existential
   ("there is …", `exists`) publishes its indefinite subject to the consequent. A conditional
   Static Spec's consequent exports the plural domain of a bare plural or "all" it introduces
   (`described(all, …)`) to the text after the conditional in its ability, as a negated or modal
   box keeps its universal domains (item 3): *Guardian Beast*'s "This effect doesn't remove Auras
   already attached to those artifacts" reads the "noncreature artifacts you control" of its "As
   long as" sentence.
3. **Negated or modal** (`cant`/`must`/`can`/`gatedBy`, `not`, an else branch). Hypothetical
   indefinite, counted and partitive participants are closed on exit; a deontic rule's Unless
   Condition is inside its box (§3.3). Under `cant`, a hypothetical indefinite has universal
   force: "A player can't cast …" forbids each player ([CR#205.4e]). Targets, definite references
   and universal domains stay accessible under their identities ([CR#101.2]).
4. **Distributive loop** (`doForEach`). Each pass binds the loop referent for the body to read.
   On exit, the loop exports plural referents for the targets the body announced and the objects
   it created. Each is a recorded discourse fact, not derived from determiners or stamps.
5. **Relative clause or comparison**, including the domain `countOf` and `aggregateAll` measure. It is
   closed on exit, so no later pronoun reads a measured domain. Attachment Hosts ("enchanted",
   "equipped") project to the ability's box ([CR#303.4b]).
6. **Definition Reference Scope:** see §2.3.

A self-replacement starts from the state the replaced action read, because the replaced event never
happens ([CR#614.6,614.15]).

### 2.3 Definition Reference Scope

A Definition body runs in its own Reference Scope. Why: the rules define a keyword independently
of the sentence that uses it. So a pronoun inside the body must not find the caller's referents,
and the body's own scratch referents must not leak out to the caller's pronouns.

- The caller's referents are hidden from it, except its arguments, an `action` Definition's
  implicit `$actor` parameter (bound at the call to the call's performer) among them: a
  Definition's body reads its call's performer through `$actor`, whereas the `actor` word reads the
  performer of the verb phrase it stands in and rebinds under a Handoff inside the body, and
  `subject` reads what its enclosing verb phrase is said of as a participant. A referent hidden by
  an enclosing Definition stays hidden.
- Each argument is read in the caller's scope and bound to the Definition's Binding Identity at its
  first mention.
- On exit, the body's changes to its arguments and its outcomes are exported to the caller. Its
  partitive and local-player scratch referents are dropped.
- The body never changes a role the caller can see. Each argument is pending in the caller as the
  call's object.

A token specification or Entry Rider passed in is read in the caller. Embedded ability text sees
only announced choices.

### 2.4 Anaphor Resolution

Resolution applies DRT techniques to the DRS, measured against a gold set of the corpus's
endophora. Whatever the technique, a referent is accessible to an anaphor only if all four hold:

1. No active Reference Scope hides it.
2. Its number agrees with the anaphor's. Inside a deontic rule's Unless Condition (§3.3), a
   universal participant is read one member at a time: a plural anaphor reads the domain
   distributively, and a singular one reads the member at hand.
3. Its sort, zone and Head Noun fit the anaphor's carrier word ([CR#109.2]).
4. The consuming verb can apply to it in its zone ([CR#109.2,701.21a]). A creature that died
   cannot be sacrificed or have counters put on it as a permanent.

A description is a `Nominal`, `nominal(pre, head, post)`, and its head sets the domain it ranges
over. A card type or subtype at `Nominal` means a permanent of that card type or subtype on the
battlefield ([CR#109.2,110.1]); at `PreModifier` it tests the characteristic alone, with no zone
implied. `permanent` means a card or token on the battlefield ([CR#110.1]); a printed "on the
battlefield" stays the postmodifier `in(battlefield)`. `card` means a Magic card or an object
represented by one, never a token ([CR#108.2,108.2b]), and carries no zone of its own: a stated
zone comes from its postmodifiers or its verb ([CR#109.2a]). `spell` means a spell on the stack
([CR#109.2b,112.1]), `source` a source in any zone ([CR#109.2c,113.7]), `token` the token test
alone ([CR#111.1,111.7]), and `player` and `opponent` a player ([CR#102.1..102.3]). Under the
copula `is(subject, predicative)`, `Predicative`'s noun-phrase form `noun(phrase)` tests its
subject in that subject's own zone without [CR#109.2]'s battlefield restriction ("If it's a land",
of a card in a library; "if it's a creature card" adds the card class through `card`). A
coordination at `Nominal` is a union of the domains its arms describe ("artifacts and creatures"),
whereas modifiers in `pre` and `post` intersect with the head; an Agreement `PostModifier` (`agree`)
constrains the enclosing plural determiner's group rather than each member alone.

A **characteristic read** ("its power") may select a departed object by its last known information
([CR#608.2h]). Such a read never makes the object the patient of a battlefield action. Oracle text
writes a departed object's information in the past tense, so a present-tense guard over a
description with no zone or "card" reads the permanent as it is.

**Participial Descriptions** ("the exiled card") require the unique referent the deed stamped.
They are the only references that reach objects a linked ability published ([CR#607.2a]).
"This way" reads only the current ability's stamps, since it is no linking phrase ([CR#607.1]).
Only a Participial Description written without "this way" reaches a linked ability's objects.

### 2.5 The Contrast Anchor

A bare printed "another" or "other" (`other`) excludes its Contrast Anchor, which need not be a
target ([CR#702.151a]). `otherThan` is only for a printed "other than". Resolution finds the
anchor like any other reference (§2.4).

## 3. What a Clause Means

What a term is built from, and how an event, a static statement and a lookback are written, is
[`surface.md`](./surface.md) §6 and §7.

### 3.1 Before and After the Event

A replacement must see the world the replaced event would change, so `would` reads its Action
**before** the event. Its patients are still in their old zone, and
prospective amounts such as "that many" exist, but products such as a drawn card do not
([CR#614.6]). Every other context reads its Action **after** the event, except for the two reads
below: patients have moved, are stamped with the deed, and become new objects where [CR#400.7]
applies.

Two reads are of the world before the event. A look-back trigger pattern (§3.2) reads its own
descriptions as the objects were immediately prior to the event ([CR#603.10,603.10a]), so "Whenever
a creature dies" matches a permanent that was a creature on the battlefield just before dying; only
the trigger's body reads after. A lookback's historical subject or patient (§3.4), and a
Participial Description of a departed object ("the sacrificed creature"), read the object as it was
when the event happened or by its last known information ([CR#608.2h]), found by the recorded
origin (§2.1). An entering permanent's replacement effects read it as it would exist on the
battlefield ([CR#614.12]), and playing a land ("play an additional land", "play lands from your
graveyard") names the land permanent the special action puts onto the battlefield from a land card
([CR#116.2a,305.1]), so a card-type head at `Nominal` keeps its permanent reading in each place.

### 3.2 The Event Gate

Events, lookbacks and deontic rules admit only the event-capable Action heads, plus a keyword-action
Definition matched by its Deed. An event pattern, a lookback or a prohibition must name something
that happens as one event; control flow (`sequentially`, `doIf`, `doForEach`, `chooseModes`, …)
and compound actions are not events, so they are refused. A deontic rule has no gate of its own,
since a "can't" outranks anything that allows the thing ([CR#101.2]).

- **The Observation Point is derived, never written.** No event head or trigger carries it. Each
  arm of a trigger pattern looks back in time when it names an event on the closed list of
  look-back triggers. The list is: leaving the battlefield, a sacrifice, leaving a graveyard, and
  an object all players can see put into a hand or library ([CR#603.10a]); phasing out, becoming
  unattached, a player losing control of an object or an opponent gaining control of it from that
  player, a spell being countered, and a player losing the game ([CR#603.10b..603.10f]). Any other
  arm observes the event after it ([CR#603.10]), and a pattern "from anywhere" is never a
  leaves-the-battlefield one ([CR#603.6c]). Arms of one `or` can differ: *Syr Konrad, the Grim*'s
  "another creature dies", "a creature card is put into a graveyard from anywhere other than the
  battlefield" and "a creature card leaves your graveyard" look back, look after and look back.
  The derivation reads an arm's event through `triggerEvent`, through the disjunction of
  trigger conditions, which cards print with "and" ([`surface.md`](./surface.md) §7.5; each
  trigger condition of it is an arm, as each arm of an event `or`
  is), and through `timed`, whose Timing is read at the same point as its event:
  *Market Gnome*'s "is exiled from the battlefield while you're activating a craft ability" is a
  leaves-the-battlefield arm, so it looks back, and so does its "while" ([CR#603.10,603.10a]).
  [CR#603.10a] sorts abilities, not occurrences; this design reads its publicly visible class per
  occurrence, so the point is not a function of the written pattern alone. Under that reading
  *Dutiful Knowledge Seeker*'s "put into a library from anywhere" looks back for a card from a
  graveyard and not for one from a hand; no ruling settles it.
- **Damage with no kind written** is the effect's own noncombat damage in an instruction, and all
  damage in a pattern ([CR#120.2b]).
- **A counter or die pattern watches its written amount.** `1` matches each occurrence; `anyAmount`
  matches the event ([CR#603.2c,715.2b]).
- **`remove` is no event head.** What it names, an effect taking an attachment off its host by
  way of a state-based action ([CR#704.5m,704.5n]), is no one event, so `remove` names no Deed
  and is refused here. How it is written is [`surface.md`](./surface.md) §7.6.

### 3.3 Static Statements

- **An Unless Condition** ("<subject> can't <action> unless <condition>", "<subject> <action>s if
  able unless <condition>") is read inside its deontic rule's box (§2.2 item 3), after the rule's
  Subject and Action, so it reaches the Subject's noun phrase and the Action's patients, a
  hypothetical indefinite among them. [CR#205.4e]'s "A player can't cast a legendary instant or
  sorcery spell unless that player controls a legendary creature or a legendary planeswalker" reads
  "that player" as the Subject's "a player"; *Pithing Needle*'s "Activated abilities of sources with
  the chosen name can't be activated unless they're mana abilities" reads "they" as the abilities.
  The rule applies to each participant it ranges over exactly when the condition, read of that
  participant, does not hold, so a singular anaphor may read that participant (§2.4 item 2): read
  ahead's "Chapter abilities of this Saga can't trigger … unless … that ability" ([CR#702.155a]).
- **A cast or play permission widens only what it states**: the zone its `from` names, its Timing or
  as-though, the additional land plays it counts ([CR#305.2]), and the zone a moved referent was put
  into or its subject names ("you may cast that card" after an exile, §2.1; *Tibalt, Cosmic
  Impostor*'s emblem's "You may play cards exiled with Tibalt, Cosmic Impostor"). With none of these
  it grants no zone and no timing, so it only says how its subject may be cast: *Massacre*'s "you
  may cast this spell without paying its mana cost" and evoke's "You may cast this card by paying
  [cost] rather than paying its mana cost" ([CR#702.74a]) are alternative costs of a cast the rules
  already allow, as they allow a cast from the hand
  ([CR#601.3,118.9,301.1,302.1,303.1,304.1,306.1,307.1,310.1]).
- **A permission's alternative or additional cost applies only to a cast it describes.** Its `from`
  bounds that cast as well as granting it: warp's cost applies to a cast from the hand, not to the
  later cast from exile its delayed trigger allows ([CR#702.185a]). A permission that states no zone
  bounds none, so its cost joins a cast any other rule or effect allows, as a morph card is cast
  face down from the top of a library under *Garruk's Horde* ([CR#702.37c,601.3e]).
- **Being an effect follows from the enclosing node.** No Action carries an effect flag, so a
  Saga's turn-based lore counter is no effect ([CR#614.16,715.3c]). A performer and a cause are
  independent. `thisEffect`, what a sentence calls "this effect", is found the same way: it is the
  effect of the enclosing spell or ability.
- **Damage is composed, not special.** Damage results are conferred properties
  ([CR#120.3a,120.3c,120.3e,120.4c]). Wither and infect replace the marking results
  ([CR#702.80a,702.90b]). Prevention, redirection and scaling replace the damage event
  ([CR#615.1,614.9]).
- **`statOf` for loyalty and defense reads by zone:** counters on the battlefield, the printed
  number elsewhere ([CR#306.5c,310.4c]).

### 3.4 Lookback

A lookback is `did(subj, action, within)`, with an optional Event Ordinal. Its subject and its
patients are read as they were when the event happened, through the zone change's recorded origin
(§2.1, §3.1): "that creature" in "If that creature dies this way" is the departed permanent.

- `thisWay` reads an event its own ability's earlier text caused ([CR#608.2c]). `triggering` reads
  the event its ability triggers on ([CR#603.2]). Both must be grounded in the text. Whether the
  event happened is the engine's concern.
- `thisObject` reads the object's own casting or activation in its current existence ([CR#400.7]).
- What an optional body or arm stamped escapes it only to Participial Descriptions, never to
  pronouns. A target it announced is an ordinary referent after it ([CR#601.2c,603.3d]). A "when you
  do" trigger is a separate ability whose targets do not escape ([CR#603.12]).
- Linked and self-linked abilities share one authored tree. Lowering finds the link.

### 3.5 Standing Mechanisms

- Splice is a reveal made as the spell is cast, not a replacement of the cast ([CR#702.47b]).
- Subtype and counter state-based actions are Conferrals on their definitions
  ([CR#704.5m,704.5q]).
- An ability's text publishes its choices, cost results and introduced objects to the abilities
  after it, never what its trigger event names ([CR#607.1]).

## 4. Reading a Term

Semantics takes the expanded surface tree and gives a meaning or a refusal. It is one reading
in four steps, stated in the order of what each needs. Whether one pass in that order is enough
is open (§6).

1. **Classification.** Each clause is classified from where it stands and the words it holds,
   with no reference resolved:
   - the clause of a trigger is an event pattern, which must name one event (the event gate of
     §3.2, read through `done`, `be`, Templates and coordinations as
     `voice-and-object-phrases.md` §8 states), or a state, for a state trigger [CR#603.8]:
     *Dandân*'s "When you control no Islands, sacrifice this creature.";
   - a clause under "if", "as long as" or "unless" is a condition: a state, a lookback, or a
     choice a player is offered or has made, as in *Mana Leak*'s "Counter target spell unless its
     controller pays {3}." [CR#118.12a];
   - a static ability's clause is a continuous effect, a deontic rule, a replacement effect, a
     cost statement or one of the other static statements, by its verb phrase;
   - a sentence with `instead` is a replacement effect [CR#614.1a], and one that has damage
     prevented is a prevention effect [CR#615.1a]; one that says damage can't be prevented is
     not. The event is the clause it answers, which usually holds a
     `would` (*Phytohydra*) and sometimes does not (*Contamination*'s "If a land is tapped for
     mana, it produces {B} instead of any other type and amount."). A clause whose subject
     carries a `would` relative is one too (`voice-and-object-phrases.md` §8);
   - a `subject` under `done` is given the role of the slot it stands in, and a `done` call's
     performer is its `by` or nobody;
   - a clause states what holds or states a deed, which decides the force of its indefinite
     subject (`voice-and-object-phrases.md` §5).
2. **Boxes.** Each classified clause and operator opens its boxes (§2.2): a
   trigger, a conditional and a replacement are duplex, a single-clause replacement among them;
   `cant`, `must`, `can`, a negation and an "otherwise" are negated or modal; a distributive
   opens a loop. A counting subject is read inside its operator's box.
3. **References.** Every mention introduces a Referent or resolves to one, in printed order, by
   the accessibility rules of §2.4. A Definition's body is read in its own
   Reference Scope and exports its outcomes to the caller (§2.3), which is what *Cursed
   Windbreaker*'s "manifest dread, then attach this Equipment to that creature" needs. §2.3 does
   not say what an outcome is (§6).
4. **Construction.** The meaning is built from the classified clauses and resolved references: an
   event with its subject and Action, a Static Spec, a Replacement. These are the values today's
   term language names with sorts, built as Rust values, and lowering reads them. Where the
   Comprehensive Rules distinguish two structures the meaning does too (§1 item 5(a)).

**A distinct cue in the English is a distinct word in the term.** Classification reads
primitives and positions, never which Template a card calls, since a Template adds no meaning
(`architecture.md`, design principle 3). So "instead" is a primitive, and an intervening "if"
keeps its own slot after the trigger condition [CR#603.4]: each is printed structure that
classification needs.

A replacement is written with the ordinary conditional word: no word of its own marks the
sentence, and classification reads the replacement off its `instead`. A Definition's body is
classified at each call, in the expanded tree.

**A word only a rule may use is refused in a card.** The rules are transcribed into surface form
as cards are ([`surface.md`](./surface.md) §4), and some of their words name what no card prints
(`asEvent`, `stateBasedActionsChecked`). No sort separates them from a card's words, so
classification refuses a card's term that calls one.

Spelling is the other bound on what a card's term may hold. A card's stored term must be among
the candidates recovery gives for its text and spell back to it (§5), so a word with no wording,
or with a wording the card does not print, has no route into it. For a card read from its text
that bound does most of the work, and the refusal above matters for a term written by hand.

Which words those are is read from the declarations. Every primitive has a declaration of its
own, which is where a wording attaches ([`syl.md`](./syl.md) §1), so a word only a rule may use
is one whose declaration has no `spelling`. A word that stands for a Gap is spelled with the Gap
mark (`spelling.md` §7), and the one word a card may hold that has no `spelling` is the promoted
`subject` of a passive. No separate list names them.

**Why one layer.** Which box a clause opens depends on what the clause is, reference resolution
needs the boxes, and the meaning needs resolved references, since an event pattern's subject is
a Referent. A reference step in a crate of its own would have to restate what each clause and
operator means.

**Each step answers one of three ways** for a term or a part of one: it is accepted; it is
refused, with the rule that refuses it; or it is not understood, because semantics has no rule
for the construction yet. Only a refusal says a reading is wrong. A term that is not understood
is reported as that, and is never dropped as though it had been refused (§5).

## 5. From Text to a Stored Term

1. **Recovery** gives the set of candidate surface terms whose wording matches a Reading in the
   card's Packed Forest and which spell back to the analyzed text. Candidates are not one per
   Reading: one Reading gives several where Constructions word alike, and several Readings give one
   term, which is one candidate.
2. **Semantics** reads each candidate. A candidate it refuses at any step drops out. A candidate
   it does not understand stays, marked, and a card with one stores no term by this pipeline:
   it has no meaning to class or rank, and the right reading may be the one not understood.
3. **Candidates with one meaning are one class.** Two surface terms that assemble to the same
   meaning differ in nothing the game can see.
4. **Pragmatics ranks the classes** when more than one is left
   ([`pragmatics.md`](./pragmatics.md)). Two classes ranked equal are a tie: it is reported and
   no term is stored.
5. **The top class's term is stored** as the card's `.syl`. Within a class the term is chosen by
   a fixed rule, so the same text always stores the same term.

Where no candidate is left, the card is reported and nothing is stored. The pipeline stores a
term only for a card that has none: it never writes over a stored term.

**The stored term is the decision, and rank only proposes it.** Reading a stored term to play
it runs semantics and never pragmatics; the check of a stored card may run pragmatics to report
where its term ranks. A wrong choice is corrected by editing the file. What is checked
for every card is that its stored term is among the candidates recovery gives for its text, that
semantics accepts it, and that it spells back to the analyzed text (`architecture.md`, "Exact
round trip"); where it ranks is reported and gates nothing, as a Reading's rank gates nothing
(`english.md` §5).

**The report** for a card gives the number of candidates at each step, the rule that refused
each dropped candidate, each candidate that was not understood, and, where pragmatics chose,
the preference that decided. Two classes that pragmatics ranks equal are reported as a tie, and
no term is stored for that card until an author picks one.

The steps above speak of candidates one at a time. `english.md` §4 has recovery run over the
forest and enumerate no tree, since a chain of unselected Preposition Phrases has Catalan-many
attachments, and `spelling.md` §8 calls enumerating trees a defect. Whether classes of meaning
are found over enumerated candidates or over a packed form is open (§6).

## 6. Open

1. **Which sorts are kinds of phrase.** [`surface.md`](./surface.md) §1 gives the test and applies
   it to the four sorts the voice spec worked around. `plugins/builtin/` declares words at more
   than a hundred sorts, among them `CharacteristicEdit`, `MoveRider`, `CostShift`, `Cost`,
   `Timing` and `TriggerCondition`, and none of those has been put to the test. Each is a kind of
   phrase the text prints, a classification that moves into the meaning, or part of the frame a
   rule is written in ([`surface.md`](./surface.md) §4, and item 7).
2. **The pragmatics defaults and the form of the data.** [`pragmatics.md`](./pragmatics.md) §4
   keeps four hypotheses for one kind of ambiguity and settles none. The list of ambiguities
   that reach pragmatics and the default for each come from the first run of §5 over the corpus.
   "This creature is tapped" under "as long as" (*Centaur Omenreader*) has a copular and a
   passive Reading, and no rule here or there picks.
3. **How candidates of one class are told apart for storing** (§5 step 5): the fixed rule is not
   chosen (item 6).
4. **Sentences that depend on an earlier one.** "If the player does, untap that creature."
   (*Apathy*) and the other follow-ups of `voice-and-object-phrases.md` §13 items 6 and 7 are
   ordinary sentences at the surface, and what links each to the choice or deed it follows is
   reference resolution's. Which Referent an optional deed introduces for "does" and "do" to
   read is not designed.
5. **Words that meet at one sort.** A name is unique at its sort (`syl.md` §3), and clauses that
   stood at `Condition`, `GameEvent`, `StaticSpec` and `Replacement` stand at `Clause`. `doIf`
   is declared at `VerbPhrase` and at `Replacement`, `replace` at `VerbPhrase` and at `StaticSpec`,
   `nthOccurrence` at three sorts and `or` at ten. Each set that meets is one word, where its
   members print alike and differ only in what they mean, or needs names that tell its members
   apart. None has been examined.
6. **When two meanings are the same** (§5 step 3). Equality has to ignore the identities
   reference resolution allocates and keep printed order; it is not defined. The fixed rule of
   §5 step 5 cannot be the Reading's Ranking Key alone, since two terms from one Reading share
   it.
7. **Which rules data becomes clauses and which sorts stay as frames**
   ([`surface.md`](./surface.md) §4): a fair amount of the first and a fair few of the second,
   declaration by declaration. With it, what replaces `Condition` and `VerbPhrase` where a clause is
   held: `SbaRule`'s `when` and `then`, the `intervening` parameter of `triggered`, a deontic
   rule's Unless Condition.
8. **Whether the four steps run once in order.** "The same is true for …" (*Soulflayer*) takes
   its clause's kind from the sentence it refers to, so classification there waits on a
   reference. A prevention's event and its "prevent 1 of that damage" are joined by a reference
   (*Urza's Armor*), and a later sentence may read a Referent the event introduced. A `shared`
   phrase's `_` is read before the phrase it stands for.
9. **What a Definition's outcome is** (§2.3), which decides what its body exports: the creature
   manifest dread puts onto the battlefield is chosen from the cards looked at, and §2.3 drops
   partitive Referents.
10. **An anaphor with more than one accessible Referent.** §2.4 says which Referents an anaphor
    may read and not which it reads when several qualify. Two meanings that differ only in an
    antecedent are one surface term, so the stored term cannot carry the choice: it is made by a
    fixed rule in the reference step each time a term is read, and that rule is not designed.
11. **Enumerated candidates or a packed form** (§5).
12. **Whether the Gap of a relative clause and the Gap a shared phrase fills are one word.**
    `which` and `_` are both the Gap at `NounPhrase`, each spelled `^` (`spelling.md` §7), and
    what fills each is decided by the construct around it. The grammar has other kinds of Gap
    (`Gap` in `crates/syntax/src/step.rs`), which the term language writes with fixed-wording
    Templates (`voice-and-object-phrases.md` §7). That at most one Gap of the right sort stands
    in a phrase is a checker refusal today and could be a property of the Rust structures.
    Against merging: `which` is nearer to `subject` than to `_`. `subject` reads what its verb
    phrase is said of and `which` what the whole postmodifier is said of, and the two coincide
    wherever a postmodifier is a bare verb phrase, which is why a subject relative and a
    participle write no Gap word. `which` is needed only where a relative clause has a subject of
    its own ("spells you cast" is `clause(you, did(cast(which)))`, where `subject` would read
    "you"). `_` is filled from the other side, by the phrase after a coordination. No card found
    prints a clause that relativizes an object and shares a phrase.
13. **Depictives.** "Put it onto the battlefield tapped and attacking", "enters tapped" and
    "attacks alone" each end in a predicative that describes the subject or object as the event
    happens, which the English grammar reads as a `Depictive`. The term language writes the ones
    on a move as a `riders` list at the sort `MoveRider`, whose words print as the description
    words do ("tapped", "transformed"), and the others as Templates of their own. At the surface
    a depictive is a dependent of its verb phrase, built from the description words, and what it
    means on each verb (an Entry Rider on an entering) is classification's. It belongs with the
    verb phrase's other unselected dependents (`voice-and-object-phrases.md` §13 item 1) and is
    not designed.
14. **An anaphor whose antecedent is printed after it.** §1 item 2 says reading order does not
    limit the direction of reference, and §4 step 3 resolves each mention in printed order by
    the rules of §2.4, which say nothing of a later mention.
15. **No way to correct an antecedent.** If the fixed rule of item 10 picks the wrong Referent
    on some card, the card's term cannot say so: it holds no labels, and pragmatics does not run
    on a stored term.
16. **A stray candidate that is not understood.** Recovery gives every term the wording allows,
    and one of them that semantics has no rule for keeps a card from storing its right term
    (§5 step 2). Refusing what is not understood instead would drop real constructions.
17. **Boxes for modes.** §2.2 has no box for `chooseModes`: a mode is neither a branch that
    closes nor top-level text.
