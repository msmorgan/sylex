# English

The English layer relates Oracle text to Readings. It knows English and
Oracle's conventions, not Magic ([Architecture](architecture.md), "The
layers"). This document records how the grammar is designed and why, and the
contract the parser keeps (§7–8). How words reach the grammar is in
[Lexicon](lexicon.md). Everything in it follows from the
charter in §4: the parser admits English, and it does not check Oracle's
templating.

## 1. The crate split

- `sylex_lexicon` analyzes and realizes words.
- `sylex_vocabulary` declares them: the core English lexicon and the Catalogs,
  each entry with its Subcategorization Frames.
- `sylex_syntax` is the engine. It loads a grammar data file at runtime, binds
  it to a lexicon and its frames, parses into a Packed Forest, ranks Readings
  and realizes them. Outside the lexicon feature-bundle mapping table, the one
  declared interface between lexicon and syntax, it names no grammar feature,
  value or Category. It may compute purely structural facts from its own host
  checks, coordinators, extensions and punctuation.
- `sylex_english` holds the Oracle English grammar data file and its binding,
  together with what belongs to the Oracle corpus: faces, Corpus Units and the
  Census.

Why: each fact has one owner. An engine that names no grammar term cannot grow
a second grammar of its own, and the grammar stays data that is reviewed and
changed without touching Rust. A grammar that names no vocabulary (§2) is
untouched when vocabulary grows.

The grammar is written as Schemas, which the engine compiles into Productions.
An author states what a Schema changes, and the engine derives the rest: the
complete feature summaries and the valence alternations. A new feature,
coordinable Category or lexical distinction that costs dozens of lines is a
defect in the Schemas or their compilation, and it is fixed there. Why: a
hand-written Production for every voice, gap and coordination of every frame
multiplies, and the copies drift apart ([Architecture](architecture.md),
design principle 4).
*Construction* is reserved for spelling's pairing of form with meaning
([Architecture](architecture.md), "Spelling is a construction grammar"). The
grammar is syntax only, and a name that suggests it carries meaning invites
meaning into it.

## 2. The Catalog constraint

No item the Comprehensive Rules enumerate appears in a grammar declaration.
This covers keyword actions [CR#701.1], keyword abilities [CR#702.1], ability
words [CR#207.2c] and every other Catalog in the
[glossary's](../contexts/oracle-english/CONTEXT.md) sense. Each is a game word
declared in its `.syl` file, which brings its own frames
([Lexicon](lexicon.md) §3): a keyword action its Complements, a keyword
ability its Keyword Parameters. The grammar parses Catalog items by lexical Category and frame. It never names one, and it defines
no feature, Schema or Production for one. A keyword action that is also an
ordinary English verb, such as *destroy*, *exile* or *search*
[CR#701.8,701.13,701.23], is still a Catalog item.

The grammar names only core closed-class words, by lexeme id. Core vocabulary
is never a Catalog. It carries Core provenance, which is what lets the grammar name its
closed-class words, and binding refuses a grammar that names anything else.

Why: the Catalogs are open. Sets keep adding keyword actions, abilities and
subtypes, and custom content adds its own. A grammar that named them would grow
with them, and a word named in the grammar attracts Productions that serve that
word alone. Under the constraint, a new keyword action or subtype is a Catalog
entry with its frame, and neither the grammar nor the engine changes.

## 3. Complements and adjuncts

Oracle text is a controlled register
([style guide](../oracle-style-guide.md)). CGEL supplies its terminology and
constituent structure, not its attachment rules (glossary, "Authority"). The
two kinds of dependent are licensed in different places, and the register
narrows only the second, and only as the charter allows (§4).

**A Complement is licensed by its head's Subcategorization Frame.** A frame is
declared once, in the lexicon, and a verb's frame in the active voice. From it
the engine derives the passive, the gapped forms of every verb form, and the
complements that coordinated heads share. It derives them for every frame,
whether or not any card prints the combination. Only an Active-only Frame
derives no passive. In the same way, every declared verb and noun has its full
inflectional paradigm: attestation in the corpus is provenance, never a filter.
Why: these alternations are productive English. Written out by hand, they
multiply as frames times forms. Filtered by attestation, they fail on the next
card that uses one.

**An Adjunct or Modifier is selected by no head.** It is licensed by the
Category of its host. For each pairing of a preposition with one of its frames,
the lexicon declares the Host Categories that the unselected phrase may modify.
That declaration records the attachments English builds in the Oracle
register. For example, unselected *for* plus a Noun Phrase modifies a
Predicate or Clause and never a Nominal, so a noun that takes a *for* phrase
(*cost for …*) declares it in its own frame. Why: open-domain English has
attachments the register never builds (*a book for my children*).

**Same-head complement priority.** A phrase that one of a head's frames
selects as a Complement is never also an unselected Adjunct of the same head
instance: in *Search your library for a card*, *for a card* is *search*'s
Complement only. A noun, adjective or determinative standing bare with its
Complement omitted takes a Preposition Phrase right after it that one of its
frames selects as that Complement, never as a dependent of another host
(*on top of your library*). Attachment to different heads stays packed (§4).
Why: a selected and an unselected attachment to the same head are one
analysis, so keeping both is a second tree (§8), while attachments to
different heads are different analyses that only meaning can choose between.

**A central Adjunct after *be* or a copula.** An adverb in central position
follows an auxiliary and precedes its Complement (*He is always miserable*),
while the same order after a lexical verb is ungrammatical (∗*He looks always
miserable*; CGEL, Ch. 3 §2.1.5, p. 102, [35]); in a clause an auxiliary heads,
central position is more often just after the verb (CGEL, Ch. 6 §7.1,
p. 575). The verb Valence schema derives a phrase with a central Adjunct
between the verb and its first Complement from each frame of *be* and from
every Copular frame, never from a catenative frame. *Still* thus precedes
*be*'s locative Complement (*is still on top of your library*, Commander
Spock) as it does its predicative one (*is still a land*), and a lexical
copula's predicative takes one too (*have it become no longer suspected*,
Deadly Complication). A locative Complement is not a predicative one (CGEL,
Ch. 4 §5.2, p. 257), so *is still on the battlefield* has the single analysis
that *is on the battlefield* has: *be* with its locative Complement. Every
other auxiliary takes a Verb Phrase Complement, and the central adverb belongs
to that phrase (*can still block*, *is still blocking*). Why: the adverb is
the clause's, so the Complement keeps the analysis it has without the adverb.
Deriving the phrase from the frames makes their preference of a locative
Complement over a predicative one (§8) apply to it, so *remain*, whose
locative frame takes none, has no *remains still on the battlefield*.
Limiting it to *be* and the Copular frames keeps the order off other lexical
verbs and off lexical *have*, which the lexicon lists among the auxiliaries:
in *has only one target* (Meddle), *only* stays the Noun Phrase's.

**No policing.** No feature or equation exists only to refuse a form no card
prints. A negative Judgment pins a formation rule that generation needs, such as
agreement, Case or verb form, on a real or cited construction, never an
unprinted word order or combination. Nor does any feature or flag name a word:
an attachment is decided by Category, function and grammatical feature, the
lexicon has no flag that limits which open-class heads may host a general
adjunct, and binding refuses the feature names such flags take. This follows
from the charter (§4). Why: an unprinted form never reaches the parser, so
refusing it only grows the grammar, and in a lexicalist grammar such constraints
have no natural home. A word flag is fitted to the cards seen so far, says
nothing about English, and breaks on the next card. Where an attachment seems to
need one, some declaration is too broad, such as a preposition's Host
Categories, and the fix is to correct that declaration.

## 4. Admission and ambiguity

**The charter.** The parser admits all English on all Magic cards. It is not a
linter that checks whether Oracle's templating conventions were followed. A
grammatical sentence a card could print is admitted, even where Oracle would
template it differently. The grammar refuses a grammatical form only where a
gate measurably prevents spurious Readings or forest growth on real card text
(§6). The style guide describes
what Oracle writes: it is evidence about the register, never a rule a Reading
must pass. So a template difference is never grounds for correcting a card's
text; an Editorial Correction
([`corrections.txt`](../../crates/english/corrections.txt)) is only for an
error in the text. A Reading that is grammatical but wrong for the rules is not the
grammar's to reject either. The layers above resolve it by meaning
([Architecture](architecture.md), "Spelling is a construction grammar"). Why:
a grammar that enforced templating would refuse any card, custom content
included, whose wording departs from current templates, and it would decide
by convention what only meaning can decide.

**Keep it packed.** When a phrase can attach at more than one site, every
grammatical attachment stays in the Packed Forest, and nothing in this layer
chooses among them. Admission and validation run over the forest. No consumer
enumerates every tree, because a chain of unselected Preposition Phrases has
Catalan-many attachments. Spelling recovery reads the forest and never
re-parses the text.

**Structure over suppression.** A missing or wrong analysis is fixed by a
general rule that cites CGEL, the style guide or the CR. The rule adds whatever
features, Categories, Productions or engine capability it needs. It is never an
exclusion aimed at one card. A spurious analysis, which no English grammar
admits, is removed by tightening or deleting the rule that admits it. A second
tree of the same analysis is a different defect, which canonicalization removes
(§8).

## 5. The Ranking Key

The Ranking Key orders the forest for presentation and for bounded best-*k*
traversal, and that is all it does. It never decides Admission and never
removes a Reading, and no Reading is right or wrong because of where it ranks.
A Judgment asserts grammaticality, never a rank. Where the endorsed Reading
ranks is reported but never gated, and no ranking work is done to move a
Reading up. Why: ranking that decided anything would be disambiguation by rank,
and the architecture resolves ambiguity by meaning alone
([Architecture](architecture.md), "Spelling is a construction grammar").

The key is purely structural. Its components (glossary, "Ranking Key") are
keyed on host kind, grammatical function, span and Production, never on what a
word means. No component is declared on a lexeme, frame or Catalog entry. Host
preference comes from Attachment Class and Right Association, and no Production
Cost prefers a Nominal host to a Predicate one. Why: a preference for each word's plausible
dependents (*cast* takes an origin, *spend* a purpose) is policing (§3) that
re-enters the grammar through the lexicon, and plausibility is meaning, which
belongs above.

Every component is a sum over the Reading's attachments or Productions, and
lexicographic order is preserved under componentwise addition. Why: the
cheapest *k* Readings can then be extracted by dynamic programming over the
Packed Forest, without building trees.

## 6. Admission gates

An admission gate is an `Exclude` or `Require` equation, or a Category that
restates some Productions in order to omit others. A gate earns its place by
what it prevents on real card text: spurious Readings and forest growth. That
effect is measured by ablation, removing the gate and comparing the census
([`crates/english/README.md`](../../crates/english/README.md), "Working
rules"). That English never builds a form is not by itself a justification,
and the style guide is evidence about the register, never a gate's
justification. A form that is only dispreferred is ranked (§5), not gated.
Why: a gate with no effect on real cards refuses only forms no card prints,
which only grows the grammar (§3), while a gate whose removal multiplies
Readings on real cards keeps the forest one the layers above can work over.

Adding or retiring a gate follows this section and needs no ruling.

## 7. The parser contract

**Lexical analysis is independent of the grammar.** Text is analyzed without
production-directed scanning. Every alternative names a declared Lexeme with
its lexical Category, Word Form, features and Provenance, and an unknown word
is reported as a vocabulary gap, never guessed. Analysis and realization share
the same inflection rules. Why: the parser then sees every lexical
alternative, so no choice is hidden in a scanner, and a word is spelled the
way it is read.

**Admission works on finite summaries.** Chart completion compares
grammatical summaries, never products of full child values, and a full tree is
built only when a Reading is requested from the forest. Packing preserves
every distinction that can affect a parent's admissibility and keeps
correlated alternatives correlated, for incomplete Derivations as well as
complete Constituents. A deferred validity check is part of Admission: an
unchecked path through the forest is not a Reading. Why: products of child
values grow with the number of trees, which is exponential, while the forest
stays polynomial.

**Both round-trip laws hold**, under the same parse context and Lexical
Environment:

```
v is an admitted Reading of s                      =>  render(v) = s
v is independently constructed and grammatical     =>  v is a Reading of parse(render(v))
```

Equality in the second law preserves structure, lexical identities and
features, which fix each word's spelling. Computed source positions are not identity,
and no echo of the source buffer or opaque catch-all leaf counts as
realization. Why: these laws carry the system's exact round trip at this layer
([Architecture](architecture.md), "Exact round trip").

**A Reading stores no punctuation.** Punctuation may decide an analysis, and
realization derives it from structure. Why: each Production fixes its own
punctuation, and where the grammar admits two punctuation forms they are
distinct Productions, so the tree already says which was written. That works
because Oracle's punctuation is uniform, and the Editorial Corrections
([`corrections.txt`](../../crates/english/corrections.txt)) fix its exceptions.

## 8. Readings

A Reading is one whole tree from the Packed Forest, and two Readings are the
same only if they are the same tree. More than one Reading is successful
parsing, not failure (§4).

**One tree per analysis.** The grammar does not build two trees for the same
grammatical analysis. Where two attachment sites or groupings differ in no
meaning, one is canonical, and the canonical site is built structurally in the
chart, never merged after parsing. Each canonicalization is reviewed and
documented where `sylex_syntax` or the grammar data builds it. Why: equivalent sites multiply, so *k*
trailing dependents with two equivalent sites each make 2^*k* trees, and
merging after parsing would mean building them first.

**Wrong analyses.** A Corpus Unit is read when a correct analysis is among its
Readings. A grammatical Reading beside the intended one is tolerated, and
meaning disambiguates it above (§4). An ungrammatical Reading is not: a newly
admitted one is a defect, never a gain. A unit whose Readings are all known to
be wrong is not covered. Why: coverage counts what the layers above can use, and they can use a
unit only if its forest holds the right analysis.
