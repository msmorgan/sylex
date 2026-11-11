# Spelling

The spelling layer relates Readings to resolved terms ([Architecture](architecture.md), "The
layers"). Recovery reads the Packed Forest of a card's text and assembles the resolved terms
whose wording matches it; spelling a resolved term yields a Reading for the English layer to
realize. One grammar serves both directions, and this document says what kind of grammar it is.
Its data is the `spelling` field of each `.syl` declaration ([`.syl`](syl.md) §2). The terms
Construction, Hole, Tail and Factoring are the
[glossary's](../contexts/oracle-english/CONTEXT.md).

The grammar is a synchronous grammar of substitution, with two closed rules (§2) and three
closed extensions (§3). It has no adjunction. The resolved term is its derivation (§4), and it
changes nothing in the English grammar, in the shape of the resolved term or in what expansion
means (§5).

## 1. Constructions

A Construction is one declaration's wording paired with the call it spells. Every declaration
that carries a `spelling` is one: a Template, a Definition, and a Primitive too, since a
primitive is where a constructor's spelling attaches ([`.syl`](syl.md) §1). The wording is one
fixed fragment of English structure, a tree whose frontier carries Holes, one for each parameter
the wording prints. The call is one node of the resolved term, with those parameters as its
arguments.

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
  uninflected: the one verb-phrase wording of a power/toughness change is "gets +3/+3" in
  Giant Growth and "get +1/+1" in Glorious Anthem, once the subject is taken out of it (§2).
- **Case.** `controllerOf`, "$subject's controller", over `it` is "its controller".
- **Voice and verb form.** An Action's wording selects one of its verb's Subcategorization
  Frames. The context that embeds the Action selects the form that frame takes, and its Subject
  selects the voice ([Semantics](semantics.md) §4.2): under `any` or `by(np)` the wording is
  the frame's passive, with the complement the passive promotes as the clause's subject (§2,
  passive subject promotion).
- **Gaps.** `which` fills a Hole and prints no word. The Reading has a Gap there, as in
  Alabaster Leech's "White spells you cast", whose lookback is
  `did(actor(you), cast(which), thisObject)`.

Why: the English layer derives the passive, the gapped forms and the inflections of every
frame from the frame's one declaration ([English](english.md) §3). A wording for each voice,
number or Gap would multiply as frames times forms, and the copies would drift apart
([Architecture](architecture.md), design principle 4).

## 2. Substitution and its two closed rules

**Substitution** is the grammar's one operation. It fills a Hole with the wording of the
argument's own Construction, whose fragment stands where the Hole stood: "draw $amount cards",
"target $subject", "$cost: $action". A wording whose fixed words stand on both sides of its
Holes is still substitution: "more $p than $subject" is one fragment with two Holes. Nothing is
inserted into a phrase another Construction spelled, and nothing is attached at its edge, except
by the extensions of §3 and the two rules below.

Why substitution only:

- **Each Construction is one fixed tree.** Recovery matches a Construction against one forest
  node and the nodes of its Holes. It never asks, at each node on the edge of a phrase, whether
  some other Construction attached words there.
- **A sentence has fewer derivations.** Outside the extensions, the two rules and the nodes the
  English layer derives (the "be" of a passive, an inflection), every node of a Reading belongs
  to the fragment of exactly one Construction. So two derivations of one Reading differ in which
  Constructions they use, never in where they attached the same ones.
- **The term is the derivation with nothing beside it** (§4). Under substitution each call
  spells one subtree of the Reading, so where a call stands in the term is where its wording
  stands in the Reading.
- **`.syl` sorts line up with English constituents.** A Hole is one constituent, or a run of
  sentences (below), so a parameter's sort is the sort of one phrase of the Reading. That is what
  lets the sort of a slot decide a word's coercion ([`.syl`](syl.md) §3): "creature" is a noun
  phrase's head at one sort and a modifier at another, and the slot it fills says which.

Substitution is in no lower complexity class than one-sided adjunction, which is context-free
and cubic too (§8). What it buys is the shape of what recovery searches and of what the term
records.

**A Hole may hold a run of sentences.** A run is one or more consecutive sentences of a
sentence sequence. A text has no syntactic structure above the sentence: the English grammar
builds a sequence by right recursion (`SentenceSequence`, `crates/english/grammar/english.fug`),
which is one grouping chosen for the forest, so spelling reads a sentence sequence as flat, and a
run fills a Hole whether or not it is a node of the Reading. A run fills only a Hole its wording
marks for one, `$action*` (§7); an unmarked Hole holds one phrase, whatever its sort. `when`'s
effect Hole is marked, and so is `replaceEvent`'s `replacement` Hole, which holds `instead` over a
run (below). A Construction's fixed words may follow a marked Hole as a sentence of their own,
which is plain substitution. Aggravated Assault prints "{3}{R}{R}: Untap all creatures you
control. After this main phase, there is an additional combat phase followed by an additional
main phase. Activate only as a sorcery." `activatedOnlyDuring`'s wording "$cost: $action*.
Activate only $timing." holds the run of the first two sentences in its `action` Hole,
a run the Reading has no node for: its `SentenceSequence` nodes span all three sentences and the
last two. Academy Wall's "Whenever you cast an instant or sorcery spell, you may draw a card. If
you do, discard a card. This ability triggers only once each turn." is the same shape, with the
last sentence the wording of `whenever`'s usage limit after the run in its effect Hole.

Where English attaches a word inside or beside a phrase, the term has the structure English
has, or a Template holds the word in its one fixed fragment:

- **An auxiliary or negation** stands between a Subject and an Action that its context takes
  apart ([Semantics](semantics.md) §4.1): "$subj can't $action", with the Action's verb phrase
  in one Hole. Only the passive needs more (passive subject promotion, below).
- **A central adverb** ("also", "still", "no longer") attaches to the verb phrase below the
  subject (`CentralAdverbial`, `crates/english/grammar/english.fug`): *Efflorescence* prints
  "that creature also gains trample and indestructible". Its Construction takes the verb
  phrase, with no subject, as its Hole, and the Construction that supplies the subject takes the
  result. A declaration that takes its subject together with its verb phrase leaves no such
  Hole; the continuous-effect declarations under `plugins/` (`getsPt`, `abilityGrant`, `gain`,
  `get` and others) do, and the `syl-continuous-effect-subject` ticket takes the subject out.
- **A floated "each"** is a sibling of the subject and the verb phrase in one flat clause
  (`FloatedEach`): *Abandon Reason* prints "Up to two target creatures each get +1/+0 and gain
  first strike until end of turn". Its Construction is that clause, "$subject each $action",
  with the subject and the subjectless verb phrase as two Holes. Only after an auxiliary
  (`AuxiliaryFloatedEach`) does "each" stand over a verb phrase of its own.
- **Modifiers on both sides of a noun** come from two lists. A description is a nominal: a head
  with the premodifiers printed before it and the postmodifiers printed after it,
  `nominal(pre = [], head, post = [])`, each list in its own list Hole (§3.1) on its own side of
  the head's Hole. In "other nonland permanents you control" (*Abdel Adrian, Gorion's Ward*),
  "other" and "nonland" are premodifiers and "you control" a postmodifier. The `nominal`
  declaration is not yet built: the descriptions under `plugins/` are one flat `and([…])` of
  predicates, the head among its modifiers, which has no wording under substitution, and the
  `syl-nominal-descriptions` ticket restructures them.
- **A word printed in several places is a Template per placement.** "Also" stands between a
  subject and its verb ("it also deals 4 damage", *Goblin Barrage*), after a copula or passive
  auxiliary ("Burakos is also a Cleric, Rogue, Warrior, and Wizard", *Burakos, Party Leader*;
  "that creature is also goaded", *Hot Pursuit*), after a modal ("may also copy that spell",
  *Parnesse, the Subtle Brush*), at the front of a clause with no subject after a comma ("also
  create X 4/4 white Angel creature tokens", *Finale of Glory*), and at the front of a sentence
  (*Beast Mode*). "Still" stands after a copula too ("a 3/3 creature that's still a land",
  *Animate Land*). "Instead" has at least five placements: at the end of the verb phrase it marks
  (706 cards, and about 50 more where another Construction's trailing adjunct follows, which is
  plain substitution: Accumulate Wisdom's "Put each of those cards into your hand instead if there
  are three or more Lesson cards in your graveyard" has a Reading [[Put … instead] if …],
  `instead`'s fragment inside the conditional's), before its clause after a fronted dependent and
  its comma (*Notion Thief*'s "instead that player skips that draw"; 146), after a modal
  (*Archmage Ascension*'s "you may instead search your library"; 10), between a subject and its
  finite verb ("that player instead puts that land onto the battlefield", *Land Equilibrium*;
  *False Dawn*), and inside a passive verb phrase ("can't be prevented or dealt instead to another
  permanent or player", *Lava Burst*, *Whippoorwill*).

Two mismatches between English and the term are not covered by substitution or the extensions
of §3. Each gets one closed rule.

**Passive subject promotion.** The contexts whose first parameter is a Subject take a Subject
and an Action apart: a trigger (`does`), a replacement (`would`), a lookback (`did`), the
deontic rules (`cant`, `must`, `can`, `gatedBy`) and the negative declarative (`doesnt`)
([Semantics](semantics.md) §4.1). An imperative, a cost and a Handoff have no Subject. Under the
Subject `any` or `by(np)`, the context's Subject position prints the complement of the Action's
frame that the English passive promotes, and the Action's verb phrase is that frame's passive
(§1), with the `np` of `by(np)` as its agent phrase. The English layer derives three such
passives: of a monotransitive frame, whose Object is promoted ("This spell can't be
countered"); of a ditransitive one, whose indirect Object is promoted ("equipped creature would
be dealt damage", the frame `deal` declares for it); and the prepositional passive of a
prepositional frame, whose preposition's object is promoted and the preposition stranded ("each
card looked at"). Which parameter of the Action fills that complement is its frame's. Abrupt
Decay's
"This spell can't be countered" is `cant(any, counter(thisSpell))`. The patient is an argument
of `counter` ([Semantics](semantics.md) §4.4), yet it is printed before "can't be", where
`cant`'s wording has its Subject, and "countered" in the verb-phrase Hole. No card under
`plugins/` writes `doesnt` with `any` or `by(np)`. Where one verb has two frames with two
promoted complements, as `deal` has, one term has two passives, and which one a card prints is
open (§10).

The rule applies only where the Subject is `any` or `by(np)`. `becomeBlocked` ("becomes
blocked") and `resolve` ("resolves") are subjectless Actions read like `die`: what becomes
blocked or resolves is the context's Subject, active, as in `does(actor(thisCreature),
becomeBlocked)`, so the rule has nothing to promote. That form is not yet built: the
`syl-shared-phrase-written-once` ticket builds it, and the corpus writes
`does(any, becomeBlocked(x))` (afflict, rampage, bushido) and `does(any, resolve(thisSpell))`
(buyback, haunt, rebound) today. "Becomes blocked by $blocker" (73 cards) is no passive of
`block`: the passive of `block`'s frame is "is blocked by"
(*General Jarkeld*'s "all creatures that the other is blocked by"), so "becomes blocked by" is
a Template whose one fragment prints it over `by(B)` and `block`.

Why: voice is already the Subject's choice ([Semantics](semantics.md) §4.2), so the rule links
two Holes the term already holds in one fixed way, and it adds no place for words: the
promoted phrase takes the subject position the context's own wording has.

**The run rule: a dependent printed in a run's first sentence may govern the whole run.** The
Construction whose wording prints the dependent takes the run in its Hole, and the dependent
stands where that wording places it in the run's first sentence, before the sentence's clause or
after its verb phrase. The run's other sentences follow the first as they are printed. The rule
applies only to a Hole marked to hold a run (above), so a dependent whose Construction has no
such Hole never governs one: "until end of turn" is a `Duration` argument of its effect's own call,
and it stays an adjunct of its one sentence.

- **A fronted trigger clause.** A triggered ability is written "[When/Whenever/At] [trigger
  condition or event], [effect]" [CR#603.1], and one with an intervening "if" clause
  "When/Whenever/At [trigger event], if [condition], [effect]" [CR#603.4]. Boommobile prints
  "When this Vehicle enters, add four mana of any one color. Spend this mana only to activate
  abilities." In the Reading, "When this Vehicle enters," is a front adjunct of the first
  sentence's clause, and `when(event, action)` takes the run of both sentences as its `action`.
  With an intervening "if", the Reading stacks both clauses as front dependents of the first
  sentence: Eon Frolicker's "When this creature enters, if you cast it, target opponent takes an
  extra turn after this one. Until your next turn, you and planeswalkers you control gain
  protection from that player." has "When this creature enters," over "if you cast it, target
  opponent takes …", and the second sentence outside both. The same holds for a delayed trigger
  ("At the beginning of the next end step, …") and a reflexive one ("When you do, …").
- **A fronted replacement with "instead".** Eruth, Tormented Prophet prints "If you would draw a
  card, exile the top two cards of your library instead. You may play those cards this turn."
  The replaced event never happens, and a modified event occurs instead [CR#614.6]; the second
  sentence reads the cards the first exiled, so the replacement is the run of both sentences. Two
  Constructions each print a dependent in the run's first sentence. `instead` prints its word
  after the first sentence's verb phrase, over its `action` Hole, which holds the run.
  `replaceEvent` prints its would-clause "If you would draw a card," at the front of the first
  sentence, over its `replacement` Hole, which holds `instead` over the run. Enduring Renewal and
  Chains of Mephistopheles have the same shape.
- **A postposed delayed trigger clause.** Goblin Kites prints "{R}: Target creature you control
  with toughness 2 or less gains flying until end of turn. Flip a coin at the beginning of the
  next end step. If you lose the flip, sacrifice that creature." A delayed triggered ability does
  something at a later time, and its trigger word won't usually begin it [CR#603.7]. The coin is
  flipped when the delayed trigger resolves, and the sacrifice depends on that flip, so the
  delayed trigger's effect is the run of the second and third sentences. "at the beginning of the
  next end step" is an adjunct after the verb phrase "Flip a coin", in the run's first sentence,
  which is the second sentence of the ability's effect: the run a dependent governs starts at the
  sentence that prints it, wherever that sentence stands in the sequence. The Template that prints
  a delayed trigger's point after its action is not yet declared (`atNext`'s `TODO(spelling)`).

The rule does not choose a run's length. Recovery yields a candidate for each run the Hole can
hold that starts at the dependent's sentence, and a candidate the checker or the discourse
reading refuses drops out (§5). 30 of the 371 lines that print a delayed trigger clause after its
effect print a further sentence after it, and other fronted dependents are followed by more
sentences too: "As long as …," on 16 lines, "Until end of turn," on 33 and "If <condition>," on
524. Where the dependent's Construction has a Hole that holds a run, each such line has a
candidate per run, and the term may follow English where a branch's output reaches later text
([Semantics](semantics.md) §2.2).

A sentence printed after a run that it governs is not in the run's first sentence, and the run
rule does not cover it. It needs no rule: it is its Construction's own wording after a Hole that
holds the run, which is substitution (above). "Activate only as a sorcery." and "This ability
triggers only once each turn." are such sentences, and so is a rounding instruction. Saw in Half
prints "Destroy target creature. If that creature dies this way, its controller creates two tokens
that are copies of that creature, except their power is half that creature's power and their
toughness is half that creature's toughness. Round up each time." The rounding is a Construction
over the Action it covers, whose wording is that Action followed by "Round $mode each time." and
whose Action Hole holds the run of the first two sentences, with every fraction inside it
unrounded (`syl-round-each-time`; the card writes `half(up, …)` at each fraction today). Its own
phrase is a run too, so a Hole may hold it: Hydroid Krasis's "When you cast this spell, you gain
half X life and draw half X cards. Round down each time." is `when` whose effect Hole holds the
rounding Construction's run of two sentences, and "When you cast this spell," governs that run by
the run rule.

Why: the term cannot follow English here, because the whole run belongs to the one context, and
writing only the first sentence under it would leave the rest outside. One rule covers every
context whose Hole holds a run, so a new context with an effect of several sentences needs no
rule of its own. Nor does the English grammar change for the layer above it (§5, §9). The cost
is in recovery: it matches Holes against runs, spans of sibling sentences, and not only against
nodes, and an ability of n sentences has n(n+1)/2 runs, on the order of n²/2 (§8).

So substitution, the extensions of §3 and these two rules are the operations this design
states; what they do not cover is listed in §10, and a wording that needs something else is a
defect in this design, reported and settled here.

## 3. The three extensions

Substitution and its two closed rules leave three things Oracle text prints unexpressed. Each
gets one closed extension. Substitution, these three and the two rules of §2 are the operations
this design states, and §10 lists what they do not yet cover. A wording that needs something
else is a defect in this design, reported and settled here, never patched in one declaration.
Inflection, voice and Gaps are not on the list because they are not this grammar's (§1).

### 3.1 Optional and list Holes

**An optional part** is printed only when the parameters that govern it are supplied, and
omitted when they are absent (§7). It may carry fixed words beside its Hole: Elvish Piper's "put
a creature card from your hand onto the battlefield" prints "from $from", and a card that names
no origin prints nothing there.

**A list Hole** prints each element of a `List` argument and states how the elements join:
coordinated, with the commas and the coordinator English uses (Angelic Skirmisher's "first
strike, vigilance, or lifelink"), or juxtaposed, as riders and modifiers are. An empty list has
no wording: its Hole stands in an optional part, and the part is omitted.

Why: without them a declaration needs a wording for every combination of its optional
parameters, and `when` alone has four optional parameters and a list. Wordings that differ only
in such a part become families of near-duplicate Templates, and 128 of the 167
`TODO(spelling)` markers in `plugins/builtin/` stand for want of exactly this.

**Printed order governs the order of a list's elements**, in every list, the conjuncts of
`and([…])` included. The elements are written in the order the text prints them. A nominal's
modifiers stand in two lists (§2; the `nominal` declaration is not yet built), so which side of
the head a modifier stands on is which list holds it, and each list's order says the rest:
"other nonland permanents you control" has "other" then "nonland" in `pre`, and "you control" in
`post`. Why: order among modifiers is wording. Favorable Winds prints "Creatures you control
with flying" and Blight Pile "creatures with defender you control", so a list in any other
order would lose which was printed. It is also what read order requires ([`.syl`](syl.md) §5),
and a list with one admissible order gives recovery one term for a noun phrase where an
unordered list would give it every permutation.

### 3.2 Tails

A Tail is part of a Construction's wording that is printed away from the rest: at the edge of
a phrase that encloses the Construction's Hole, and the wording names the Category of that
phrase. It is the one place a Construction prints words outside its own fragment. Quantity
adjuncts are the case. In Kabira Outrider's
"target creature gets +1/+1 until end of turn for each creature in your party", what is
scaled is "+1/+1", and one `forEach` holds it and the predicate. The "+1/+1" is printed where
the verb's object stands, and "for each $p" is printed at the end of the verb phrase, past the
duration. "Equal to" does the same in Dogpile's "deals damage to any target equal to
the number of attacking creatures you control", where the recipient stands between "damage" and
what measures it.

Three properties make the extension closed:

- **On the English side a Tail is an ordinary dependent.** The Reading has "for each …" as an
  Adjunct of the verb phrase, which is where the English grammar already puts it. What the Tail
  adds is the link: the Adjunct's meaning belongs to a call several arguments down from the
  call that spells the phrase, and it prints only at the edge of a phrase of the Category it
  names.
- **A Tail states where it stands among its phrase's other adjuncts**: inside them, next to
  what the host itself prints, or outside them. Aurochs prints "gets +1/+0 until end of turn
  for each other attacking Aurochs" and Ral's Staticaster "gets +1/+0 for each card in your
  hand until end of turn". Both Tails attach to one verb phrase, so the Category alone does not
  tell them apart, and the two placements are two Templates (§4). The notation writes the
  inside placement `<~inner Category: …>` (§7); which Templates need it is the
  `syl-tail-placement` ticket's to find.
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
attacks or blocks" has one subject for two events, and so does Chub Toad's "Whenever this creature
blocks or becomes blocked". Aether Tunnel's "Enchanted creature gets +1/+0 and can't be blocked"
has one subject that stands in two roles, the subject of an active verb phrase in one arm and the
patient a passive promotes in the other ([Semantics](semantics.md) §4.2). Predator's Strike's
"Target creature gets +3/+3 and gains trample until end of turn" has one subject and one duration
over two actions. The English grammar reads each as a coordination whose conjuncts share a
dependent ([English](english.md) §3).

Substitution links one place in the wording to one place in the term, and so would
adjunction of any kind. None of them relates a phrase printed once to several arms.

**A shared phrase is written once in the term**, as any value the text prints once is
([`.syl`](syl.md) §2). It is never copied into each arm, and no arm re-mentions it with an
anaphor the card does not print. Why: a copy of a phrase that introduces a Referent introduces
again. Predator's Strike has one target, and a term with `target(…)` in each arm has two.
Powerstone Minefield prints "Whenever a creature attacks or blocks, this enchantment deals 2
damage to it", and a term with the indefinite in each arm introduces a Referent per arm for
the one creature that "it" names. An unprinted anaphor is a word in the term that the text does
not have, and spelling would have to know to print nothing for it. The term writes the phrase
once in one of two ways:

- **Where the phrase has the same role in every arm, the term coordinates where English does.**
  The coordinated phrases stand under one subject: Predator's Strike's two continuous-effect verb
  phrases under the one Construction that supplies their subject (not yet built, §2), and one
  `does` over an `or` of the Actions of "attacks or blocks" and of Chub Toad's "blocks or becomes
  blocked", `does(actor(thisCreature), or([block, becomeBlocked]))` (not yet built: no `or` stands
  at `Action` under `plugins/builtin/`, `semantics.md` §4.6 refuses one at the event gate,
  `becomeBlocked` still takes its subject, and the `syl-shared-phrase-written-once` ticket settles
  both). The subject of Predator's Strike is no Handoff's, since a Handoff makes its subject the
  Actor and a creature that gets +3/+3 performs nothing. A list Hole spells the coordination
  (§3.1), and no extension is involved. "Attacks or blocks" is one trigger condition, met when a
  game event matches either event [CR#603.2], and an `or` of event-capable Actions under one
  Subject is how the term says so when the events share their Subject. Several trigger conditions
  of one ability, each with its own trigger word, are instead the disjunction at
  `TriggerCondition` [CR#603.1b].
- **Where the roles differ, the coordination binds the phrase once.** Its Construction takes
  the shared phrase as a parameter, and each arm holds a placeholder word where the phrase
  belongs, as a lookback holds `which` (§1). The parameter is the binding, read once
  ([`.syl`](syl.md) §4). Aether Tunnel's "gets +1/+0 and can't be blocked" is this case, and
  so is Cephalid Illusionist's "dealt to and dealt by target creature you control". Dream
  Fighter's "blocks or becomes blocked by a creature" is an open question of the
  `syl-shared-phrase-written-once` ticket.

**Factoring** is the third extension, and it serves the second way only. In the Reading the shared
phrase is a sibling of the coordination, in no arm, and every arm lacks it. A shared subject
stands above a coordination of subjectless verb phrases: Aether Tunnel's "Enchanted creature gets
+1/+0 and can't be blocked" is `SubjectPredicate` over `Coordination:VerbPhrase`. A shared
complement follows a coordination of verb phrases that each have a gap for it: Cephalid
Illusionist's "dealt to and dealt by target creature you control" is
`Coordination:VerbPhrase/shared`, whose head is the coordination of "dealt to" and "dealt by" and
whose shared dependent is the noun phrase. The coordination's Construction prints the shared
phrase once, in that sibling position, and every arm's placeholder prints nothing, standing where
the arm lacks the phrase. Recovery reads the one printed phrase as the parameter.

Where the text repeats the phrase, the term does too. Ancient Craving prints "You draw three
cards and you lose 3 life" and Ambition's Cost "You draw three cards and lose 3 life", and
those are two terms: two Handoffs in sequence, and one Handoff over two Actions in sequence.

## 4. The resolved term is the derivation

The resolved term stays a plain tree of calls, and that tree is the derivation: each call is one
Construction, and where it stands says what it was combined with. Nothing records a derivation
beside the term.

Substitution makes that hold directly: each call's wording is one fixed fragment, so, outside the
extensions, the two rules and the nodes the English layer derives, each call spells one subtree of
the Reading, or one run of its sentences (§2). The extensions and closed rules keep it, because
each relates parts of the Reading to calls the term already has. A Tail is an argument, at some
depth, of the call that spells its phrase, as the `forEach` inside a verb's amount is. Factoring
prints a parameter of the coordination's own call (§3.3). Passive subject promotion links the
complement the passive of an Action's frame promotes to the Subject position of the context that
takes the Action, and the run rule links a dependent printed in a run's first sentence to the
whole run; in both, the context is a call that takes the Action as an argument. None needs a node
the term would not otherwise have.

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
yields the bare form, which is the corpus's ([`.syl`](syl.md) §2). Likewise, when a Template's
expansion is a call that could also be written directly, the written form is the Template: a
one-condition trigger is the Ability `when`, `whenever` or `at`, and `triggered` is written only
over a disjunction of trigger conditions. Why: two candidates that differ only in how an author
wrote the same term differ in nothing the text chose.

Why: a derivation kept apart from the term would be something one direction of translation
invents and the other discards, which the canonical form does not store
([Architecture](architecture.md), design principle 2).

## 5. What spelling leaves alone

**The English grammar.** Its Productions, its Readings and its Packed Forest do not change.
A Reading is a tree, so it already has every Adjunct and Modifier on one side of the host
whose Category licenses it, and a shared dependent beside its coordination
([English](english.md) §3). Recovery reads the Packed Forest and never parses text again
([English](english.md) §4): a Construction's wording is matched against forest nodes, and a Hole
that holds a run against a span of sibling sentences (§2), which reads the grammar's
`SentenceSequence` as flat and adds no node to it. The grammar still names no Catalog item and
carries no meaning ([English](english.md) §1–2).

**The shape of the resolved term.** A term is a tree of calls with named and positional
arguments, lists and records. The two closed rules, Tails and Factoring are relations between
that tree and a Reading. None of them adds a kind of node, and `.syl` gains no syntax outside the
`spelling` string. Only a declaration carries a `spelling`, so every part of a card's `text`
that prints words is a call. A record written in place has no wording of its own: where its
fields are printed, a Template takes them as parameters and spells them: `creatureToken` is
the Template for Archfiend's Vessel's "a 5/5 black Demon creature token with flying". Whether a
created token's description becomes a nominal (§2) instead is the open question of the
`syl-token-records-to-templates` ticket.

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

- An optional part is printed exactly when every parameter that governs it is supplied, and a
  `spelling` whose skeleton is ambiguous fails to load (§7), so the arguments decide the
  wording and the wording the arguments.
- A list is spelled in its written order, which is the printed order.
- A Tail has one position and one placement for a given Template, so two printed positions are
  two Templates.
- An Action prints its frame's passive exactly when the Subject is `any` or `by(np)` (§2). A
  dependent that governs a run is printed where its Construction's wording places it in the run's
  first sentence, and the run's sentences are its argument's, printed in order; a trigger clause
  printed after its effect is a different Template from one printed before it. So neither rule
  leaves a choice open.
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

A `spelling` is fixed words with markup, each mark with one job. `$` marks a Hole, `@` the
printed form of its head word, `*` a Hole a run of sentences may fill, `~` ties a Hole to
letters, parentheses mark an optional part, angle brackets a list's join, and `<~ … >` a Tail.

```
spelling := part*
part     := text | hole | optional | tail
hole     := [affix "~"] "$" name [mark] ["~" affix]   -- a parameter's argument
          | "$" name "<" join ">"                     -- a List parameter
mark     := "@" form | "*"
form     := "Cardinal" | "Arabic" | "Ordinal" | "Roman" | "Counting"
          | "Present" | "Preterite" | "PastParticiple" | "GerundParticiple"
join     := "" | "," | "." | "and" | "or" | "and/or" | "then"
optional := "(" part+ ")"
tail     := "<~" ["inner"] Category ":" part+ ">"
affix    := letter+
text     := any characters except "$", "~", "@", "*", "(", ")", "<" and ">"
```

**A Hole is `$name`**, the sigil that reads the same parameter in `body`
([`.syl`](syl.md) §2): "draw $amount cards", "$subject's controller". A Hole printed again is
written again, and recovery requires the same term at each (§4).

**A tie, `~`, joins a Hole and the letters beside it into one word**: "non~$p",
"$quality~walk". The word is formed by the lexicon's morphology, never by putting the characters
side by side: "non~$p" is "nonland" in Abrupt Decay and "non-Aura" in Absorbing Man. A Hole that
abuts a letter or digit with no tie fails to load, so a missing space is never read as a tie, and
the affix is a bound form the lexicon declares. An ordinal is no tie: "third" is a numeral of the
lexicon's, written `$n@Ordinal` (below).

**A printed form is `$name@Form`.** It states the form of a Hole's head word where neither
agreement nor a governing word fixes it (§1). The forms are the lexicon's (`Numeral` and
`WordForm`, `crates/lexicon/src/types.rs`), and `@` has no other meaning:

- **A numeral's notation**: `Cardinal` ("three"), `Arabic` ("3"), `Ordinal` ("third") and `Roman`
  (a Saga's chapters), and `Counting` (below). The English grammar admits more than one at one
  place, "Draw two cards." and "Draw 2 cards." alike, so the notation is wording, and the slot
  states it: *Approach of the Second Sun* prints one value both ways, "put Approach of the
  Second Sun into its owner's library seventh from the top and you gain 7 life", which is
  `fromTop`'s "$n@Ordinal from the top of $loc" and `gainLife`'s "gain $amount@Arabic life".
- **A verb form on an `Action` Hole that nothing governs**: `Present`, `Preterite`,
  `PastParticiple` and `GerundParticiple`. After "can't" or "would" the verb is governed and
  nothing is written; in a clause whose Subject and Action are two Holes, nothing governs it, so
  `does` and `did` differ in their spelling, the first `@Present` ("this creature enters") and
  the second `@Preterite` ("you attacked this turn"), both in *Mardu Hordechief*'s "When this
  creature enters, if you attacked this turn, create a 1/1 white Warrior creature token."

A label is refused where the position admits only one form. A wording at a numeric sort that
leaves its numeric Hole unlabeled, as `lit`'s "$value" and `plus`'s "$left plus $right" do,
prints it in the notation of the Hole its own call fills: `plus(thatMuch, 1)` is "that many plus
one +1/+1 counters" under `putCounters` in *Hardened Scales* and "that much damage plus 1" under
`dealDamage` in *Sulfuric Vapors*.

**`Counting` is `Cardinal`, except that the integer 1 prints as the article or as no word in
three places.** Oracle counts objects that way, and it is the one alternation chosen by a value
that the notation states (below). The three places:

- 1 as the whole determiner prints the indefinite article, with a singular noun: "Draw a card."
  (*Opt*) beside "Draw two cards." (*Divination*).
- 1 as the whole determiner before "target" prints no word: "Destroy target creature."
  (*Murder*) beside "Destroy two target nonblack creatures." (*Wicked Pact*).
- 1 after another determiner prints no word: "the top card of your library" (*Courser of
  Kruphix*) beside "the top three cards of your library" (*Sensei's Divining Top*).

Elsewhere 1 is "one": in "up to one target creature" (*Witch of the Moors*) the determiner is
"up to one", not 1. The grammar parses "Draw one card." and "Destroy one target creature." too,
so the exceptions are wording, not agreement. A numeral "one" printed where `Counting` would
print the article, as in *Melira, the Living Cure*'s "you get one poison counter" and *Noxious
Vapors*'s "chooses one card of each color", is the parameterless Template `one` at `Amount`,
whose spelling is "one".

**A list Hole is `$name<join>`.** A `List` parameter must state its join and no other parameter
may, so a list is visible at its Hole. The joins are a closed set: `<>` juxtaposes the elements;
`<,>` separates them with commas and no coordinator; `<.>` prints each element as a run of
sentences (`*`, below) after the one before, so such a list needs no `*`; `<and>`, `<or>`,
`<and/or>` and `<then>` coordinate them. No parameter chooses a join: a coordinator is printed by
the Template that names it, as the Templates `and`, `or` and `andOr` print "and", "or" and
"and/or" over the three `coord` primitives (at `NounPhrase`, `Predicate` and `Location`), which
have no `spelling`. Under
`plugins/`, no `coord` has a `spelling`, and the Templates `and`, `or` and `andOr` have none yet
either; the `syl-spelling-optional-segments` ticket writes them. The commas of a coordination of
three or more are the English layer's and are never written. One element prints alone, with no
join. An element whose own wording prints nothing in place, having only a Tail, takes no part in
the join. A `<` directly after a Hole's name is only ever a join, and `<~` only ever opens a Tail,
so a Tail may follow a Hole with or without a space. A `~` anywhere else, outside `<~`, is a tie.

**A Hole a run of sentences may fill is `$name*`** (§2, the run rule). An unmarked Hole holds one
phrase. `when`'s `action` Hole is `$action*`, and holds Boommobile's two sentences. The parts of
a wording that stand in a `*` Hole's own sentence are the dependents the run rule prints in the
run's first sentence: `when`'s "When $event," before it, `instead`'s "instead" after its verb
phrase. A sentence of the wording after the Hole's, as `activatedOnlyDuring`'s "Activate only
$timing.", follows the run's last.

**An optional part is `( … )`.** A parameter is omissible when it has a declared default, or
its sort is `Option(T)` or a list; its absent value is the default, else `none`, else the
empty list. An optional part is governed by every parameter with a Hole directly in it, and
each of those must be omissible. It is printed when every governor differs from its absent
value and omitted when none does: "put $subject (from $from) onto the battlefield
($riders<>)". A term in which only
some governors are supplied has no wording by that declaration. An optional part may hold
another, which can print only when the outer one does. All the Holes of one parameter stand in
one optional part, or all stand outside every one. An empty list is never printed by its Hole;
it is spelled only as the absent value of an optional part.

**A Tail is `<~Category: …>` or `<~inner Category: …>`** (§3.2): "$per <~VerbPhrase: for each $p>".
Its host is the nearest phrase of the named Category above the Hole the declaration's call fills
that the English grammar lets carry the Tail's words as a trailing dependent. For ", except …"
that is a finite Clause and never a relative's Clause, whose Gap the grammar keeps it from
closing: in Shredder, Shadow Master the nearest Clause above the copy is the relative "that's a
copy of Shredder", and the host is the next one up, "create a token … attacking that player". In
Aurochs and Ral's Staticaster the host of "for each …" is the verb phrase "gets +1/+0". Where the
declaration prints nothing in place, its Hole has no node in the Reading, and how its host is
found is open (§10). Unmarked, a trailing Tail stands inside the fronted dependents of that
Category and outside the host's other trailing dependents with the same head: it attaches to the
largest phrase of that Category with the same head that begins with no fronted dependent. That is
the tree the forest builds, where a comma-set post-dependent stands inside a fronted one
(`CompiledGrammar::normal_form`, "fronted dependent outermost",
`crates/syntax/src/normal_form.rs`). Shredder, Shadow Master prints "Whenever Shredder attacks a
player, for each other opponent, create a token that's a copy of Shredder tapped and attacking
that player, except it isn't legendary": the copy's ", except it isn't legendary" is a
post-dependent of the clause "create … attacking that player", inside both "Whenever …," and "for
each other opponent,". Marked `inner`, the Tail prints at the edge of the host itself, before the
host's other trailing dependents. Tails that land at one edge print in the order of their Holes,
and an argument printed at two Holes prints its Tail once. A Tail stands last among the parts
beside it and holds no Tail.

**The notation does not yet express passive subject promotion (§2).** Nothing marks a
context's Subject Hole and Action Hole as the pair the rule links, or which Action's parameter
fills the promoted position. Until it does, a promoted Subject stands outside the check that a
variant is English (below), and so do the sentences of a run after its first: that check
substitutes one phrase for each Hole where the Hole stands, and under passive subject promotion
and in a run's later sentences the printed material stands elsewhere (§10).

**No escape exists, because none is needed.** None of `$`, `~`, `@`, `*`, `(`, `)`, `<` and `>`
occurs in the analyzed Oracle text ([Lexicon](lexicon.md) §1). `$`, `~`, `@`, `*`, `<` and `>`
occur in no in-scope card's text at all, and every parenthesis is removed with reminder text: a
parenthetical is reminder text even where it reminds the player of something other than a
keyword's rule, as on Mistform Ultimus, and one inside an ability-word label has no rules
meaning. Square brackets and braces are Oracle's own characters, for cleave and for symbols, and
are never markup. `+` is Oracle's too ("+1/+1", Will Kenrith's "[+2]"), and is no mark.

**A `spelling` is refused at load** when it breaks a rule above, and also when:

- **A parameter has no Hole and is not omissible.** A parameter with no Hole is fixed at its
  absent value by that `spelling`, and a term that supplies another value has no wording by it.
- **An optional part directly holds no Hole.** Fixed words alone recover no argument.
- **Nothing is certain to print.** Outside every optional part there is text, a Hole or a
  Tail.
- **A variant is not English.** Every variant, which is each consistent choice of optional
  parts with each list Hole at one, two and three elements, must parse under the English
  grammar: its in-place part at a Category its result sort is spelled as, and each Tail as a
  trailing dependent of a host of its named Category, each Hole standing for a phrase of any
  Category that its own sort's Constructions spell, with the features agreement will fix left
  unmarked. A sort is not one Category: a `Predicate` is a noun in "creature", an adjective in
  "tapped", a preposition phrase in "with flying" and a relative clause in "you control". A
  `spelling` whose in-place part parses in several ways names no one fragment, and which of them
  spelling prints is open (§10).
- **Its skeleton is ambiguous.** Replace each Hole by its parameter's sort, counting two sorts
  as one when a coercion or `Option` takes one to the other, each optional part by an optional,
  and each list Hole by one or more of its sort with its join between. The result is a regular
  expression over words and sorts, and no sequence may match it in two ways. Two optional
  parts that each hold only a bare Hole of one sort are the case it catches, since one printed
  phrase would recover as either argument. No `spelling` under `plugins/` is refused by it
  today; `durationBetween`, whose `start` and `end` are both `Option(GameEvent)`, is a
  declaration that needs a fixed word beside each to pass.
- **A lone list would word as its element.** Where the `spelling` outside optional parts is one
  list Hole and an element can stand at the result sort, a list of one words exactly as its
  element, so that declaration spells lists of two or more only.

Why the skeleton rule: it is the round trip of §6 as one check, in place of a rule on each
construct. It is a check on the `spelling` alone. Two sorts whose wordings overlap can still
give recovery two candidates, and meaning chooses between those (§5).

**What the notation does not say is a Template, or a change to the declaration.** Apart from
`Counting`'s 1 (above), it has no part chosen by a value and no omission governed by a value
that is not absent. A wording that differs that way is a different wording, and gets its own
Template (§4), as *Melira, the Living Cure*'s "one" is `one`. Three more
things are known limits:

- **Coordinating whichever optional pieces are present.** Akul the Unrepentant prints "Activate
  only as a sorcery and only once each turn": "Activate" is printed if either restriction is
  there and "and" only if both are, and an optional part is all or none. One list of
  restrictions with an `<and>` join says it; separate optional parameters do not.
- **A word for an absent value.** Angelic Aberration's "4/4 colorless Eldrazi Angel creature
  tokens" prints a word for an empty list of colors.
- **A Tail between two of its host's adjuncts.** Only the two placements exist.

The declarations under `plugins/` still write a Hole `<name>`: 776 Holes on 593 `spelling`
lines, three of which need a tie (`landwalk`, `typecycling` and `non`) and two an ordinal
(`fromTop` and `fromBottom`, "$n@Ordinal from …"). No declaration writes a form label, a `*` or
a `<.>` join yet. The `syl-spelling-conventions` ticket migrates the Holes, and
`syl-spelling-forms-and-runs` writes the labels, marks and joins.

## 8. Capacity and cost

What the grammar can describe is known only in part, and what recovery costs is not known at
all, because no spelling crate exists. This section separates the two.

- **Substitution keeps the English side context-free.** Each Construction's English half is a
  tree of a kind the context-free English grammar already builds, and substitution builds only
  such trees. Tree insertion grammar, substitution with left and right adjunction and no
  wrapping, is shown by Schabes and Waters (1995) to generate context-free languages and to
  parse strings in cubic time, so substitution is in no lower complexity class than one-sided
  adjunction. These are results about parsing strings with one grammar, and they say nothing
  about relating a forest to terms.
- **The extensions and closed rules are outside those results.** An optional part or a list
  Hole abbreviates a finite or regular family of wordings. A Tail links an Adjunct to a call
  some arguments below its phrase, Factoring relates one phrase to several arms, a Hole may
  hold a run of sentences that is no node, and the two rules of §2 link a Hole's phrase to a
  position in another Construction's fragment. None
  belongs to a published synchronous formalism, and no bound is claimed for them here.
- **Recovery's cost is a design expectation.** Recovery never parses text, so its work is matching
  Constructions against the nodes of a forest the parser has already built. The expectation is
  that each match is local: a Construction looks at one node and its Holes' nodes, a Tail at one
  enclosing phrase, Factoring at one coordination, passive subject promotion at one context and
  its Action, the run rule at the first sentence of one run. A Hole that holds a run is the known
  exception to matching against nodes: recovery matches it against spans of sibling sentences, and
  an ability of n sentences has n(n+1)/2 runs, on the order of n²/2 candidate spans. Whether that
  keeps recovery proportional to the forest is for the spelling crate to measure on the corpus,
  and a Construction that makes recovery enumerate trees instead of reading the forest is a defect
  ([English](english.md) §4).

## 9. Rejected alternatives

- **General one-sided adjunction**, a synchronous tree insertion grammar, in which any
  Construction may attach its wording to the left or right of a node on the edge of a phrase
  another Construction spelled. Recovery would then ask at each node on a phrase's edge whether
  something attached there, and the same words attached at different nodes of one edge would be
  different derivations of one sentence. Where an adjunct attaches is not in the term, so either
  the term records it beside its calls or two derivations share one term. A phrase that another
  Construction adjoined into is no longer the phrase of one argument, so a slot's sort no longer
  names one constituent. What it served is covered without it: the passive under a context by
  passive subject promotion, a dependent over several sentences by the run rule,
  modifiers on both sides of a noun by a nominal's two lists, and an adverb by substitution over
  a subjectless verb phrase or a Template per placement (§2).
- **Wrapping adjunction**, a full or regular-form synchronous tree-adjoining grammar, in which an
  adjunct contributes words on both sides of its host at once. Nothing in-scope needs it. The
  constructions that would are cross-serial pairings, and no in-scope card prints
  "respectively" or "the former … the latter". What looks discontinuous in card text is either
  fixed words around Holes, which is substitution, or one phrase printed away from the unit it
  measures, which is a Tail (§3.2). It would add every cost of one-sided adjunction and more,
  and it still would not provide Factoring (§3.3).
- **Substitution with no extensions or closed rules**, a plain synchronous tree-substitution
  grammar. It cannot print a passive under its context or a dependent in the first of several
  sentences it governs (§2), it needs a wording per combination of optional parameters (§3.1)
  and a Template per verb for every detached quantity (§3.2), and it cannot print a shared
  phrase once (§3.3).
- **Copying a shared phrase into each arm**, or re-mentioning it there with an anaphor the card
  does not print. The first duplicates targets and Referents, and the second puts a word in the
  term that the text does not have (§3.3).
- **Fixing every mismatch in the term.** Surface faithfulness ([Semantics](semantics.md) §1, item
  1) moves into the term each mismatch the term can follow: a nominal's premodifier and
  postmodifier lists, and a verb phrase without its subject (§2). Three mismatches stay with the
  grammar: a detached quantity, the passive and a dependent over a run of sentences. An optional
  `per` parameter on each of about thirty verbs, in place of a Tail, is rejected because scaling
  an amount from outside its Action needs a primitive that reaches into the Action to change its
  amount, which is a second place for one value. The passive stays a closed rule, because voice is
  the English layer's to derive from the frame (§1). A dependent attached at the level of the
  Ability or the sentence sequence in the English grammar, in place of the run rule, is rejected
  because it changes the English grammar for the sake of the layer above it.

## 10. Open

The notation of §7 has no form yet for the placeholder a coordination's arms hold (§3.3).

**A Tail whose declaration prints nothing in place.** Its Hole has no node in the Reading, so
"the nearest phrase above the Hole" names nothing, and a rule by position, such as the smallest
phrase that ends where the Hole stands, does not fit a Reading, which is a tree. How such
a Tail's host is found is not settled.

**A list element that prints only a Tail.** Such an element takes no part in its list's join
(§7), so `[A, B, T]`, `[A, T, B]` and `[T, A, B]` print one Reading, and recovery cannot recover
where in the list `T` stands, against printed order (§3.1) and the round trip (§6). A load rule
that fixes its position, such as "an element that prints only a Tail stands last", would settle
it; none is stated.

**A wording with several Readings.** Spelling a resolved term yields a Reading, and a
Construction is one fragment (§1), but a `spelling`'s fixed words can parse in several ways, and
the notation cannot say which. "Exile all cards from target player's graveyard." gets two
Readings: "from target player's graveyard" postmodifies "cards" in one and is an adjunct of the
verb phrase in the other. Hot Pursuit's "that creature is also goaded." gets two: "is also
goaded" is a copula with "also" and the adjective "goaded" (`PostCopularAdverbial`) in one, and
a passive "is" over "also goaded" (`CentralAdverbial` over the passive participle) in the other.
Which Reading `spell` yields for such a wording is not settled.

**The notation for passive subject promotion.** §7 has no mark for a context's Subject and
Action Holes, or for which of the Action's parameters passive subject promotion moves. Until it
has, a promoted Subject stands outside the check that a variant is English.

**Which passive a term prints.** English has two passives of `deal`. Panther Habit prints "If
equipped creature would be dealt damage", the passive of the ditransitive frame `deal` declares
for it, with the recipient promoted: 158 lines print "is", "was" or "be dealt damage", and 2
"would be dealt damage". Angel of Suffering prints "If damage would be dealt to you", the
passive of the frame with a *to* phrase, with the damage promoted: 40 lines. The parser derives
both. One term, `dealDamage` under `any` with an amount and a recipient, stands for both, and
passive subject promotion (§2) promotes one complement per frame, so nothing in `.syl` says
which passive a card prints. Two answers are candidates: a Template per promoted complement,
each fixing its frame, or promotion that names the promoted parameter, written at the
context's Subject.
