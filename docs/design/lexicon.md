# Lexicon

How words reach the English grammar. The lexical half of the parser contract
is [English](english.md) §7 and the Catalog constraint §2; terms are the
[glossary's](../contexts/oracle-english/CONTEXT.md).

## 1. The analyzed Oracle text

English reads a Card Face's Oracle text, Editorial Corrections applied, with
two changes, and the exact round trip is over the result
([Architecture](architecture.md), "Exact round trip"). A cleave line is read as
its Uncleaved and its Cleaved Text.

**Reminder text is removed.** It summarizes a rule that applies to the card
[CR#207.2a] and has no game function ([style guide](../oracle-style-guide.md)
§3, "Parentheses"): the keyword it explains already carries what it says.

**Keyword-line semicolons become commas** on a line that lost reminder text
and holds no sentence. Why: Oracle uses the semicolons only because a keyword
carried reminder text (style guide §8, "Keyword abilities on a shared line"),
so a keyword list keeps one separator and a Reading need not store which was
written.

## 2. Lexical analysis

`sylex_lexicon` depends on no grammar, vocabulary file or Game Model. Why: a
crate that cannot see the grammar cannot scan for it.

**Analysis commits to no word split.** Lexical matches are overlapping edges:
a multiword term and its words, or a host and its bound affix, stand as
alternatives, and the chart decides between them (§5).

**Nothing is guessed.** An unknown word is a vocabulary gap, and no Category is
inferred from spelling: braces do not license an unknown symbol. Five slots
are open, each read from the text at its own positions because nothing
enumerates what fills it. A Flavor Word is the italic run before an ability's
em dash [CR#207.2d]. An Introduced Name is a name the text itself gives an
object [CR#111.4], the title run written where the text introduces it (§6).
An Anchor Word is one of the two words a face's text offers as its permanent
enters, each heading the bulleted ability linked to that choice
[CR#614.12c,607.2m] (*choose Khans or Dragons*; *• Khans — ...*). No one line
settles it: a bulleted line that opens with a label may be a flavor-labeled
mode (*• Thay — ...*, Cult of Skaro), and a line that offers two capitalized
words may offer two subtypes (*choose Island or Swamp*, Roots of Life
[CR#607.2d]). So it is read across a face's lines: they are parsed with the
label of every bulleted line in scope as an Anchor Word, and the labels that
an Anchor Choice in some Derivation of a line offers are the face's Anchor
Words. Its Card Scope holds them (§6), and an Anchor Word leaf stands only
where the scope holds its word, at the head of a bulleted ability and in an
Anchor Choice; there the label is no Flavor Word. A text read alone states
its Anchor Words as it states its card's name.
A Vote Option is one of the words a vote offers, "words with no rules meaning
that are each connected to a different effect" [CR#701.38a,701.38b] (*votes
for sickness or psychosis*; *If psychosis gets more votes*; *for each numbers
vote*). It has no shape of its own: it is any word (*free*, *return*,
*planeswalk*) or title run (*Mines of Moria*), and the text's other uses of
that word stay what they are (*chaos ensues* on the Path of the ... cards). Nor
does the offer alone settle it, since a vote may as well offer what two words
name [CR#701.38b]. So it is read from the face's lines by two parses. A line
that writes a word whose frame selects an Option Choice is parsed with every
word and title run in it in scope as a Vote Option, and the words an Option
Choice in some Derivation offers are what its vote offers. The face's lines are
then parsed with those words in scope, and an offer is confirmed where some
Derivation cites one of its words in an Option Mention, as the Subject of a
clause that counts its votes or before *vote*: that is the word's connection to
an effect. The words of a confirmed offer are the face's Vote Options, both of
them (*innocent*, which Trial of a Time Lord offers and never cites). Its Card
Scope holds them (§6), and a Vote Option leaf stands only where the scope holds
its word; the grammar admits it in an Option Choice and an Option Mention and
nowhere else. A text read alone states its Vote Options as it states its
Anchor Words. Every printed vote has its offer and its mentions in one line,
but the words are still read before that line's Readings are taken, not within
them as an Introduced Name is: lexical analysis proposes an Introduced Name at
a title run, a shape, and a Vote Option has none to propose it at.
A Set Name is the name of an expansion a card's text cites before the noun *expansion* [CR#206.3a..206.3c] (*a name originally printed in the
Antiquities expansion*): no card-scoped list or catalog of printed sets is
consulted. It is read by title shape alone, right before a noun whose parameter
pattern takes a Set Name before it: the longest run of whole space-separated
words before the noun that begins and ends on a title word (one that begins with
an ASCII capital or digit and holds only alphanumerics and `:`, `'`, `-`, `.`,
`,` and `/`), with the connectors *of*, *the*, *to*, *in*, *for*, *into*, *at*,
*over*, *and*, *a*, *vs.*, `&`, `/` and `×` between them (*Phyrexia: All Will Be
One*, *The Brothers' War*, *Shadows over Innistrad*, *Warhammer 40,000
Commander*). The name is the whole run, never a shorter run inside it (*Urza's
Saga*, never *Saga*; *Adventures in the Forgotten Realms*, never *Forgotten
Realms* after *the Adventures in the*). No run is refused for spelling an
English word or a card's name; a Set Name is no Proper Name, so it stands only
as its noun's parameter, which premodifies it. The shape covers every set in
mtgjson's `SetList.json` but *The List (Unfinity Foil Edition)*, whose
parentheses reminder-text removal strips (§1). Why: a guessed Category lets the
grammar admit text no declaration accounts for, and the gap is never reported;
an open slot read at its own positions is no guess, and a list of the names
cards cite would be knowing the cards before parsing them.

**Every irregularity is declared.** Each Word Form has one default spelling
rule, and every exception, consonant doubling included (*committed*), is an
override on its Lexeme; nothing infers stress or a paradigm class. A word's
Onset is its first letter's, a vowel letter's Vowel and any other's Consonant,
and a Lexeme whose first sound is not its first letter's declares its Onset
(*an hour counter*, *a unity counter*, *a Unicorn*, *an X/X*); a numeral's is
its spoken cardinal's (*an 8/8*, *a 1/1*), and a measure's sign is spoken
*plus* or *minus*. A spelling with neither has no Onset. Why: each irregularity
is a reviewable fact, and the rules that analyze a form are the rules that
realize it.

**A missing feature is inapplicable, not a wildcard.** Forms spelled alike keep
their own Feature Bundles. Two kinds of form are each one bundle unmarked for
Number: a finite form spelled alike in both numbers (*entered*, *can*), and a
name (§6). Where the text shows agreement, the agreeing Clause records the
Number in the Reading (*When The Notary Hobbits enter* is a Plural Clause,
*Mizzium Mortars deals* a Singular one), and the form's own node stays
unvalued; where the text shows none, the Number stays unvalued. Why: two bundles would be two
Readings differing in a Number the text does not mark, a second tree
([English](english.md) §8).

**A form has one spelling, and its context selects it.** Each slot of a Lexeme
is spelled one way, so each lexical value realizes one way. Oracle text has no
free variation, and no curly quotation mark or apostrophe. Its allomorphs are
forms of one Lexeme told apart by an Adjacency Agreement feature, which the
word next to it fixes as a subject fixes a verb's Number: the indefinite
article's by the Onset of the word after it (*a creature*, *an artifact*, *an
8/8*, *a +1/+1 counter*), *non*'s by how the host after it takes a prefix,
hyphenated before a subtype or *outlaw* and solid before any other noun,
adjective or verb (*non-Vampire*, *nonland*, *nonattacking*), while a word of
any other Category takes no prefix and agrees with neither, and the genitive's
by the host before it: the bare genitive after any Plural noun form ending in
*s*, regular or irregular, and after the singular of a noun spelled alike in
both numbers with final *s*, and the *'s* genitive after any other form, a
singular noun ending in *s* whose plural differs and a name the text treats as
singular ending in *s* included (*players'*, *elves'*, *Thalakos'* singular or
plural, *player's*, *Spawn's*, *Swarm of Rats's*; CGEL, Ch. 18 §4.2: [35i], p.
1595, and p. 1596, where the bare genitive is obligatory with plural nouns
ending in *s* and *species* takes it "in the singular as well as the plural";
for a singular proper name ending in *s* the bare genitive is only a variant,
[35ii], p. 1595, and Oracle writes *'s*). The host's realized forms decide it:
no word lists the hosts. The grammar checks the agreement between each word and
the next, whatever Constituents they end and begin; a mark is a word there only
where its Production declares it spoken, as a sign is, and the lexicon's word
for the sign gives its Onset (*plus*, *minus*: *a +8/+8 creature*), since one
character is a sign in one construction and a range's dash in another. Why: a
free choice of spelling lets the parser accept *a artifact* and *players's*,
and leaves realization nothing to derive the spelling from but the source text.

**A word in two Parts of Speech is two Lexemes**, related by Conversion, never
one Lexeme with a fused-head use standing in for the other: *red* is a noun in
*protection from red*. An adjective heads a fused-head Quality only where it
has no noun of its own (*protection from multicolored*), and the lexicon
declares it there. Why: both analyses for one position would be a second tree.

## 3. Provenance and sources

A lexical value's Provenance is part of its identity and has three kinds:
**Core**, ordinary English declared in `sylex_vocabulary`; **Catalog**, game
vocabulary; and **Symbol**, notation. Why: Provenance exists to check the
Catalog constraint, which needs only the line between ordinary English and game
vocabulary, so a finer split of either side adds nothing.

**Every game word is a `.syl` declaration** of its own sort under `plugins/`,
its `lexical` field beside its spelling and meaning ([Syl](syl.md) §1–2):
*creature* is `creature.syl`, of the `card_type` sort. A Catalog verb that means
nothing on its own and is shared across phrases (*face* [CR#700.17,701.55],
*beam* [CR#702.196]) is a `verb` declaration (`Verb`) carrying only `lexical`,
which each declaration using the verb names in its `lexical`. Why: a word's
wording and meaning are one fact, so they have one source and cannot drift
apart; where two phrases share a head verb, neither owns the Lexeme.

**The core lexicon is only ordinary English**: the closed classes and the
open-class words Oracle uses as plain English (*other*, *top*), plus the
notation symbols. Loading refuses a Core word that is also a Catalog item of the
same Category, because its Core spelling would let the grammar name the item.

**Names come from the Oracle data**, never by hand: card names from the card
data, and nothing else about a card; and dungeon names from the faces of card
type Dungeon in the card data's token sets, oversized aids apart. The lexicon
holds no card's type line, text or Shortened Name: a face's Card Scope is
derived when its text is parsed (§6). A dungeon is a card outside the
corpus, brought into the game by venturing [CR#309.1,309.2], so no Card Scope
restricts its name: any card's text may name it (*if you haven't completed Tomb
of Annihilation*). Why: the card list is the authority on what cards and
dungeons exist, and a fact about one card prepared before parsing is the parse
being told its answer.

## 4. Binding

**A singleton is reported, not refused.** Binding reports a Lexical Class with
one member, a feature with no value, and a value with fewer than two holders.
Why: that is how a word flag looks ([English](english.md) §3), so it gets a
look, but it is no defect when the construction really has one holder.

## 5. Multiword terms

A multiword term declared outside the core lexicon is one leaf, but its words
keep their own analyses where they straddle a constituent boundary: where the
first word ends one constituent and the last begins the next, the Catalog term
does not hold them. The words straddle a boundary only when, in the
smallest Constituent spanning the term, the child holding its first word begins
before the term and the child holding its last word ends after it; otherwise
they would rebuild the term, alone or with a dependent of one word. A core
multiword term (*mana value*) never splits. A multiword noun whose
first word is also a preposition is no barrier: its words form a Preposition
Phrase, never the noun's own analysis. Why: words that meet by accident are a
different analysis from the term coined from them, while words that rebuild the
term are a second tree of its analysis ([English](english.md) §8).

A licensed term is two words, a word that holds a license feature directly
before the word that carries its head value ([English](english.md) §3): a
counter's kind and *counter* (*time counters*). It is no leaf: its own
analysis is the Production that admits the head (`:= Head`), built from its
words. Its words keep their other analyses as a Catalog term's do, only where
they straddle a constituent boundary, and only its own analysis may rebuild
them. So *five or more plot counters* reads no verb *plot* with *counters* its
Object (Deadly Designs) and *a training counter* no participle premodifying
*counter* (Sensei Golden-Tail), while *the first time [counters have been put
on that creature]* stands (Stalwart Successor). Why: a kind name's homograph
and its head that form one constituent are the kind's analysis built a second
way, as a Catalog term's words are.

## 6. Names and notation

**Names are scoped.** A card's own name, full or Shortened, is its own only in
the text of its own face [CR#201.5,201.5c,712.6], and everywhere in
that text. A face's Card Scope holds, of card names, its own name and nothing
else, in its Full form and, where the card has a Shortened Name, in its
Shortened form; beside the name it holds the face's Anchor Words and Vote
Options (§2). The name
of a meld result is the name of neither card of its meld pair, so it is another
card's name on both, and neither card's text shortens it: *Mishra* on
Phyrexian Dragon Engine is no name.

**A legendary permanent has a Shortened Name, whatever its text says.** The
Oracle convention gives the shortened form to a legendary name
([style guide](../oracle-style-guide.md) §5: "An unambiguous shortened form
may replace a long legendary name"), so a face has one when its type line is a
legendary permanent's and its name has a shortened spelling
(`sylex_vocabulary::face_scope`). Whether its own text uses the form is no
condition: *Abuelo, Ancestral Echo* never says *Abuelo*, and *When Abuelo
enters* still reads on it. The scope is derived from the face's name and type
line when its text is parsed, and nothing about the card is prepared
beforehand. A text read as if on a card of a given name, which need be no
printed card's, states whether that card is legendary
(`sylex_vocabulary::stated_scope`), and without that only the Full form
stands. A card has no Shortened Name that other vocabulary in scope on it
already spells: a token name always, any other word unless it is the name
before the comma or a planeswalker type (*Sliver* on Sliver Queen). Another
card's name is no such vocabulary, since it competes with the Shortened Name
only after *named* (*Nicol Bolas* on Nicol Bolas, the Deceiver). Why: admitted
everywhere, every word spelled like a Shortened Name would be read as a name
on every card.

**The shortened spelling is a rule over the name alone**
(`sylex_lexicon::shortened_spelling`): a listed title and the last word before
any comma (*Captain Kirk* for Captain James T. Kirk); else the words before
the comma (*Dina*); else, for a name with no comma, nothing when it starts
with *The*, and otherwise the name less a trailing Roman numeral, or cut before
its first *and the*, *the* or *of*, or its first word (*Nissa* for Nissa
Revane). A cut of a name with no comma that ends in a possessive word, or just
before one, is no Shortened form (*Gaea's* of Gaea's Cradle, *Marit* of Marit
Lage's Slumber), while a possessive later in the name leaves the cut standing
(*Krydle* for Krydle of Baldur's Gate). The
titles are listed because nothing in a name's words shows which one is a
title. A longer title the text names that starts with the Shortened Name
(*Mishra, Lost to Phyrexia* on Mishra, Claimed by Gix) withholds nothing: the
longer title is another card's name at its position, and *Mishra* alone is the
card's own. Inside the card's own full name, written out, its Shortened form
is no name of its own: on Instrument of the Bards, *Instrument of the Bards*
is the name, never the Shortened Name *Instrument* and a phrase *of the
Bards*. Why: the full name refers to the card [CR#201.5], and a reading of its
first words as the Shortened Name and the rest as a dependent is a second tree
for a text that says one thing.

**An own name is its own kind of leaf.** A name the Card Scope holds is read as
`LexicalReading::OwnName`, which records the card's full name and the form
written, and is spelled from those alone: the Full form is the name, the
Shortened form the rule's spelling. It needs no Lexeme, so a text may be read
as if on a card of any name. Text that uses the Shortened Name refers to the
card as its full name does [CR#201.5c], so a Reading of *Chandra* on Chandra,
Pyromaster is that card's own name in its Shortened form, and realization
spells *Chandra* from the tree alone. The scope is keyed by form, not by
spelling, and one predicate of the lexicon, `Lexicon::stands_in`, says whether
a value stands in a scope: an own name where the scope holds that form of that
name, and a card name's Lexeme, which is another card's name, everywhere but
where the scope holds its name. Analysis finds only values that stand, and
admission refuses any other, so a Reading built apart from a parse is held to
the scope its text would be parsed in. Why: one name is one leaf whatever its
form, and a spelling chosen apart from the form would be a free variant the
tree does not record; a scope checked by parsing alone would admit a tree that
no parse of its text yields.

**A name is number-neutral.** No name declares a Number: a proper name has the
Number of what it names, which only its text shows, so its Lexeme is unmarked
for Number (§2). Why: a declared Number is knowledge of the cards before
parsing them.

**Another card's name stands only after *named*, *Partner with*, *from
among* or *meld ... into*, and before *token*.** A card's full name is spelled on every face, and off its own card it
is another card's name, which a position admits only by opting in: no leaf
symbol of the grammar admits it but `OtherCardName()` and, before *token*,
`DistinctOtherCardName()` (below), which admit nothing else. Four positions
opt in as a `NameComplement`, alone or coordinated: the
predicative complement of *named* (*cards named Relentless Rats*); the Keyword
Parameter of *Partner with*, whose name is the one its ability searches for as
*a card named [name]* [CR#702.124j]; the complement of *among* after
*from*, wherever that phrase attaches, as in the options of a choice (*choose a
card name ... from among Disenchant, ..., and Black Lotus*); and the complement
of *into* after *meld* (*meld them into Brisela, Voice of Nightmares*), where
Oracle text uses "the full name when the words identify an actual card name or
face", meld text included (`docs/oracle-style-guide.md` §5); one card of each
meld pair has the ability that melds it [CR#713.1a], and the pairs and their
results are fixed [CR#713.2,713.2a..713.2g]. *From* selects
that *among* phrase by its Complement's kind and *among* takes a name only so
selected, as *do the same for* selects *for* with a cited word
(`PPWith(["for"], Mention)`): a name after *among* under any other head
(*divide it among Black Lotus*) is refused. *Meld* selects its *into* phrase
the same way (CGEL, Ch. 4 §6, pp. 273–274), and *into* takes a name under no
other head (*put it into Black Lotus*). Any other position, a Subject,
Object, possessor or premodifier among them, refuses another card's name with
no clause of its own. A Vote Option spelled like another card's name is no
card's name where its vote offers it or its votes are counted (*votes for
Redhorn Pass or Mines of Moria*, Travel Through Caradhras): those positions
take a Vote Option and no name (§2). Why:
a card's text mentions another card by name only in those positions, and a
card name read anywhere else is another word misread (*Exile* in *Exile
target attacking creature*).

**A token may be named for a card.** One more position takes another card's
name: the premodifier of *token* inside the Object of *create*, which makes a
token with the characteristics of the card so named [CR#111.11] (*create a
Tarmogoyf token*,
*create a tapped Mutavault token*). It opts in through
`DistinctOtherCardName()`, which refuses a card name that any other word is
spelled like: *a Blood token* is the predefined token, though a card is named
Blood. Why: a name that another word also spells, a subtype or a predefined
token's name among them, is that word there, and reading the card's name
beside it would be a second tree.

**An Introduced Name is read from the text.** A name a card's text gives an
object, the name of a token it creates [CR#111.4] or of a permanent or copy
it changes, is listed nowhere, and nothing keyed by card holds it. Analysis
proposes one title run at each capitalized word that begins a word
(`sylex_lexicon`'s `introduced` module): the longest run of capitalized words
and numerals, continuing over lowercase *of*, *the* and *to* before a
capitalized word and over a comma before a capitalized word or *the* (*Voja,
Friend to Elves*; *Ashaya, the Awoken World*), and ending before any other
lowercase word (*named The Mightstone and Weakstone* stops before *and*). A
genitive mark that ends the run is the clitic, no part of the name (*Beau's
power*). Card names are kept out, where a card's full name is one a card
name's Lexeme spells or the Card Scope holds:

- a run that a card's full name spells is no Introduced Name;
- nor is a run that a card's full name written at that position runs past (*a
  creature named Fang, Fearless l'Cie*);
- a comma does not continue a run that a card's full name already spells
  (*Equipment named Helm of Kaldra, Sword of Kaldra, and Shield of Kaldra*),
  and a Shortened Name is no full name, so *Icingdeath* ends no run on
  Icingdeath, Frost Tyrant, which creates *Icingdeath, Frost Tongue*;
- no run starts inside such a card name written out.

The leaf is `LexicalReading::IntroducedName`, a Proper Name spelled as its
run and unmarked for Number, whose genitive is formed as a card name's is. The
grammar admits it through two symbols. `IntroducedName()` stands where a name
is introduced:

- the Complement of *named* (*a 2/2 colorless Wizard Soldier creature token
  named Cadet*; *a creature you control named Vecna*, on a face that
  introduces the name nowhere else);
- the Object of *create*, before the description of the token, a Supplement
  headed by *token* (*create Angelo, a legendary 1/1 green and white Dog
  creature token*); without the description, *Create Angelo.* names nothing;
- the value of *its name is* in a copy's exception (*except its name is
  Mishra's Warform*).

`RepeatedName()` stands where any other name does, as a Subject, a possessor
or a premodifier, and only in a Reading that also introduces that name: a
later sentence (*Ragavan enters tapped and attacking*) or the token's own
quoted ability, which names the token [CR#201.5] (*"Jumblebones can't
block"*, *"Beau's power is equal to ..."*). It is never the Object of
*create*, which offers the name for a new token (*if you don't control a
creature named Mowu, create Mowu, a legendary 3/3 green Dog creature token*).

The grammar names the words of these positions as it names any word: *named*
and *except* as Closed Words, and *create*, *token* and *name* as the Lexical
Classes `NamingVerb`, `TokenNoun` and `NameNoun`, which their declarations
join ([English](english.md) §2).

Whether a Reading that names a name again also introduces it is no fact of
one Constituent, so each Constituent's summary records the names it
introduces and the names it names again, and a root holds the second set
within the first. Only a title run its text writes at more than one place can
be named again, before or after the place that introduces it. Admission
applies the same summary to a Reading built apart from a parse, and holds each
Introduced Name to the title run analysis finds in its text
([English](english.md) §7, the second round-trip law). A title run is a
vocabulary gap unless some partial Production of the parse introduces it, or
names it again where another introduces that name.
Why: the name is a fact the text settles, and a list of names by card is the
parse being told its answer; admitted everywhere, every capitalized word
would be read as a name.

**Bound keywords are analyzed in parts**, a suffix on a stem from its declared
stem classes, so *islandwalk* needs no declaration [CR#702.14a]. A stem is
never an independent noun.

**Numerals own no spelling.** The numeral codecs read cardinal, ordinal, Arabic
and Roman notation beside the ordinary entries, so *one* and *I* keep their
other analyses.
