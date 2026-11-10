# Game Model

Terms the project coins for its model of Magic. A Magic rules term means what the Comprehensive
Rules say and is not redefined here; the CR outranks this file. English grammar terms live in
[Oracle English](../oracle-english/CONTEXT.md). A spelling used in both contexts (Object,
Predicate, Subject) means what the glossary of the context using it says.

## Declarations

**Primitive**:
A `.syl` declaration that binds a word to one model constructor; the only place a constructor's
spelling attaches.

**Template**:
A named expansion for printed wording the rules do not define. It expands into what its parts
would be written as and adds no meaning. It binds its parameters as a Definition does; in expansion
it differs from one only in having no Reference Scope and no Deed.

**Definition**:
A named expansion for vocabulary the rules define: a keyword action or ability
([CR#701.1,702.1]), a Counter, a Designation, a Defined Token. It carries its own Reference
Scope; a keyword action's Definition also carries a Deed, as does a keyword ability's where text
can ask who did it (crewing, [CR#702.122b]). A Counter's, Designation's or Defined Token's
carries none. A Defined Token's takes no arguments, so its scope holds nothing of its caller's.

**Coercion**:
A nameless `.syl` declaration through which a value of one sort stands at another by way of an
existing Primitive: a Card Type or Subtype as a Head Noun, an integer as an Amount. It applies
only where the value does not stand at the expected sort itself, and never twice in a row.

**Defined Token**:
A token whose characteristics the rules define in full: a predefined token ([CR#111.10]) or one
a keyword action's rule defines, as empower Jace defines a Jace planeswalker token
([CR#701.71a]). Its Definition is named by the key an effect writes to create it ("create a Clue
token").

**Primitive Keyword Ability**:
A Keyword Ability the engine implements natively rather than as a Definition.

**Composite Keyword Ability**:
A Keyword Ability made of component Abilities that stay parts of the one named keyword.

**Registry Definition**:
What one registry declaration (a Counter, Subtype, Card Type, Supertype or Designation) means:
the rules content its name stands for.

**Permanent Type**:
A Card Type that is one of the six permanent types: artifact, battle, creature, enchantment,
land and planeswalker ([CR#110.4]). Whether an object is a permanent card or spell, or an
instant or sorcery, follows from the types of the face it has, so a card's faces can differ
([CR#110.4a,110.4b]).

**Same-Is-True-For Ability**:
One Ability printed for the first keyword of a list and applied in turn to each keyword "the
same is true for" ([CR#702.1c]).

## Discourse

**Read Order**:
The order in which a term's arguments are read: printed order.

**Referent**:
A discourse entity with a stable identity, created by the mention that introduces it; a
re-mention resolves to it.

**Box**:
A discourse scope (top level, conditional, negated or modal, distributive, relative clause) that
decides which Referents later text can reach.

**Reference Scope**:
The Referents a reference can reach. A Definition body has its own, seeing only its arguments;
`actor` reads the current Actor.

**Binding Identity**:
The identity a Referent is bound under, fixed at its first mention; later mentions resolve to it
without a pronoun search.

**Head Noun**:
The Object class, Card Type or Subtype a reference names its Object by ("this creature", "this
Aura") ([CR#109.2]).

**Participial Description**:
A description naming what a Deed or zone change did to its referent ("the exiled card", "each
creature destroyed this way"). It is the only reference that reaches what a linked ability acted
on ([CR#607.2a]).

**Contrast Anchor**:
What a bare "another" or "other" excludes when no "other than" is written.

## Actions

**Action**:
The bare infinitive verb phrase of one verb ("sacrifice a creature"). One Action serves every
context: imperative, cost, trigger, replacement, prohibition, lookback and negative declarative
("this effect doesn't remove this Aura").

**Instruction**:
One executable step of core, the engine's AST. An Action is semantic and lowers to instructions.

**Deed**:
What a card's text says is done: a core-rules deed, a keyword action ([CR#701.1]), or a verb a
keyword ability defines that text can ask about, as crewing ([CR#702.122b,702.122c]). It is what
triggers, lookbacks and prohibitions name. Putting a counter and gaining a designation are none.

**Subject**:
Who or what an Action is said of in its clause: the active `actor(np)`, the agentless passive
`any`, or the passive with an agent `by(np)`. Its noun phrase is a Referent, which pronouns and
demonstratives read ("whenever a creature dies, … it"); it is not the Actor.

**Patient**:
What an Action is done to. It stays inside the Action; the Subject supplies only the performer.

**Actor**:
Who or what performs an Action. In an instruction, the controller of the text's spell or ability
([CR#109.5]), unless a Handoff names another; in a clause, what its active Subject or its `by`
agent names. It is a role that only the `actor` noun-phrase word reads, not a Referent; the noun
phrase naming it is an ordinary Referent.

**Handoff**:
`instruct(subject, action)`: an Action making a named Player or Permanent the Actor of the
Action it contains ("target player discards a card").
_Avoid_: rebinding "you"; inside a Handoff "you" still means the controller ([CR#109.5]).

**Rules Procedure**:
Activating, casting or playing, or a resolution, as a Deed text can name or have a Player carry
out (`procedure`) ([CR#601.2,602.1,608.2]).

**Observation Point**:
Whether a trigger observes an event just before or just after it happens. It is derived from the
trigger pattern's arm and the occurrence, and written nowhere ([CR#603.10,603.10a..603.10f]).

**Trigger Condition**:
What a triggered ability triggers on. The `TriggerCondition` sort's wording includes its
trigger word ("when this creature enters"); the CR's form "[When/Whenever/At] [trigger condition
or event], [effect]" uses the term for the part after the word ([CR#603.1]). One ability may
have several, each with its own trigger word, and triggers on any of them ([CR#603.1b,603.2]);
cards print "and" between them.

## Statics and rules data

**Static Spec**:
What holds while its carrier is in force: the payload of a Static Ability, or of a one-shot's
effect with a Duration. Applying it establishes a Continuous Effect ([CR#611.2]).

**Deontic Exception**:
The one source a "can't" exempts ("can't be blocked except by artifact creatures",
[CR#702.13b]).

**Unless Condition**:
The condition under which a deontic rule does not apply ("A player can't cast a legendary instant
or sorcery spell unless that player controls …", [CR#205.4e]). It is read inside the rule, so it
reaches the rule's Subject and patients.

**Permission Rider**:
The rest of a permission's ability that modifies the act it allows, applying only to acts done
under that permission ([CR#113.2c]).

**Entry Rider**:
How a Permanent enters the battlefield, stated with what puts it there: tapped, attacking, with
counters, attached ([CR#614.1c]).

**Characteristic Edit**:
A change that sets, adds or removes a characteristic or a value within it, such as added or
removed abilities or a new mana cost.

**Ability Selection**:
Which abilities a Characteristic Edit names: specified ones, all except some, or those printed
below a keyword ([CR#702.165a]).

**Text Change**:
A Characteristic Edit to rules text: replacing a word, removing bracketed words, "target" to
"each", appending or exchanging text ([CR#612.1]).

**Mode Change**:
A Static Spec changing how its own spell's modes are chosen: all modes, extra modes for a cost,
a different number, or repeats ([CR#700.2]).

**Resolution Delayed Trigger**:
A Static Ability of a Permanent Spell that creates a delayed triggered ability as the permanent
it becomes enters while the spell resolves, as dash and blitz do ([CR#608.3g]).

**Off-Stack Static**:
A Static Ability that carries out an Action without using the stack, as an event happens or
whenever a condition holds, as daybound and ascend do ([CR#702.145b,702.131b]).

**Roll Modifier**:
An Ability that changes a die roll before its result is final; not a trigger ([CR#706.2]).

**Mana Substitute**:
An act a Player may do instead of paying one mana of a cost, as convoke and delve are
([CR#702.51a,702.66a]).

**Conferral**:
A rule, Card Type, Subtype, Supertype, Counter or Designation supplying an Ability or rules
property to what has it, as a basic land type confers its mana ability ([CR#305.6]).

**Combatant**:
A Permanent the creature-like combat rules apply to, whether or not it may attack or block now
([CR#508.1a,509.1a]).

**Designation**:
A named marker an Object, a Player or the game holds that is neither an Ability nor a copiable
value: the monarch, goaded, day ([CR#701.15b,731.1]).

**Relation**:
A named link between two Objects that one ability makes and others read: haunting, soulbond,
cipher ([CR#702.55b,702.95b,702.99b]).
_Avoid_: Designation, which marks one thing, not a pair.

**Attachment Host**:
The Object or Player an Aura, Equipment or Fortification is attached to ([CR#701.3a]).

**Linked Facet**:
What a linked ability reads back about a cost: whether it was paid, how many times
([CR#607.2i,702.33d]), or the mana spent on it.

**Link Source**:
The cost a Linked Facet reads back: a keyword's cost, or "the additional" or "the alternative"
cost ([CR#607.2i]).

## Values

**Entity**:
A Player or an Object ([CR#102.1,109.1]).

**Predicate**:
A truth condition tested against a candidate.

**Selection**:
In core, a resolution-time group or query over Objects, such as all matching Objects or a
group held in a register.

**Sharing**:
A Predicate true of an Object that has a value in common with another on one characteristic
(`shares`: "a creature that shares a color with it") ([CR#105.2,205.3m]).

**Agreement**:
Whether the members of a group share, or do not share, a value on one characteristic (`agree`:
"two creatures that share a creature type"). It holds of the group, not of one member.

**Core Deed**:
A Deed the core rules define outside the keyword actions: attacking, blocking, drawing,
targeting and the rest ([CR#508.1,509.1]).

**Amount**:
An expression denoting a number: a literal, a count or a characteristic read.
_Avoid_: Quantity, which bounds a count.

**Quantity**:
A determiner's count constraint: "one", "up to two", "any number".

**Delta**:
A change to a numeric property: up, down or set to an Amount.

**Mana Cost Term**:
The mana a cost change is stated in: printed symbols, or an Object's mana cost ([CR#202.1a]).

**Location**:
Where Objects are or a move takes them: a Zone, a position in a pile, anywhere, or a
combination ([CR#400.1]).

**Slice**:
A number of Objects counted from one end of a pile, still in its zone ("the top three cards of
your library") ([CR#700.3c]).

**Run**:
The Objects taken from one end of a pile up to and including the first that matches a
description ([CR#701.57a]).

**Duration**:
How long a one-shot's Continuous Effect lasts; with none stated, until the game ends
([CR#611.2a]).

**Timing**:
When a Player may act or a trigger or replacement applies; none written means whenever the
Player has priority ([CR#117.1a]).

**Turn Point**:
A moment in the turn structure that an "at" trigger names: the beginning of a phase or step
([CR#603.2b]), or of a turn, picked by whose it is and which occurrence; "at end of combat" names
the end of combat step's beginning ([CR#511.2]). The game reaching one is an event.
_Avoid_: Moment, Step trigger.

**Event Ordinal**:
Which matching event a description picks by position ("the second spell you cast each turn")
([CR#700.1]).
_Avoid_: Usage Limit, which counts what may be done.

**Usage Limit**:
How many times something may be done per turn, game or existence ("activate only once each
turn", [CR#602.5b]).

## Engine

**Execution Frame**:
The context that supplies the bindings an ability's instructions read as it resolves.

**Decision Point**:
Where the engine waits for an external agent to submit a Decision.

**Decision**:
What an agent submits at a Decision Point.
_Avoid_: Choice, the CR's rules-level selection a Decision may encode.
