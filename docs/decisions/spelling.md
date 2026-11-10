# Spelling

The spelling layer relates Readings to resolved terms ([Architecture](architecture.md), "The
layers"). Recovery reads the Packed Forest of a card's text and assembles the resolved terms
whose wording matches it; spelling a resolved term yields a Reading for the English layer to
realize. One grammar serves both directions, and this document says what kind of grammar it is.
Its data is the `spelling` field of each `.syl` declaration ([`.syl`](syl.md) §2). The terms
Construction, Hole, Tail and Factoring are the
[glossary's](../contexts/oracle-english/CONTEXT.md).

The grammar is a synchronous grammar of substitution and one-sided adjunction (§2), with three
closed extensions (§3). It has no wrapping adjunction. The resolved term is its derivation (§4),
and it changes nothing in the English grammar, in the shape of the resolved term or in what
expansion means (§5).

## 1. Constructions

A Construction is one declaration's wording paired with the call it spells. Every declaration
that carries a `spelling` is one: a Template, a Definition, and a Primitive too, since a
primitive is where a constructor's spelling attaches ([`.syl`](syl.md) §1). The wording is a
fragment of English structure with Holes, one for each parameter the wording prints. The call
is one node of the resolved term, with those parameters as its arguments.

A Construction pairs wording with the call, not with the call's body. The resolved term keeps
the call, and what the call expands to is expansion's business (§5). Why: a body may use a
parameter several times, as `getsBoth` passes its one delta to both power and toughness, and
may rearrange its parameters freely. If the pairing reached into the body, every such body would
be a place where one printed phrase stands for two parts of the meaning. Pairing with the call
keeps one printed phrase for one argument.

## 2. The two operations

**Substitution** fills a Hole with the wording of the argument's own Construction. It serves
every case where a phrase stands whole where its parent's wording leaves room for it: "draw
<amount> cards", "target <subject>", "<cost>: <action>". A wording whose fixed words stand on
both sides of its Holes is still substitution: "more <p> than <subject>" is one fragment with
two Holes, and nothing is inserted into another Construction's phrase.

**One-sided adjunction** attaches a Construction's wording to the left or the right of a
phrase that another Construction spelled, at a node on that phrase's edge, which need not be
its root. The phrase is the adjunct's host. Substitution cannot do this, because it only fills
room a wording left; it cannot put words inside or beside a phrase that is already complete.
Three things Oracle text does routinely need it:

- **An auxiliary or negation inside its Action's clause.** Abrupt Decay's "This spell can't be
  countered" is `cant(any, counter(thisSpell))`. The patient is an argument of `counter`
  ([Semantics](semantics.md) §4.4), yet it is printed before "can't be", and "countered" after.
  The words `cant` contributes attach inside the clause `counter` spells.
- **Modifiers on both sides of a noun from one list.** In "other nonland permanents you
  control" (Abdel Adrian, Gorion's Ward), "nonland" stands before the head noun and "you
  control" after it, and the term holds them as elements of one `and([…])`. Each modifier
  attaches on its own side of the nominal.
- **A trigger header over an effect of several sentences.** Boommobile prints "When this
  Vehicle enters, add four mana of any one color. Spend this mana only to activate abilities."
  In the Reading, "When this Vehicle enters," belongs to the first sentence. In the term,
  `when(event, action)` takes the whole effect, both sentences, as its `action`. The header
  attaches at the left edge of its host, inside the host's first sentence.

Why one-sided: in each case the adjunct's words are a single run that lies wholly to one side
of the node it attaches to. A grammar built from such attachments builds only trees a
context-free grammar builds, so every Reading the English parser already produces is one the
spelling grammar can describe, and the parser does not change (§5).

**There is no wrapping adjunction**, the operation of a full tree-adjoining grammar in which an
adjunct contributes words on both sides of its host at once. Why: nothing in in-scope Oracle
text needs it. The constructions that would are cross-serial pairings, and no in-scope card
prints "respectively" or "the former … the latter". What looks discontinuous in card text is
either fixed words around Holes, which is substitution, or one phrase printed away from the
unit it measures, which is a Tail (§3.2). Wrapping would buy capacity no card uses, at the price
of a larger class of derivations for recovery to search, and it still would not provide
Factoring (§3.3).

## 3. The three extensions

Substitution and one-sided adjunction leave three things Oracle text prints unexpressed. Each
gets one closed extension. They are the whole list: a wording that needs something else is a
defect in this design, reported and settled here, never patched in one declaration.

### 3.1 Optional and list Holes

**An optional part** is printed only when its parameter differs from the parameter's declared
default. It may carry fixed words beside its Hole: Elvish Piper's "put a creature card from
your hand onto the battlefield" prints "from <from>", and a card that names no origin prints
nothing there. Giant Growth's "Target creature gets +3/+3 until end of turn" and Glorious
Anthem's "Creatures you control get +1/+1" are one Construction whose duration is present in
the first and absent in the second.

**A list Hole** prints each element of a `List` argument and states how the elements join:
coordinated, with the commas and the coordinator English uses (Angelic Skirmisher's "first
strike, vigilance, or lifelink"), or juxtaposed, as riders and modifiers are. An empty list
prints nothing.

Why: without them a declaration needs a wording for every combination of its optional
parameters, and `when` alone has four optional parameters and a list. Wordings that differ only
in such a part become families of near-duplicate Templates, and about five in six of the
`TODO(spelling)` markers in `plugins/builtin/` stand for want of exactly this.

**Printed order governs the order of a list's elements**, in every list, the conjuncts of
`and([…])` included. The elements are written in the order the text prints them, the head among
its modifiers: "nonland permanent you control" lists the negated land type, then permanent,
then the control test. Which side of the head a modifier stands on comes from the modifier's
own Construction, and the list's order says the rest. Why: order among modifiers is wording.
Favorable Winds prints "Creatures you control with flying" and Blight Pile "creatures with
defender you control", so a list in any other order would lose which was printed. It is also
what read order requires ([`.syl`](syl.md) §5), and a list with one admissible order gives
recovery one term for a noun phrase where an unordered list would give it every permutation.

### 3.2 Tails

A Tail is part of a Construction's wording that is printed away from the rest: it attaches as
a one-sided adjunct at the edge of a phrase that encloses the Construction's Hole, and the
wording names the Category of that phrase. Quantity adjuncts are the case. In Kabira Outrider's
"target creature gets +1/+1 until end of turn for each creature in your party", the amount is
"+1 for each creature in your party", one `forEach(per, p)` inside the delta. Its `per` is
printed where the amount stands, and "for each <p>" is printed at the end of the verb phrase,
past the duration. "Equal to" does the same in Dogpile's "deals damage to any target equal to
the number of attacking creatures you control", where the recipient stands between "damage" and
what measures it.

Three properties make the extension closed:

- **On the English side a Tail is ordinary one-sided adjunction.** The Reading has "for each …"
  as an adjunct of the verb phrase, which is where the English grammar already puts it. Only
  the link is longer: the adjunct's meaning belongs to a call several arguments down from the
  call that spells the phrase.
- **A Tail is printed once however often its Hole is.** `getsBoth` prints its one delta twice,
  as "+1/+1", and the delta's Tail once ([`.syl`](syl.md) §2).
- **A Tail is read where it is printed.** Gaze of Adamaro "deals damage to target player equal
  to the number of cards in that player's hand": the amount is the declaration's first
  parameter, and its Tail mentions "that player" after the recipient introduced them. Read
  order is printed order ([`.syl`](syl.md) §5), so the walk that reads a term reads a Tail at
  its printed position, not at its Hole's.

Where the text prints a quantity both in place and as a Tail, or prints a Tail on either side
of another adjunct of the same phrase, the two are two wordings, and each is its own Template
(§4). Essence Backlash prints "deals damage equal to that spell's power to its controller" with
nothing detached. Aurochs prints "gets +1/+0 until end of turn for each other attacking
Aurochs" and Ral's Staticaster "gets +1/+0 for each card in your hand until end of turn".

Why: the alternative is a Template per verb. "For each" is printed after its clause's verb on
well over a thousand in-scope cards, under about thirty different verbs, and with substitution
alone each of those verbs needs its own "… for each" Template beside its plain one. A Tail lets
the one `forEach` serve them all.

### 3.3 Shared material at a coordination

English prints once what coordinated clauses share. Howling Golem's "Whenever this creature
attacks or blocks" has one subject for two events. Chub Toad's "Whenever this creature blocks
or becomes blocked" has one subject that stands in two roles, the performer of one event and
the patient of the other ([Semantics](semantics.md) §4.2). Predator's Strike's "Target creature
gets +3/+3 and gains trample until end of turn" has one subject and one duration over two
actions. The English grammar reads each as a coordination whose conjuncts share a dependent
([English](english.md) §3).

Substitution and adjunction, with or without wrapping, link one place in the wording to one
place in the term. Neither relates a phrase printed once to two places in a term, so a term
whose arms each hold the shared part needs a third extension. **Factoring** is that extension:
where the arms of a coordination would each print the same phrase at the same edge, spelling
prints it once, and recovery distributes the one printed phrase to every arm. An alternative
event is written with whole-event arms today, each with its own subject, since a triggered
ability may have more than one trigger condition [CR#603.1b] and the event gate refuses an `or`
over Actions ([Semantics](semantics.md) §4.6): Chub Toad's is
`or([does(actor(thisCreature), block), does(any, becomeBlocked(thisCreature))])`.

Factoring loses nothing where the unfactored form is never printed, and for alternative events
it is not: no in-scope card prints "Whenever X attacks or X blocks" with the subject repeated.
Where both forms are printed the term already differs. Ancient Craving prints "You draw three
cards and you lose 3 life" and Ambition's Cost "You draw three cards and lose 3 life", and
those are two terms: two Handoffs in sequence, and one Handoff over two Actions in sequence.

Whether a given coordination is spelled by Factoring or written as a term that coordinates
where the English does is not settled (§10).

## 4. The resolved term is the derivation

The resolved term stays a plain tree of calls, and that tree is the derivation: each call is one
Construction, and where it stands says what it was combined with. Nothing records a derivation
beside the term.

That holds under adjunction because an adjunct is always in one of two places in the term.
Either it is a call that takes its host as an argument, as `cant` takes its Action and `when`
its effect, or it is an argument, at some depth, of the call that spells its host, as a
conjunct of `and([…])` is and as the `forEach` inside a verb's amount is. No adjunct needs a
node the term would not otherwise have.

So wording is carried by exactly two things: **which Template a card calls, and the printed
order of a list's elements** (§3.1). Two wordings with one meaning are two Templates with one
expansion, or one list in two orders. Nothing is stored beside the term: no flag, no position
index, no record of which side or which order.

Each parameter of a Construction is one argument, and its argument is read once however often
it is printed or used ([`.syl`](syl.md) §4). A value the text prints once is written once
([`.syl`](syl.md) §2). A Hole that a wording prints twice is the same argument both times, and
recovery admits the wording only when the two printings agree.

Why: a derivation kept apart from the term would be something one direction of translation
invents and the other discards, which the canonical form does not store
([Architecture](architecture.md), design principle 2).

## 5. What spelling leaves alone

**The English grammar.** Its Productions, its Readings and its Packed Forest do not change.
A Reading is a tree, so it already has every Adjunct and Modifier on one side of the host
whose Category licenses it, and a shared dependent beside its coordination
([English](english.md) §3). Recovery
reads the Packed Forest and never parses text again ([English](english.md) §4): a
Construction's wording is matched against forest nodes. The grammar still names no Catalog item
and carries no meaning ([English](english.md) §1–2).

**The shape of the resolved term.** A term is a tree of calls with named and positional
arguments and lists. Adjunction, Tails and Factoring are relations between that tree and a
Reading. None of them adds a kind of node, and `.syl` gains no syntax outside the `spelling`
string.

**What expansion means.** Expansion substitutes a Template's parameters and wraps a
Definition's ([`.syl`](syl.md) §4), and a Template still adds no meaning
([Semantics](semantics.md) §3). Spelling relates Readings to the resolved term, before
expansion; the copying and rearranging a body does to its parameters happens after, where no
wording is involved. The discourse reading still reads the expanded tree in printed order, with
a Tail read where it is printed (§3.2).

**Meaning still disambiguates.** Spelling is many-to-many. Recovery yields every resolved term
whose wording matches some Reading in the forest, and a candidate the checker or the discourse
reading refuses drops out ([Architecture](architecture.md), "Spelling is a construction
grammar"). Nothing in this document chooses among candidates by rank or by rule order.

## 6. The round trip

The round trip over the analyzed Oracle text stays exact ([Architecture](architecture.md),
"Exact round trip"). The English layer's two laws carry it between text and Reading
([English](english.md) §7). Spelling carries it between Reading and term, and each extension
keeps it:

- An optional part is printed exactly when its argument differs from the default, so the
  argument decides the wording and the wording the argument.
- A list is spelled in its written order, which is the printed order.
- A Tail has one position for a given Template, so two printed positions are two Templates.
- Factoring is applied where the unfactored form is not a wording any card prints, or where
  the two forms are already two terms (§3.3).

Punctuation is not spelling's concern. A Reading stores none, and realization derives it from
structure ([English](english.md) §7). The square brackets of a cleave line [CR#702.148a] are
likewise restored outside any Reading, so a `spelling` never contains one.

## 7. What a `spelling` string must express

The formalism fixes what the `spelling` notation must be able to say. Its concrete syntax
belongs to [`.syl`](syl.md) §2. A `spelling` must be able to state:

- a Hole for a parameter, and the same Hole printed again;
- an optional part: the parameter that governs it, and the fixed words printed with it;
- a list Hole and how its elements join, coordinated or juxtaposed;
- a Tail: the words and Holes that are detached, and the Category of the phrase they attach to;
- nothing else by markup. Fixed words are plain text. Square brackets and braces are Oracle's
  own characters, for cleave and for symbols, and never mark anything. Angle brackets appear in
  no in-scope Oracle text, and a Hole is written `<name>`.

As an illustration of the shape only, not of the syntax:

```
forEach             "<per> <^VerbPhrase for each <p>>"
putOntoBattlefield  "put <subject> <?from: from <from>> onto the battlefield <riders*>"
when                "when <event> <joins*> <?if: , if <if>>, <action>"
getsBoth            "<subject> get <delta>/<delta> <duration?>"
```

A wording is written the way the grammar will read it, so each `spelling` is checked by parsing
it with the English grammar, its Holes standing as phrases of their sorts' Categories. A
`spelling` no Reading covers fails to load.

## 8. Capacity and cost

What the grammar can describe is known only in part, and what recovery costs is not known at
all, because no spelling crate exists. This section separates the two.

- **One-sided adjunction keeps the English side context-free.** Tree insertion grammar, the
  monolingual formalism of substitution with left and right adjunction and no wrapping, is
  shown by Schabes and Waters (1995) to generate context-free languages and to parse strings in
  cubic time. That is a result about parsing strings with one grammar. It bears on spelling in
  one way only: the English half of every Construction describes trees of a kind the
  context-free English grammar already builds. It says nothing about relating a forest to
  terms.
- **The extensions are outside that result.** An optional part or a list Hole abbreviates a
  finite or regular family of wordings. A Tail links an adjunct to a call some arguments below
  its host, and Factoring relates one phrase to several arms. Neither belongs to tree insertion
  grammar or to any synchronous form of it, and no published bound is claimed for them here.
- **Recovery's cost is a design expectation.** Recovery never parses text, so its work is
  matching Constructions against the nodes of a forest the parser has already built. The
  expectation is that each match is local: a Construction looks at one node and its Holes'
  nodes, a Tail at one enclosing phrase, Factoring at one coordination. Whether that keeps
  recovery proportional to the forest is for the spelling crate to measure on the corpus, and
  a Construction that makes recovery enumerate trees instead of reading the forest is a defect
  ([English](english.md) §4).

## 9. Rejected alternatives

- **Substitution alone**, a synchronous tree-substitution grammar. It cannot put an auxiliary
  inside its Action's clause, split one list's modifiers around a head, or put a trigger header
  inside the first sentence of an effect it governs (§2), and it needs a Template per verb for
  every detached quantity (§3.2).
- **Wrapping adjunction**, a full or regular-form synchronous tree-adjoining grammar. Nothing
  in-scope needs the added capacity, and it lacks Factoring like every other variant (§2).
- **Fixing the term instead of the grammar.** Surface faithfulness ([Semantics](semantics.md)
  §1, item 1) argues for moving each mismatch into the term: an optional `per` parameter on
  each of about thirty verbs in place of a Tail, explicit premodifier and postmodifier slots in
  place of adjunction on a nominal, and a trigger attached at the level of the Ability in the
  English grammar. That comes near substitution with optional parts and lists. It is rejected
  because scaling an amount from outside its Action needs a primitive that reaches into the
  Action to change its amount, which is a second place for one value; because it changes the
  English grammar for the sake of the layer above it; and because it still leaves passives
  under auxiliaries and shared-material coordination unsolved.

## 10. Open

**Where a shared-material coordination is expressed.** §3.3 gives the formalism Factoring. It
does not say which coordinations use it. The candidates:

- *Factor in spelling.* The term keeps whole arms, each with the shared part written
  identically, and the Construction for the coordination prints it once. It needs no change to
  the event gate or to any Action, and it covers a subject in two roles ("blocks or becomes
  blocked"). It costs the one extension that no grammar formalism supplies ready-made, and a
  term that writes once per arm a value the text prints once, against [`.syl`](syl.md) §2.
- *Coordinate the term where English does.* A coordinated Action stands under one Subject or
  one `does`, as two Actions already stand under one Handoff, and a list Hole spells it with no
  Factoring. It writes the shared part once. It costs a change to the event gate, which refuses
  an `or` over Actions today ([Semantics](semantics.md) §4.6), and it does not reach a subject
  in two roles, which has no single Subject to stand under, so that case needs Factoring or its
  own Templates anyway.

The two can coexist, each for the coordinations it fits; what is open is which is the rule.
Until it is settled, an alternative event is written with whole-event arms and the shared
subject identical in each, and no Template is minted for a particular pair of verbs.

**Where the notation is designed.** The notation for optional parts and list Holes has a
planned ticket. The notation for Tails, and for Factoring if spelling keeps it, has none.
