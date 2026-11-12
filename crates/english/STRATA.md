# Complexity Strata

A Complexity Stratum is one of eight cumulative levels of syntactic complexity
(`docs/contexts/oracle-english/CONTEXT.md`). `profile` assigns each Corpus Unit
the highest Stratum among the Syntactic Families it contains
([README](README.md#complexity-strata)). This file says what each Stratum is and
what it brought into the grammar.

## Stratum 0

The card frame: Type Lines, Keyword Lines and their parameters, labels, modal
headers and bullets, costed modes, chapter and level headers, table rows,
activated-ability spines, symbol costs and sentence sequences.

Stratum 0 covers Type Lines, complete keyword lists and non-clausal parameters,
symbolic costs and structured headers. `Document` composes lines with explicit
paragraph boundaries.

A `KeywordPhrase` distinguishes supplied parameters from an omitted-parameter
mention. Keyword lists require supplied parameters; noun modifiers and selected
keyword complements can use mentions.

Modal headers take an imperative verb phrase followed by a dash (*Choose one
—*), with ordinary verb frames selecting their objects. Keyword catalogs
distinguish dashed verbal/composite costs from space-separated mana costs;
mandatory dash forms such as suspend retain their catalog punctuation. The
corpus writes a negative loyalty cost with a Unicode minus and without display
brackets; the grammar accepts that source form as well as bracketed symbol
notation.

Costed modes use bulleted costs, the corpus's Spree plus marker, pawprint
quantities, or a bulleted Flavor Word followed by its cost. A Saga chapter
scopes over the modal ability, not separately over its header. Activation
effects select sentences or modal abilities, so nested cost colons and bare
keyword lists are rejected. A symbol-cost-prefixed modal header is a Stratum 0
fragment; a verbal cost raises its profile to the cost’s Stratum. Bound-keyword
premodifiers belong to the Keyword Quality before the suffix and cost.

`ActivationCost` includes loyalty notation; keyword parameters select `Cost`,
which excludes it. Numeric zero in a neutral loyalty symbol remains a numeral
leaf. Mixed costs use one right-recursive structure, preventing duplicate trees
from overlapping symbol and verbal cost alternatives. Modal selection with
adjuncts or relatives belongs to the corresponding later strata.

The profiler recognizes bound stems and preposed qualities without treating
keyword verbs as keyword heads.

## Stratum 1

Core clauses and phrases: imperative and finite active clauses, and the Noun and
Preposition Phrases inside them.

Outer ability spines compose with Stratum 1 imperative and finite active
clauses. Layered nominals preserve noun, adjective and participial premodifiers,
genitives, quantities and frame-selected complements.

The Targeting Marker `det:target` ends a Determiner Phrase: alone, singular
(*target creature*, *target Merfolk*); after an adjective (*other target
creatures*); or after an outer determiner, which gives it its Number (*two
target creatures*, *each other target creature*, *[one or two] target
creatures*), including a maximum Preposition Phrase (*up to one target
creature*). Alone in the plural oblique of a Number-transparent noun it takes
that plural (*any number of target Equipment*). An ordinary Determined Noun
Phrase or possessor then takes it, so every postmodifier attaches inside the
Nominal, below the determiner (*target [creature you control]*). Its homographic
Target Noun is restricted to nominal heads. Preposition frames selecting a bare
nominal take a singular count-noun head and its selected complements, without
arbitrary noun compounds or the plural/mass alternatives already supplied by
Noun Phrases. Numeric arguments use Amount frames; numeric subjects and
predicatives retain Noun Phrase analyses. Quantity nominals use general
determination. Modified quantities admit the indefinite article, and
distributive quantities preserve singular external agreement independently of
the numeral–noun agreement inside the group. A bare slash measure is a Measure
Noun Phrase only where a head's frame selects one (*gets +2/+2*, *gets twice
-X/-X*, *your choice of 5/1 or 1/5*) and a predicative Complement after *be* or
*become* (*it's 4/3*); it is a noun only under a determiner (*an additional
+2/+2*), as a bracketed loyalty symbol is, so neither is a general Object, a
Manner Adjunct or the head of an attributive noun (*Draw a card 3/3.* has no
Reading). Slash measures take nonnegative amounts with optional sign markers, so
a negative numeral does not duplicate the marker-plus-amount analysis; signed
zero and variables retain their notation. Attributive-only adjective position is
independent of determination, preserving bare plural `other types`.

The state adjectives `tapped` and `untapped` coexist with the verbal participles.
The premodifier judgments require both analyses; the depictive `enters tapped`
requires the adjective. Transform also explicitly declares the state adjective
`transformed`; the Garland and Startled Awake judgments require its depictive
Adjunct reading. Short focused Objects may be postposed after a PP.
Temporal/manner Objects are not globally forbidden by their game meaning.
The scalar complement in `base power 0` attaches to `power`, with `base`
modifying that headed nominal. `until counter spells` retains the noun-premodifier
reading of `counter spells`, while rejecting a subjectless finite Clause.

Independent genitives agree with the possessed referent, not the possessor.
Quantity nominals use general determination, including `an additional two
creatures` and `every two turns`. Articles select modified quantities; `a two creatures` remains rejected.
Distributive quantities have singular external agreement, and `each two turns` and
`every two creatures` are quantity nominals.
The Targeting Marker is `det:target`, the last word of its Determiner
(`DeterminerPhrase "target" as Determiner of creature` in `target creature`,
`DeterminerPhrase "two target" as Determiner of creature` in `two target
creatures`); the Target Noun has no attributive noun use. Bare numeric arguments select Amount
complements rather than becoming arbitrary recipient or object Noun Phrases;
numeric subjects and predicatives retain their Noun Phrase analyses.
The `vocabulary_batch_1` judgment for `different kinds of counters` requires
the noun-headed nominal and its selected `of` Complement, without pinning a
separate intermediate `kinds` projection.

## Stratum 2

Auxiliaries, negation, predicative complements, infinitival and catenative
complements, and gerund-participial clauses.

Stratum 2 adds auxiliaries and modals as heads of their Verb Phrases with
bare-infinitival, perfect, progressive and passive complements; synthetic and
analytic negation (`can't`, `doesn't`; `not` after the copula or before a
*to*-infinitival); *to*-infinitival Clauses as Catenative Complements and as
Purpose Adjuncts (not on a head whose frame selects one); gerund-participial
Complements and preposed adjuncts; existential *there* with a Displaced Subject
and an optional locative or temporal Extension, a Complement of *be* that is
never also the Displaced Subject's postmodifier; and predicative Complements of
`be`, `become`, `remain`, `get` and complex-transitive verbs, including
Preposition Phrases that could modify a nominal (*face down*, *of the chosen
type*). Locatives after `be` and `remain` are Locative Complements, not
predicatives. Passives carry their `by` phrase as a Complement; it is *by* plus
a Noun Phrase. A selected *by* extent (*reduced by {2}*, *increased by two*) is
also an Internalised Complement Reading, which semantics rejects by type.
Adjectival Passives come from frames whose passive leaves no other Complement;
they are predicative Complements only.

## Stratum 3

Relative clauses, bare relatives, and participial and adjectival postmodifiers.

The profiler assigns reduced relatives and `named` postmodifiers here, including
adverbial relatives (*the next time a source would deal damage*), relatives
after keywords and symbols (*with flying that's attacking you*, *mana value X
whose controller*) and postmodifiers of a keyword or symbol sequence. A relative
after a numeral (*two that share …*) belongs here even inside a keyword
parameter.

Stratum 3 adds Relative Clauses as Postmodifiers of a nominal, a quantified
nominal (*the next 3 damage that would be dealt*) or, when marked, a fused head
(*one that hasn't been chosen*). A bare relative has an object, predicative or
stranded-preposition Gap, and its Clause opens with its Subject, never with a
preposed adjunct (CGEL, Ch. 12 §3.5.5, p. 1055); *that* or a relative pronoun
also introduces a subject Gap, which agrees in number with the antecedent; a
*whose* Relative Phrase is Subject or fills an object or predicative Gap (*the
player whose turn it is*); a Preposition Phrase may be fronted with its relative
pronoun (*for which*); an adverbial relative of a temporal nominal has an
Adjunct Gap (*the next time you cast a spell*). Fused relatives (*whichever
comes first*) are Noun Phrases. A temporal Noun Phrase may be preposed (*This
turn, ...*), interleaved before a selected Complement (*dealt to it this turn by
...*) or postmodify a temporal nominal (*the first time each turn*). *Cost*
selects a hollow *to*-infinitival (*costs {0} to cast*), and a purpose adjunct
may contain the Gap a relative clause binds (*the first spell that mana from a
Treasure was spent to cast*).

A temporal Noun Phrase Adjunct is a period noun with a deictic or distributive
determinative and no postmodifier (*this turn*, *each combat*, *this game*),
serial *time* with any determinative (*any time*, *an additional time*, *three
times*), or a temporal nominal with a relative clause (*the next time you cast a
spell*); *the turn* and *your turn* are ordinary Noun Phrases. A period noun
with a deictic or distributive determinative is also an ordinary Noun Phrase
(*skips that turn*, *the beginning of each end step*, *after this phase*), so
*can't block this turn* has an Object Reading beside the Adjunct one, which
semantics rejects. A postpositive Adjective Phrase with its own Complement may
be postposed out of the Object past the verb's selected Preposition Phrase
Complements (*put a number of counters on it equal to the result*), still the
Object's Modifier, ranked behind the Adjective Phrase in place. A copular verb
takes a central adverb before its predicative Complement (*become no longer
suspected*); a symbol sequence takes a postmodifier (*each {G}{G} spent to cast
it*); a bracketed loyalty symbol is also a counted noun (*an additional [+1]*).
Loyalty brackets retain their Stratum 0 notation role. Table rows inherit the
stratum of their payload.

Quantified frequency adjuncts include *a number of times equal to the
difference* (CGEL, Ch. 8 §9, p. 715 n. 19).

## Stratum 4

Triggers, prepositions with clause complements, and content clauses.

The profiler assigns triggers here, with a keyword used as a verb (*as you
cascade*).

Stratum 4 adds Clause Complements of prepositions: a finite Clause (*when this
creature enters*, *unless its controller pays {3}*, *as though it had flash*) or
a reducible verbless one (*if able*, *while saddled*). A triggered ability
[CR#603.1] is a Clause with its *when*, *whenever* or *at* phrase preposed as an
ordinary Adjunct; an intervening *if* phrase is a second one, and a modal header
is a Clause too (*When this creature enters, choose one —*). An interrogative
content clause, the Complement of *choose*, *guess* or *decide*, is *whether*
with a finite or *to*-infinitival Clause, a *which* phrase as Subject or filling
a Gap, or *how* with a Clause. *, where X is ...* is a Supplement to a Clause, a
keyword or an indexed instruction, never a Predicate Adjunct. A comma-set
post-adjunct (*, except it has flying*, *, unless …*) or Supplement attaches to
a finite Clause, never to a Verb Phrase, a *to*-infinitival or a relative's
Clause. Only such a comma-set Preposition Phrase takes a comma-delimited Clause
coordination as its Complement (*, except it's an artifact, and it has flying*).
A gerund-participial Clause may be Subject; a bare relative may relativize a
subjective predicative Complement for an ordinary noun, under a lexical verb or
a negative auxiliary (*the permanent it becomes*, *a color the other isn't*),
never affirmative *be* (*the player it's attacking* has an object Gap); a bare
relative postposes past *at random* (*choose a creature at random this Aura can
enchant*); *all* floats after a pronoun Subject. A Prepositional verb has a
stranded passive (*each card looked at*), and a passive participial Supplement
follows a Clause (*, followed by an additional main phase*). Keyword rules
declare the nouns they define (*if tribute wasn't paid*).

Expandable content-clause Complements retain the explicit subordinator in
*except that the copy is red*, beside unmarked *except the copy is red*;
non-expandable prepositions do not gain that form (CGEL, Ch. 11 §4.8, p. 971;
OSG §12, Copies).

## Stratum 5

Partitives, comparisons and arithmetic.

Stratum 5 adds partitives, comparisons and arithmetic. A determinative heads a
partitive Noun Phrase with its selected *of* oblique (*each of them*, *all of
that mana*), which takes the oblique's Number, a distributive head's singular,
or with free-choice *any* either (*if any of those cards shares ...*); a
quantity takes one too (*two of those cards*, *that many of those tokens*, *up
to two of them*, *a third of their life*), and a relative clause may follow
(*each of those cards that has suspend*). A number-transparent noun under *any*
takes its oblique's Number (*any number of target creatures become ...*); *the
number of* stays singular. *All* is two determinatives, as *more* is: one over a
count plural, one over a non-count singular (*all Equipment* is plural, *all
damage* singular). *Up to N* is a Preposition Phrase in Determiner function or a
fused head (*choose up to one*); *at least*, *exactly* and *no* modify a
quantity (*no more than one creature*). A comparative determinative selects
*than* (*more than one creature*, *more than twice*) and may postpose it past
its head (*more lands than you*); a comparative adjective selects it (*greater
than 2*), postposes it as an indirect Complement (*greater power than this
creature*), as *same* does *as*, or takes a measure phrase (*2 greater*, *{1}
less*, *at least 7 life more than ...*). A postpositive Adjective Phrase with a
Complement follows its noun (*damage equal to its power*, *turns other than
yours*). *Plus* and *minus* head Preposition Phrases on a quantity (*2 plus the
number of ...*, *X plus 3 life*, *that many plus one*), and *N times X*
multiplies an amount. After *tied for*, Oracle drops the article a superlative requires
(*tied for fewest*, *tied for lowest life total*); *no* modifies a comparative
determinative with no *than* Complement (*no more scream counters*); *all*
precedes a fused cardinal (*if you've done all four*). A multiplying adverb
(*twice that many*) modifies a Noun Phrase or a measure, never a temporal Noun
Phrase: *twice this turn* is the frequency adverb and a second Adjunct, as *once
each turn* is. *But* coordinates Clauses (*that damage is dealt, but all other
damage is healed*).

## Stratum 6

Coordination.

The profiler assigns asyndetic participle premodifiers (*nonattacking,
nonblocking*) here.

Stratum 6 coordinates Clauses, Verb Phrases, Noun Phrases, nominals, noun and
participle premodifiers, Adjective, Preposition and Adverb Phrases, keyword
phrases, relative and interrogative clauses, determinatives and symbol costs,
binary and serial, after *both*, *either* or *neither*, and in a delimited
polysyndetic chain (*A, or B, or C*). An *and*-coordination of Noun Phrases or
nominals is plural, but singular when every conjunct is determined by a
distributive determinative (*each attacking creature and each blocking creature
doesn't untap*); *and/or* is plural after a plural last conjunct and of either
Number after a singular one; an *or*-coordination of nominals shares its
conjuncts' Number and of Noun Phrases takes its last one's. A postmodifier
agrees with each conjunct of a coordinated nominal. Bare imperatives and
declaratives coordinate in either order (a declarative before an imperative only
after a comma); a preposed adjunct scopes over the whole chain. Unlike
categories coordinate in one function: depictives, attributes, postmodifiers,
predicate adjuncts (an ordinal's position and a locative *on*-phrase: *second
from the top or on the bottom*) and *and*-lists of an Object with keywords
(*base power and toughness 4/4, flying, and haste*). Right Node Raising shares
an NP, keyword phrase, *to*-infinitival, predicative, bare infinitival, a noun's
single-preposition Complement (*the top or bottom of their library*), a
preposition's Amount (*less than or equal to X*) or, respectively, nouns'
Amounts under a slash Measure (*base power and toughness 4/4*) or under one
Amount (*power or toughness 2 or less*). A bare *power* or *toughness*, or a
coordination of them, is a Noun Phrase only when it ends in its value or another
dependent (*power equal to its toughness*, *base power and toughness each equal
to X*) or follows a graded adjective that implies the value (*creatures with
greater power*, *equal toughness*). It is never one by premodification or
coordination alone (*base power*, *power or toughness*). Its counted plural is
an ordinary bare plural (*different powers*). The Targeting Marker's determiner
takes a whole coordination of nominals, each with its own postmodifiers or none
(*up to one target artifact or creature*, *target planeswalker that player
controls or battle that player protects*).

## Stratum 7

The long tail: replacement, discourse sequencing, pro-form *do*, ellipsis,
absolute clauses, Bracketed text, apposition, quoted abilities, metalinguistic
mentions, words on a Type Line that are not types, and units with a vocabulary
gap.

*Do the same* is a pro-form, with *do so*. *The same is true for ...* is one too, and a
text-changing effect's *instances* of words are a metalinguistic mention.

The profiler assigns Bracketed text, a cleave line read as its Uncleaved Text,
and metalinguistic mentions of a subtype (*with Vampire*) here. A stranded
auxiliary whose Verb Phrase is understood (*each opponent who can't loses 3
life*) is Ellipsis, here like pro-form *do*.

Stratum 7 adds joint coordination: a singular determiner over an
*and*-coordination of singular nominals is a singular Noun Phrase (*a Cleric,
Rogue, Warrior, and Wizard*; *that base power and toughness*). An attributive
adjective may modify a whole *and*-coordination of singular nominals, its shared
Complement inside it (*total [power and toughness 5 or less]*). Floated *each*
and *both* follow a plural or second-person Subject or an auxiliary (*you each
lose*, *this creature and target creature both gain*), and *each* precedes a
postmodifying Preposition Phrase (*cards each with mana value 3 or less*); fused
*each* is an implicit partitive Subject (*each enters with*) or a quantity's
oblique (*create one of each*). A temporal Noun Phrase is an unlike
predicate-adjunct conjunct (*that many times or until you lose a flip*). A
preposition that selects both an Amount and a Noun Phrase also selects their
unlike coordination (*equal to 2 or the greatest power among ...*), and a
trailing Adjunct may scope over a coordination of Clauses.

Complement Clusters end in an optional Adjunct, and any number coordinate after
their shared verb, parallel by frame shape (*has lifelink if you control a
Cleric, deathtouch if you control a Rogue, ..., and flying if you control a
Wizard*; *has trample as long as ..., and "{B}: Regenerate this creature" as
long as ...*). A later Cluster may elide the keyword that heads the initial
Cluster's Object and keep its parameters (*gain protection from white if you
control a Plains, from blue if you control an Island, ...*). A medial conjunct
may elide an element at its beginning, recovered from the initial conjunct: a
copula before a Predicative (*is enchanted, equipped, and has a counter on it*)
or a Subject before a finite Verb Phrase, agreeing with it (*you gain 4 life,
draw four cards, and he deals 4 damage to each opponent*); the initial and last
conjuncts never do. A keyword after *with* is a predicate's Adjunct only
coordinated with a *with*-phrase that is one on its own (*enters with flying and
with X +1/+1 counters on it*). A main Clause may take a qualifying added element
after *but* or *and*, a fragment of Adjuncts focused by *only* or *not* (*Any
player may activate this ability but only as a sorcery*). *But* joins a Noun
Phrase or a Preposition Phrase to a *not*-modified one (*life equal to the
damage dealt, but not more life than ...*; *from your graveyard, but not from
anywhere else*). Unlike predicate Adjuncts coordinate serially; the conjuncts of
an unlike serial need only not all be alike, so its first and last may match
(*indestructible, "...," and equip {0}*); a serial coordination of Clauses with
an imperative first and a declarative last admits either mood between, its last
a *then*-linked one too (*reveal cards ..., Kaboom! deals damage ..., then you
put ...*); a non-initial Clause conjunct may open, after its coordinator, with a
fronted Adjunct set off on both sides (*and, if it's a creature, it enters
...*). Plural *those* is a fused determiner-head (*except those that share a
creature type with ...*), and *either* and *neither* fuse in an implicit
partitive (*If a player does either*).
