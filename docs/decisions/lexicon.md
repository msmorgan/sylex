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
inferred from spelling: braces do not license an unknown symbol. The one open
slot is the Flavor Word, because nothing enumerates them [CR#207.2d]. Why: a
guessed Category lets the grammar admit text no declaration accounts for, and
the gap is never reported.

**Every irregularity is declared.** Each Word Form has one default spelling
rule, and every exception, consonant doubling included (*committed*), is an
override on its Lexeme; nothing infers stress or a paradigm class. Why: each
irregularity is a reviewable fact, and the rules that analyze a form are the
rules that realize it.

**A missing feature is inapplicable, not a wildcard.** Forms spelled alike keep
their own Feature Bundles. The one merge is a finite form spelled alike in both
numbers (*entered*, *can*), which is one bundle unmarked for Number. Why: two
bundles would be two Readings differing in a Number the text does not mark, a
second tree ([English](english.md) §8).

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
singular ending in *s* whose plural differs and a name ending in *s* included
(*players'*, *elves'*, *Thalakos'* singular or plural, *player's*, *Spawn's*,
*Swarm of Rats's*; CGEL, Ch. 18 §4.2 [35], p. 1596, where *species* takes the
bare genitive "in the singular as well as the plural", and the bare genitive of
a name is only a variant, [35ii], which Oracle does not write). The host's
realized forms decide it: no word lists the hosts. The grammar checks the
agreement between each word and the next, whatever Constituents they end and
begin. Why: a free choice of spelling lets the parser accept *a artifact* and
*players's*, and leaves realization nothing to derive the spelling from but the
source text.

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
*Khans* is `khans.syl`, of the AnchorWord sort. Why: a word's wording and
meaning are one fact, so they have one source and cannot drift apart.

**The core lexicon is only ordinary English**: the closed classes and the
open-class words Oracle uses as plain English (*other*, *top*), plus the
notation symbols. Loading refuses a Core word that is also a Catalog item of the
same Category, because its Core spelling would let the grammar name the item.

**Names come from the Oracle data**, never by hand: card names from the card
data, Shortened Names from a rule over a face's name, supertypes and types
(`sylex_vocabulary::short_name`) checked at load against each face's own Oracle
text (§6). Why: the card list is the authority on what cards exist.

## 4. Binding

**A singleton is reported, not refused.** Binding reports a Lexical Class with
one member, a feature with no value, and a value with fewer than two holders.
Why: that is how a word flag looks ([English](english.md) §3), so it gets a
look, but it is no defect when the construction really has one holder.

## 5. Multiword terms

A multiword term declared outside the core lexicon is one leaf, but its words
keep their own analyses where they straddle a constituent boundary: in *if it's
the first time counters have been put on that creature*, *time* ends the
Nominal *first time* and *counters* begins the clause, so the Catalog term *time
counter* does not hold them. The words straddle a boundary only when, in the
smallest Constituent spanning the term, the child holding its first word begins
before the term and the child holding its last word ends after it; otherwise
they would rebuild the term, alone or with a dependent of one word. A core
multiword term (*mana value*) never splits, nor does a term with a part that is
not an ordinary word (the measure of *+1/+1 counter*). A multiword noun whose
first word is also a preposition is no barrier: its words form a Preposition
Phrase, never the noun's own analysis. Why: words that meet by accident are a
different analysis from the term coined from them, while words that rebuild the
term are a second tree of its analysis ([English](english.md) §8).

## 6. Names and notation

**Names are scoped.** A card's own name, full or Shortened, is its own only in
the text of its own face [CR#201.5,201.5c,712.8d..712.8f], and everywhere in
that text. A card has a Shortened Name only where its own text uses one, and
never one that other vocabulary in scope on that card already spells: a token
name or Introduced Name always, any other word unless it is the name before the
comma or a planeswalker type. Why: admitted everywhere, every word spelled like
one would be read as a name on every card.

**A Shortened Name is a form of its card name.** A card name's Lexeme has a
Full form and, where the card has a Shortened Name, a Shortened form, told
apart by a feature and each with one spelling. Text that uses the Shortened
Name refers to the card as its full name does [CR#201.5c], so a Reading of
*Chandra* on Chandra, Pyromaster is that card's own Lexeme in its Shortened
form, and realization spells *Chandra* from the tree alone. Only its own card
admits the Shortened form; the scope is keyed by form, not by spelling. A Card
Scope holds each of its card names in the forms the text admits, a face's own
names in every form, and one predicate of the lexicon, `Lexicon::stands_in`,
says whether a value stands in it: analysis finds only such values, and
admission refuses any other, so a Reading built apart from a parse is held to
the scope its text would be parsed in. Why: one name is one Lexeme, and a
spelling chosen apart from the form would be a free variant the tree does not
record; a scope checked by parsing alone would admit a tree that no parse of
its text yields.

**Another card's name stands only after *named*, *Partner with* or *from
among*.** A card's full name is spelled on every face, and off its own card it
is another card's name, which a position admits only by opting in: no leaf
symbol of the grammar admits it but `OtherCardName()`, which admits nothing
else. Three positions opt in, as a `NameComplement`, alone or coordinated: the
predicative complement of *named* (*cards named Relentless Rats*); the Keyword
Parameter of *Partner with*, whose name is the one its ability searches for as
*a card named [name]* [CR#702.124j]; and the complement of *among* after
*from*, wherever that phrase attaches, as in the options of a choice (*choose a
card name ... from among Disenchant, ..., and Black Lotus*). *From* selects
that *among* phrase by its Complement's kind and *among* takes a name only so
selected, as *do the same for* selects *for* with a cited word
(`PPWith(["for"], Mention)`): a name after *among* under any other head
(*divide it among Black Lotus*) is refused. Any other position, a Subject,
Object, possessor or premodifier among them, refuses another card's name with
no clause of its own. A name a card gives an object, or a vote option, spelled
like another card's name is listed as that card's Introduced Name or vote
option (*create a Tarmogoyf token*, *votes for Redhorn Pass or Mines of
Moria*). Why: a card's text mentions another card by name only in those
positions, and a card name read anywhere else is another word misread (*Exile*
in *Exile target attacking creature*).

**Bound keywords are analyzed in parts**, a suffix on a stem from its declared
stem classes, so *islandwalk* needs no declaration [CR#702.14a]. A stem is
never an independent noun.

**Numerals own no spelling.** The numeral codecs read cardinal, ordinal, Arabic
and Roman notation beside the ordinary entries, so *one* and *I* keep their
other analyses.
