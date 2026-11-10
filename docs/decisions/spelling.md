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
parameter several times, as `bushido` raises both power and toughness by its one count, and
may rearrange its parameters freely. If the pairing reached into the body, every such body would
be a place where one printed phrase stands for two parts of the meaning. Pairing with the call
keeps one printed phrase for one argument.

**A wording is grammatical structure, not a string of words.** It is the fragment of a Reading
that its `spelling` parses to (§7): Lexemes, Categories and Selected Frames, with Holes among
them. What the English layer derives from that structure is never written into a wording, and
none of it is an operation of this grammar:

- **Inflection and agreement.** A verb whose agreement depends on a Hole is written
  uninflected: one wording is "gets" in Giant Growth and "get" in Glorious Anthem.
- **Case.** `controllerOf`, "$subject's controller", over `it` is "its controller".
- **Voice and verb form.** An Action's wording is its verb's one Subcategorization Frame. The
  context that embeds the Action selects the form the frame takes, and its Subject selects the
  voice ([Semantics](semantics.md) §4.2): under `any` or `by(np)` the wording is the frame's
  passive, with the patient's Hole as the clause's subject.
- **Gaps.** `which` fills a Hole and prints no word. The Reading has a Gap there, as in
  Alabaster Leech's "White spells you cast", whose lookback is
  `did(actor(you), cast(which), thisObject)`.

Why: the English layer derives the passive, the gapped forms and the inflections of every
frame from the frame's one declaration ([English](english.md) §3). A wording for each voice,
number or Gap would multiply as frames times forms, and the copies would drift apart
([Architecture](architecture.md), design principle 4).

## 2. The two operations

**Substitution** fills a Hole with the wording of the argument's own Construction. It serves
every case where a phrase stands whole where its parent's wording leaves room for it: "draw
$amount cards", "target $subject", "$cost: $action". A wording whose fixed words stand on
both sides of its Holes is still substitution: "more $p than $subject" is one fragment with
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
gets one closed extension. They are the whole list of this grammar's operations: a wording
that needs something else is a defect in this design, reported and settled here, never patched
in one declaration. Inflection, voice and Gaps are not on the list because they are not this
grammar's (§1).

### 3.1 Optional and list Holes

**An optional part** is printed only when its parameter differs from the parameter's declared
default. It may carry fixed words beside its Hole: Elvish Piper's "put a creature card from
your hand onto the battlefield" prints "from $from", and a card that names no origin prints
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
"target creature gets +1/+1 until end of turn for each creature in your party", what is
scaled is "+1/+1", and one `forEach` holds it and the predicate. The "+1/+1" is printed where
the verb's object stands, and "for each $p" is printed at the end of the verb phrase, past the
duration. "Equal to" does the same in Dogpile's "deals damage to any target equal to
the number of attacking creatures you control", where the recipient stands between "damage" and
what measures it.

Three properties make the extension closed:

- **On the English side a Tail is ordinary one-sided adjunction.** The Reading has "for each …"
  as an adjunct of the verb phrase, which is where the English grammar already puts it. Only
  the link is longer: the adjunct's meaning belongs to a call several arguments down from the
  call that spells the phrase.
- **A Tail states where it stands among its phrase's other adjuncts**: inside them, next to
  what the host itself prints, or outside them. Aurochs prints "gets +1/+0 until end of turn
  for each other attacking Aurochs" and Ral's Staticaster "gets +1/+0 for each card in your
  hand until end of turn". Both Tails attach to one verb phrase, so the Category alone does not
  tell them apart, and the two placements are two Templates (§4). The notation's form for the
  placement is pending (§10).
- **A Tail is read where it is printed.** Gaze of Adamaro "deals damage to target player equal
  to the number of cards in that player's hand": the amount is the declaration's first
  parameter, and its Tail mentions "that player" after the recipient introduced them. Read
  order is printed order ([`.syl`](syl.md) §5), so the walk that reads a term reads a Tail at
  its printed position, not at its Hole's.

Where the text prints a quantity both in place and as a Tail, the two are two wordings, and
each is its own Template (§4). Essence Backlash prints "deals damage equal to that spell's
power to its controller" with nothing detached.

**"For each" is declared at the sort of what it scales.** At `Amount` it scales a number the
text prints: "a card for each …", "3 life for each …". Two things are scaled whole, with no
number of their own to stand beside:

- **A power/toughness change.** The change is one value of its own sort, which the "gets"
  wordings take whole, and `forEach` is declared at that sort. Johtull Wurm's "gets -2/-1 until
  end of turn for each creature blocking it beyond the first" is one change scaled once, and
  Aurochs's "+1/+0 … for each" has one reading, where a `forEach` inside one of two separate
  deltas would also recover as a flat 1 beside a scaled 0. Inside the change, a bare "+N/+N" is
  two printed values and is written as two.
- **Mana printed with no count.** Gaea's Cradle prints "Add {G} for each creature you control",
  with no "one" before the symbol, so the wording is a Template at the sort of the produced
  mana that takes the mana whole. A term that counted the mana with a `forEach` amount would
  store a 1 the card does not print.

This is a known weak point. Each sort scaled whole needs its own "for each" Template, and
there may be more of them than these two; if they multiply, the remedy belongs here, not in
one more Template.

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
place in the term. Neither relates a phrase printed once to several arms.

**A shared phrase is written once in the term**, as any value the text prints once is
([`.syl`](syl.md) §2). It is never copied into each arm, and no arm re-mentions it with an
anaphor the card does not print. Why: a copy of a phrase that introduces a Referent introduces
again. Predator's Strike has one target, and a term with `target(…)` in each arm has two.
Powerstone Minefield prints "Whenever a creature attacks or blocks, this enchantment deals 2
damage to it", and a term with the indefinite in each arm introduces a Referent per arm for
the one creature that "it" names. An unprinted anaphor is a word in the term that the text does
not have, and spelling would have to know to print nothing for it. The term writes the phrase
once in one of two ways:

- **Where the phrase has the same role in every arm, the term coordinates where English
  does.** The coordinated Actions stand under one Subject: one Handoff over the Actions of
  Predator's Strike, and one `does` over an `or` of the Actions of "attacks or blocks". A list
  Hole spells the coordination (§3.1), and no extension is involved. A triggered ability may
  have more than one trigger condition [CR#603.1b], and an `or` of event-capable Actions under
  one Subject is how the term says so when the conditions share their Subject.
- **Where the roles differ, the coordination binds the phrase once.** Its Construction takes
  the shared phrase as a parameter, and each arm holds a placeholder word where the phrase
  belongs, as a lookback holds `which` (§1). The parameter is the binding, read once
  ([`.syl`](syl.md) §4). Chub Toad's "blocks or becomes blocked" is this case, with the
  performer of one event and the patient of the other; so are Aether Tunnel's "gets +1/+0 and
  can't be blocked" and Cephalid Illusionist's "dealt to and dealt by target creature you
  control".

**Factoring** is the third extension, and it serves the second way only: the coordination's
Construction prints the shared phrase once, where the first arm's placeholder stands, and the
placeholders of the other arms print nothing. Recovery reads the one printed phrase as the
parameter.

Where the text repeats the phrase, the term does too. Ancient Craving prints "You draw three
cards and you lose 3 life" and Ambition's Cost "You draw three cards and lose 3 life", and
those are two terms: two Handoffs in sequence, and one Handoff over two Actions in sequence.

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

One wording recovers one written form. Where `.syl` admits two written forms of one term, the
bare word a coercion wraps and the written call of the primitive it wraps through, recovery
yields the bare form, which is the corpus's ([`.syl`](syl.md) §2). Why: two candidates that
differ only in how an author wrote the same term differ in nothing the text chose.

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
arguments, lists and records. Adjunction, Tails and Factoring are relations between that tree
and a Reading. None of them adds a kind of node, and `.syl` gains no syntax outside the
`spelling` string. Only a declaration carries a `spelling`, so every part of a card's `text`
that prints words is a call. A record written in place has no wording of its own: where its
fields are printed, a Template takes them as parameters and spells them: `creatureToken` is
the Template for Archfiend's Vessel's "a 5/5 black Demon creature token with flying".

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
- A Tail has one position and one placement for a given Template, so two printed positions are
  two Templates.
- A shared phrase is written once and printed once, and a phrase the text repeats is written
  again, so the two forms are two terms (§3.3).

Punctuation is not spelling's concern. A Reading stores none, and realization derives it from
structure ([English](english.md) §7).

The square brackets of a cleave line [CR#702.148a] are different. No Reading holds them
either, and no `spelling` contains one, but nothing derives them: "Destroy target [attacking]
creature" and the same line without brackets have one Uncleaved Text and one Reading. The
bracketed spans are kept beside the Uncleaved Text, outside the Reading, so a cleave card
round-trips only if its term says which of its parts are bracketed and spelling relates those
parts to the spans. How the term says it is `.syl`'s to settle with cleave, which has no
representation yet.

## 7. The `spelling` notation

A `spelling` is fixed words with three kinds of markup, each with one job. `$` marks a Hole,
parentheses an optional part, and angle brackets a Tail.

```
spelling := part*
part     := text | hole | optional | tail
hole     := "$" name                    -- a parameter's argument
          | "$<" name ">"               -- the same, where a word character follows
          | "$" name "<" join ">"       -- a List parameter
join     := "" | "," | "and" | "or" | "and/or" | "then" | "$" name
optional := "(" part+ ")"
tail     := "<" Category ":" part+ ">"
text     := any characters except "$", "(", ")", "<" and ">"
```

**A Hole is `$name`**, the sigil that reads the same parameter in `body`
([`.syl`](syl.md) §2): "draw $amount cards", "$subject's controller", "non$p". A Hole printed
again is written again, as in "$subject get $delta/$delta". Where a letter, digit or underscore
follows the name, the name is delimited: "$<quality>walk", "$<n>th from the top of $loc". The
formatter writes the undelimited form everywhere else.

**A list Hole is `$name<join>`.** A `List` parameter must state its join and no other
parameter may, so a list is visible at its Hole. The joins are a closed set: `<>` juxtaposes
the elements; `<,>` separates them with commas and no coordinator; `<and>`, `<or>`, `<and/or>`
and `<then>` coordinate them; and `<$name>` coordinates them with the coordinator a parameter
of the declaration supplies. The commas of a coordination of three or more are the English
layer's and are never written. A `<` directly after a Hole's name always opens a join, so a
Tail that follows a Hole stands after a space.

**An optional part is `( … )`.** It is printed exactly when the one defaulted parameter whose
Hole stands directly in it differs from its declared default: "put $subject (from $from) onto
the battlefield $riders<>", "$subject get $delta/$delta ($duration)". An optional part may
hold another, as in "( … $a ( … $b ))", and each has its own parameter. A `spelling` fails to
load when an optional part directly holds no Hole, the Hole of a parameter with no default, or
the Holes of two parameters. Why: with exactly one governing parameter, the argument decides
whether the part is printed and the printed part decides the argument, which is the round trip
of §6 by construction.

**A Tail is `<Category: …>`**, naming the Category of the enclosing phrase it attaches to
(§3.2): "$per <VerbPhrase: for each $p>". The notation says nothing yet of where a Tail stands
among that phrase's other adjuncts (§10).

**No escape exists, because none is needed.** None of `$`, `(`, `)`, `<` and `>` occurs in the
analyzed Oracle text ([Lexicon](lexicon.md) §1). The first, fourth and fifth occur in no
in-scope card's text at all, and every parenthesis is removed with reminder text: a
parenthetical is reminder text even where it reminds the player of something other than a
keyword's rule, as on Mistform Ultimus, and one inside an ability-word label has no rules
meaning. Square brackets and braces are Oracle's own characters, for cleave and for symbols, and
are never markup.

**What the notation does not say is a Template.** It has no part chosen by an enum value and
no omission governed by a value that is not a default. A wording that differs that way is a
different wording, and gets its own Template (§4).

The declarations under `plugins/` still write a Hole `<name>`: 774 Holes on 592 `spelling`
lines, four of which need the delimited form. The `syl-spelling-conventions` ticket migrates
them.

A wording is written the way the grammar will read it, so each `spelling` is checked by parsing
it with the English grammar, each Hole standing for a phrase of any Category that its sort's
Constructions spell, with the features agreement will fix left unmarked. A sort is not one
Category: a `Predicate` is a noun in "creature", an adjective in "tapped", a preposition phrase
in "with flying" and a relative clause in "you control". A `spelling` no Reading covers fails
to load.

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
- **Copying a shared phrase into each arm**, or re-mentioning it there with an anaphor the card
  does not print. The first duplicates targets and Referents, and the second puts a word in the
  term that the text does not have (§3.3).
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

The notation of §7 has no form yet for two things this document requires: a Tail's placement
among its phrase's other adjuncts (§3.2), and the placeholder a coordination's arms hold
(§3.3).
