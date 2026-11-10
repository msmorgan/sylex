# ADR: Semantics — DRT Reference Resolution, Definitions and Templates, and the Action-Clause Spine

- **Companion:** [`syl.md`](./syl.md), the `.syl` data language this model is written in.

## Overview

A card's meaning is a `.syl` term that follows the card's English phrase by phrase. The pipeline
has four steps:

1. Parse the `.syl` files ([`syl.md`](./syl.md) §2).
2. Resolve names by expected sort, and expand Templates and Definitions ([`syl.md`](./syl.md)
   §3–§4).
3. Build a Discourse Representation Structure from the expanded term, resolve every reference
   against it (§2), and check the term against the action-clause spine (§4).
4. Lower the term to the engine's core.

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
   results, state-based actions, turn-based actions, intrinsic type rules ([CR#702.15b,704.5q,703.1]).
   Their CR citations, not the checker, vouch for their correctness. Why: a rule written as data is
   written once and read by every tool. A rule in engine code is a second source of truth that the
   model cannot see.
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
- Each mention has a grammatical role in its clause: subject, object or other (§4.2).
- **Actors are not referents.** A Handoff makes its Actor the innermost for the Action it contains.
  Pronouns, groups and counts never read an Actor; only the `actor` word does.
- A disjunctive noun phrase exports only the referents common to every branch. A coordination bound
  to a parameter binds every member under the parameter's one Binding Identity.
- A zone change keeps the referent and records the move on it: its new zone and the deed. Carrier
  words ("that card" after "exile target creature", [CR#109.2]), the consuming verb's
  applicability (§2.4) and Participial Descriptions read those. That the moved card is a new
  object in the game ([CR#400.7]) is game identity, which lowering supplies.

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
   ("there is …", `exists`) publishes its indefinite subject to the consequent.
3. **Negated or modal** (`cant`/`must`/`can`/`gatedBy`, `not`, an else branch). Hypothetical
   indefinite, counted and partitive participants are closed. Targets, definite references and
   universal domains stay accessible under their identities ([CR#101.2]).
4. **Distributive loop** (`doForEach`). Each pass binds the loop referent for the body to read.
   On exit, the loop exports plural referents for the targets the body announced and the objects
   it created. Each is a recorded discourse fact, not derived from determiners or stamps.
5. **Relative clause or comparison**, including the domain `countOf` and `aggregate` measure. It is
   closed on exit, so no later pronoun reads a measured domain. Attachment Hosts ("enchanted",
   "equipped") project to the ability's box ([CR#303.4b]).
6. **Definition Reference Scope:** see §2.3.

A self-replacement starts from the state the replaced action read, because the replaced event never
happens ([CR#614.6,614.15]).

### 2.3 Definition Reference Scope

A Definition body runs in its own Reference Scope. Why: the rules define a keyword independently
of the sentence that uses it. So a pronoun inside the body must not find the caller's referents,
and the body's own scratch referents must not leak out to the caller's pronouns.

- The caller's referents are hidden from it, except its arguments and the innermost Actor. A
  referent hidden by an enclosing Definition stays hidden.
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
2. Its number agrees with the anaphor's.
3. Its sort, zone and Head Noun fit the anaphor's carrier word ([CR#109.2]).
4. The consuming verb can apply to it in its zone ([CR#109.2,701.21a]). A creature that died
   cannot be sacrificed or have counters put on it as a permanent.

A **characteristic read** ("its power") may select a departed object by its last known information
([CR#608.2h]). Such a read never makes the object the patient of a battlefield action. Oracle text
writes a departed object's information in the past tense, so a present-tense guard over a
description with no zone or "card" reads the permanent as it is.

**Participial Descriptions** ("the exiled card") require the unique referent the deed stamped.
They are the only references that reach objects a linked ability published ([CR#607.2a]).
"This way" reads only the current ability's stamps, since it is no linking phrase ([CR#607.1]).
Only a Participial Description written without "this way" reaches a linked ability's objects.

### 2.5 The Contrast Anchor

A bare printed "another" or "other" (`isOther`) excludes its Contrast Anchor, which need not be a
target ([CR#702.151a]). `otherThan` is only for a printed "other than". Resolution finds the
anchor like any other reference (§2.4).

## 3. Definitions and Templates

A named word is a primitive, a Template or a Definition; why the language has the three is a
language matter ([`syl.md`](./syl.md) §1). A primitive only names a model constructor. The other
two differ in what they mean:

- **A Definition** is vocabulary the rules define: a keyword action or ability ([CR#701.1,702.1]),
  a Counter, a Designation, a Defined Token ([CR#111.10,701.71a]). It carries rules meaning and
  its own Reference Scope (§2.3).
  - A keyword action's Definition carries a Deed that triggers, lookbacks and prohibitions can
    name ("whenever you scry", "the card exiled this way", "can't be regenerated"). "This way"
    reads the current ability's stamps (§2.4), and the stamp a keyword action leaves is its
    Definition's Deed. That is why each keyword action has exactly one Deed (§3.1): a variant with
    its own Deed would make "exiled this way" miss it.
  - A keyword ability's Definition carries a Deed only where the rules let text ask who did it:
    a creature tapped to pay a crew cost "crews" the Vehicle, and the Vehicle is "crewed by" it
    ([CR#702.122b,702.122c]), so Crew's Definition carries the Deed paying that cost stamps.
  - A Counter's, a Designation's or a Defined Token's Definition carries none.
  - A Defined Token's Definition takes no arguments, so its Reference Scope holds nothing of
    the caller's; the ability text in its characteristics is embedded ability text (§2.3).
- **A Template** is printed wording only. It expands transparently into the expressions its parts
  would be written as, and it adds no meaning.

They are kept apart because conflating them goes wrong both ways. A phrasing given a Deed or a
scope invents a rules distinction the CR does not make: an event then matches it, and referents it
should see are hidden from it. A rules-defined keyword treated as a phrasing loses its identity, so
"can't X" and "whenever X" can no longer name it. So the kind a declaration states decides which it
is ([`syl.md`](./syl.md) §1). That a Definition's body is the rules' meaning of its keyword is
trusted, vouched for by its CR citation like any rule declaration (§1 item 4). Its reference
resolution and sorts are checked in each context that expands it.

### 3.1 Keyword Actions and Deeds

- Every keyword action is a Definition with exactly one Deed ([CR#701.1]). The exception is
  an action the rules define as another action: `convert` is a Template over `transform`
  ([CR#701.28a]).
- A keyword ability's verb that is done by doing another deed is a Definition over that deed:
  `devour` over `sacrifice` ([CR#702.82b]).
- With no Handoff to a Permanent, a deed a permanent performs is performed by the Source
  ([CR#701.14a,701.44a]). The checker never guesses a deed from a body's shape.

## 4. The Action-Clause Spine

### 4.1 One Action Per Verb

Every verb is one Action, its bare infinitive verb phrase. Why: otherwise each verb is encoded once
per clause mood (as an instruction, an event, a prohibition and a lookback), each copy with its own
subject and patient plumbing, and the copies drift apart. With one Action, "sacrifice" means the same
thing in "sacrifice a creature", "whenever you sacrifice", "can't be sacrificed" and "if you
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

Triggers, replacements, deontic rules and lookbacks share one head gate (§4.6), and every context
shares one reading of subject, action and agent. A context adds only what is its own.

### 4.2 Voice

Voice belongs to the Subject, not to the Action, so the passive needs no separate event. The
passive matters only because it promotes the patient to grammatical subject. The patient is then
read first, ahead of any agent, and that decides which occurrence of a parameter is its first
mention ([`syl.md`](./syl.md) §4). A Subject is one of three:

- `actor(np)`: active. `np` is read first and is the Actor.
- `any`: passive with no agent. The patient is promoted to subject.
- `by(np)`: passive with an agent. The patient is promoted and read first, then `np` with role
  other.

What an active Subject names need not be a player or a permanent: a spell or ability ("<ability>
triggers") or an effect (`actor(thisEffect)`, §4.7) can be one.

"Becomes blocked" is therefore not the agentless passive of `block`. It is its own event head,
`becomeBlocked`: the Blocked status beginning, once per combat, even when an effect blocks the
creature with no blocker ([CR#509.1h,509.3c]). "Becomes blocked by <B>" is `by(B)` over `block`,
one event per blocker ([CR#509.3d]).

### 4.3 `any`

There is one wildcard word, `any`, written at every sort that needs one. No sort has a wildcard
word of its own.

### 4.4 Patients and Relative Clauses

- A transitive Action always keeps its patient inside it. The Subject supplies only the performer.
- A relative clause filtering by an event is a lookback with `which` in the relativized slot.
- "The same is true for …" is one frame with a `thatKeyword` placeholder, checked per keyword
  ([CR#702.1c]).

### 4.5 Before and After the Event

A replacement must see the world the replaced event would change, so `would` reads its Action
**before** the event. Its patients are still in their old zone, and
prospective amounts such as "that many" exist, but products such as a drawn card do not
([CR#614.6]). Every other context reads its Action **after** the event: patients have moved, are
stamped with the deed, and become new objects where [CR#400.7] applies.

### 4.6 The Event Gate

Events, lookbacks and deontic rules admit only the event-capable Action heads, plus a keyword-action
Definition matched by its Deed. An event pattern, a lookback or a prohibition must name something
that happens as one event; control flow (`sequentially`, `doIf`, `doForEach`, `chooseModes`, …)
and compound actions are not events, so they are refused. A deontic rule has no gate of its own,
since a "can't" outranks anything that allows the thing ([CR#101.2]).

Event decisions:

- **A move is the one Zone Change event.** Its Observation Point is arrival or departure
  ([CR#603.6a,603.6c,603.10a]). `die`, `enter` and `leave` are Templates over it ([CR#700.4]). A
  move carries no cause; a caused move is wrapped in `causes` or `causedBy`.
- **Damage with no kind written** is the effect's own noncombat damage in an instruction, and all
  damage in a pattern ([CR#120.2b]).
- **A counter or die pattern watches its written amount.** `1` matches each occurrence; `anyAmount`
  matches the event ([CR#603.2c,714.2b]).
- **Pattern-only heads.** `gainControlOf` is a pattern head only; performed control change is
  `establish(controlGrant(…))` ([CR#613.1b]). `anyCost` and `anyBallot` likewise appear only in
  patterns.
- **Paying life** is a cost payment of `loseLife`; there is no life Cost ([CR#119.4,119.8]).
- **Alternative events are an `or` over events**, since a triggered ability may have more than one
  trigger condition ([CR#603.1b]). Each arm is a whole event with its own subject: "attacks or
  blocks" is `or([does(actor(x), attack), does(actor(x), block)])`, and "blocks or becomes
  blocked" joins two events whose subjects differ. An `or` over Actions would be a compound
  action, which the gate refuses.
- **Joined headers are not a disjunction.** "When this creature enters and whenever it attacks
  while saddled" is two trigger headers of one ability, each with its own trigger word,
  concurrent condition and timing; an `or` over events stands inside one header's event.
- **`remove` is no event head.** What it names, an effect taking an attachment off its host by
  way of a state-based action ([CR#704.5m,704.5n]), is no one event, so `remove` names no Deed
  and is refused here. It is written in a negative declarative, "this effect doesn't remove this
  Aura" ([CR#702.16n]).

### 4.7 Static Specs

- **A deontic rule** is a Subject, an Action and optional modifiers: Timing, as-though, a Deontic
  Exception, a cause (`cant` only), and a Permission Rider (`can` only) ([CR#113.2c]). "Can't A or
  B" is a conjunction of prohibitions.
- **A replacement's `repl`** says what happens:
  - `instead`: in the event's place.
  - `before`: before the event, which still happens; the choices "as it enters" requires
    ([CR#614.12a]).
  - `alongside`: as part of the event; "enters with" ([CR#603.6d]).
  - `withRiders`: the entering happens with Entry Riders.
- **Being an effect follows from the enclosing node.** No Action carries an effect flag, so a
  Saga's turn-based lore counter is no effect ([CR#614.16,714.3c]). A performer and a cause are
  independent. `thisEffect`, what a sentence calls "this effect", is found the same way: it is the
  effect of the enclosing spell or ability.
- **A negative declarative** `doesnt(subj, action)` says what its Subject does not do. It is no
  deontic rule and takes no modifiers. "This effect doesn't remove this Aura" is
  `doesnt(actor(thisEffect), remove(thisAura))`, its own Static Spec beside the spec it talks
  about. A `conjunction` joins the two in one static ability, so the sentence's references reach
  the spec's referents ("those artifacts"). Which consequence of the effect it exempts, an Aura
  put into its owner's graveyard or Equipment unattached ([CR#704.5m,704.5n]), is the engine's
  ([CR#702.16n]).
- **Damage is composed, not special.** Damage results are conferred properties
  ([CR#120.3a,120.3c,120.3e,120.4c]). Wither and infect replace the marking results
  ([CR#702.80a,702.90b]). Prevention, redirection and scaling replace the damage event
  ([CR#615.1,614.9]).
- **`statOf` for loyalty and defense reads by zone:** counters on the battlefield, the printed
  number elsewhere ([CR#306.5c,310.4c]).

### 4.8 Lookback

A lookback is `did(subj, action, within)`, with an optional Event Ordinal.

- `thisWay` reads an event its own ability's earlier text caused ([CR#608.2c]). `triggering` reads
  the event its ability triggers on ([CR#603.2]). Both must be grounded in the text. Whether the
  event happened is the engine's concern.
- `thisObject` reads the object's own casting or activation in its current existence ([CR#400.7]).
- What an optional body or arm stamped escapes it only to Participial Descriptions, never to
  pronouns. A target it announced is an ordinary referent after it ([CR#601.2c,603.3d]). A "when you
  do" trigger is a separate ability whose targets do not escape ([CR#603.12]).
- Linked and self-linked abilities share one authored tree. Lowering finds the link.

## 5. Standing Mechanisms

- `getsPt` carries both printed deltas ("+X/+0"), because spelling must render them back
  ([CR#613.4c]).
- A word replacement accepts only the replaceable word sorts, so a card name is unrepresentable
  ([CR#612.2a]).
- Splice is a reveal made as the spell is cast, not a replacement of the cast ([CR#702.47b]).
- Subtype and counter state-based actions are Conferrals on their definitions
  ([CR#704.5m,704.5q]).
- An ability's text publishes its choices, cost results and introduced objects to the abilities
  after it, never what its trigger event names ([CR#607.1]).
