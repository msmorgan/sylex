//! The authored shape of every lexicon and catalog data file.
//!
//! Entries state only what the defaults do not: regular inflections come from
//! `sylex_lexicon`'s morphology, prepositions are named by lemma, and a
//! Frame names its Frame Family and the prepositions it selects.

use std::collections::BTreeMap;

use serde::Deserialize;
use sylex_lexicon::Binding;
use sylex_lexicon::Case;
use sylex_lexicon::Countability;
use sylex_lexicon::GenitiveAlternant;
use sylex_lexicon::Hyphenation;
use sylex_lexicon::Number;
use sylex_lexicon::Onset;
use sylex_lexicon::Person;
use sylex_lexicon::WordForm;
use sylex_syntax::frames::Hosts;

/// A rule of the Comprehensive Rules, written as a citation: `[CR#701.8]`.
/// The citation tool checks it like any other.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
#[serde(try_from = "String")]
pub struct Rule(String);

impl Rule {
    /// The rule number alone, without the citation brackets.
    #[must_use]
    pub fn number(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Rule {
    type Error = String;

    fn try_from(citation: String) -> Result<Self, String> {
        citation
            .strip_prefix("[CR#")
            .and_then(|rest| rest.strip_suffix(']'))
            .filter(|number| {
                !number.is_empty()
                    && number
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '.')
            })
            .map(|number| Self(number.to_owned()))
            .ok_or_else(|| {
                format!("`{citation}` is not one rule written as a bracketed CR citation")
            })
    }
}

/// What fills a Frame Slot, with prepositions named by lemma.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum FillSpec {
    NP,
    PP(Vec<String>),
    /// A Preposition Phrase headed by one of the prepositions with a
    /// Complement of this kind only (*do the same for [Vampire]*, a cited
    /// word: `PPWith(["for"], Mention)`).
    PPWith(Vec<String>, Box<FillSpec>),
    Pred,
    /// A card's name, in a position where another card's name stands: the
    /// predicative complement of *name* (*cards named Relentless Rats*), the
    /// complement of *among* after *from* (*from among Disenchant, ..., and
    /// Black Lotus*), or the complement of *into* after *meld* (*meld them
    /// into Brisela, Voice of Nightmares*).
    Name,
    AdjP,
    /// An Adverb Phrase or a Noun Phrase of frequency (*than twice*).
    AdvP,
    LocativeNP,
    /// An ordinal position from an end of a pile (*put that card third from
    /// the top*).
    Position,
    /// A slash measure as a Noun Phrase (*gets +2/+2*; *your choice of 5/1
    /// or 1/5*). A bare slash measure occurs only where a head selects one,
    /// so *get* and *of* are its only holders (project term Measure Noun
    /// Phrase).
    MeasureNP,
    FiniteClause,
    /// A temporal preposition's finite Clause, which locates its host's
    /// situation at its own time
    /// (`sylex_syntax::frames::ClauseKind::Temporal`).
    TemporalClause,
    /// A comparative clause, the Complement of *than* or equative *as*
    /// (`sylex_syntax::frames::ClauseKind::Comparative`).
    ComparativeClause,
    /// An expandable declarative content clause (CGEL, Ch. 11 §4.8, p. 971).
    ExpandableClause,
    /// A Clause set off by a dash, the content of the Object before it.
    SetOffClause,
    VerblessClause,
    /// A non-finite or verbless clause with its Subject (*with your hand
    /// revealed*; CGEL, Ch. 14 §10(a), p. 1267).
    AbsoluteClause,
    ToInf,
    /// A *to*-infinitival with an object Gap the Subject fills.
    HollowToInf,
    BareInf,
    Gerund,
    PastPart,
    KeywordPhrase,
    /// A quoted granted ability (*a token with "This token can't block."*;
    /// [CR#113.1a]). Its one preposition holder is *with*, beside the verbs
    /// *has* and *gain*.
    QuotedAbility,
    /// A word cited rather than used (*choose left or right*, *other than
    /// Wall*): a metalinguistic Noun Phrase (CGEL, Ch. 5 §8.2, p. 401), only
    /// where a head's frame selects one.
    Mention,
    Nominal,
    Amount,
    Cost,
    /// No Complement: only an intransitive preposition (*out* in *phase out*).
    Intransitive,
}

/// A catenative verb's non-finite Complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Catenation {
    ToInf,
    BareInf,
    Gerund,
    PastPart,
}

/// A Subcategorization Frame as authored, its slots in their active-voice
/// order.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum FrameSpec {
    Intransitive,
    Monotransitive,
    Transitive(FillSpec),
    Ditransitive,
    TransitiveWithPP(Vec<String>),
    TransitiveWithComplement(FillSpec),
    Prepositional(Vec<String>),
    Copular,
    Catenative(Catenation),
    /// An object and a predicative complement of the given kind: `Pred` in
    /// *turn it face up*, `Name` in *cards named Relentless Rats*.
    ComplexTransitive(FillSpec),
    /// An object and a non-finite complement: *cause you to sacrifice*.
    ComplexCatenative(Catenation),
    /// A content clause: *decide whether to flip again*.
    ContentClause,
    Amount,
    TransitiveAmount,
    Cost,
    /// The one Complement of a noun, adjective or determinative.
    Complement(FillSpec),
    /// A verb with a fixed preposition among its Complements: *come up heads*.
    Idiom(Vec<IdiomPart>),
    /// A verb's frame restricted to active voice, deriving no passive (CGEL,
    /// Ch. 16 §10.1.2, p. 1432, [17]): *costs {1} less*, *has flying*.
    ActiveOnly(Box<FrameSpec>),
    /// A verb's frame restricted to passive voice, with no active (CGEL,
    /// Ch. 16 §10.1.2, p. 1435, [31]): *this creature is dealt damage*.
    PassiveOnly(Box<FrameSpec>),
}

/// One Complement of an [`FrameSpec::Idiom`], in order.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum IdiomPart {
    /// The fixed preposition, heading its phrase alone.
    Prep(String),
    Pred,
}

/// A replacement for the regular spelling of some inflection slots of a verb.
/// Unnamed person and number match every slot of the form.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormOverride {
    pub form: WordForm,
    #[serde(default)]
    pub person: Option<Person>,
    #[serde(default)]
    pub number: Option<Number>,
    pub surface: String,
}

/// A verb with its full paradigm and active Subcategorization Frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerbEntry {
    pub lemma: String,
    /// The third-person singular present.
    #[serde(default)]
    pub third_singular: Option<String>,
    /// Both the preterite and the past participle.
    #[serde(default)]
    pub past: Option<String>,
    #[serde(default)]
    pub preterite: Option<String>,
    #[serde(default)]
    pub past_participle: Option<String>,
    #[serde(default)]
    pub gerund: Option<String>,
    /// The singular irrealis (*were*), for the one verb that has it.
    #[serde(default)]
    pub irrealis: Option<String>,
    /// Person- or number-specific replacements, applied after the fields above.
    #[serde(default)]
    pub forms: Vec<FormOverride>,
    /// A defective verb with only finite forms (the modals and the negative
    /// auxiliaries): no plain, gerund-participle or past-participle slot, and
    /// a preterite only when one is spelled.
    #[serde(default)]
    pub finite_only: bool,
    /// A negative form of an auxiliary (*can't*, *doesn't*).
    #[serde(default)]
    pub negative: bool,
    /// A modal auxiliary (*can*, *may*, *must*, *will* and their negative
    /// forms). Its preterite is ambiguous between past time and modal
    /// remoteness (CGEL, Ch. 3 §2.4, p. 107), so it fixes no time and has no
    /// tense (`sylex_lexicon::LexicalProperties::modal`).
    #[serde(default)]
    pub modal: bool,
    /// The verb whose forms these are: a contracted auxiliary (CGEL, Ch. 18
    /// §6.2, pp. 1614–1616) or a negative inflectional form (CGEL, Ch. 9 §2.1,
    /// p. 799) is named by that verb's lemma (*'re* and *aren't* are forms of
    /// *be*), while keeping its own Lexeme id and spellings.
    #[serde(default)]
    pub form_of: Option<String>,
    /// A contracted auxiliary bound to its host (*'ve* in *you've*).
    #[serde(default)]
    pub binding: Option<Binding>,
    /// Every verb has a frame, except an auxiliary the grammar names as a
    /// Closed Word or Lexical Class member (perfect *have*, *hasn't*, *'ve*).
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
    /// The Lexical Classes of the grammar it is a member of
    /// (`NamingVerb`: its Object is an object it brings into being, which
    /// the text may name, [CR#111.4]).
    #[serde(default)]
    pub classes: Vec<String>,
}

/// Noun subclasses with distinct adjunct distributions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum NounKind {
    #[default]
    Ordinary,
    Temporal,
    /// A Manner Adjunct without a governing preposition (*do it this way*;
    /// CGEL, Ch. 8 §2.1, p. 671). It has one holder, *way*, and that is the
    /// whole class; `MannerAdjunct` reads it.
    Manner,
}

/// A noun with its number paradigm and optional Complement frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NounEntry {
    #[serde(default)]
    pub distribution: NounDistribution,
    #[serde(default)]
    pub kind: NounKind,
    /// Ordered notation parameters, for indexed headings.
    #[serde(default)]
    pub parameters: Vec<Vec<sylex_syntax::frames::ParameterPart>>,
    pub lemma: String,
    #[serde(default = "count")]
    pub countability: Vec<Countability>,
    /// The non-count nominal stands bare as a Noun Phrase only with a
    /// Complement or Modifier (*power 3*, *mana value less than X*, *control
    /// of target creature*, *turn order*), never as an unmodified single word
    /// (*bidding*, *devotion*, *rest*, *text*, *loyalty*, *power*,
    /// *toughness*, *mana value*, *mana cost*, *control*, *order*, *worth*).
    #[serde(default)]
    pub bare_mass_modified: bool,
    #[serde(default)]
    pub plural: Option<String>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
    /// How a prefix joins the noun, where it is not solid, the default
    /// (*nonland*): hyphenated (*non-outlaw*). A subtype takes it hyphenated
    /// without declaring it (*non-Vampire*).
    #[serde(default)]
    pub prefix_hyphenation: Option<Hyphenation>,
    /// Its Onset, where its first letter's is not it (*an hour*, *a
    /// Unicorn*).
    #[serde(default)]
    pub onset: Option<sylex_lexicon::Onset>,
    /// The Lexical Classes of the grammar it is a member of (`TokenNoun`,
    /// `NameNoun`).
    #[serde(default)]
    pub classes: Vec<String>,
}

fn count() -> Vec<Countability> {
    vec![Countability::Count]
}

/// A noun written as its lemma alone when every default applies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum NounDecl {
    Lemma(String),
    Entry(NounEntry),
}

impl NounDecl {
    #[must_use]
    pub fn entry(&self) -> NounEntry {
        match self {
            Self::Lemma(lemma) => NounEntry {
                distribution: NounDistribution::Ordinary,
                kind: NounKind::Ordinary,
                parameters: Vec::new(),
                lemma: lemma.clone(),
                countability: count(),
                bare_mass_modified: false,
                plural: None,
                frames: Vec::new(),
                prefix_hyphenation: None,
                onset: None,
                classes: Vec::new(),
            },
            Self::Entry(entry) => entry.clone(),
        }
    }
}

/// An adjective. Its inflected grades, when it has them, are declared beside
/// it and become adjectives of their own `Grade`.
///
/// An adjective not restricted to attributive use and spelled like a verb's
/// past participle (*tapped*, *attached*) is that verb's Adjectival Passive:
/// the loader marks the verb and the adjective `AdjectivalPassive: Lexical`,
/// and the grammar does not derive the adjective from the participle again,
/// nor postmodifies with the adjective where the participle does (*Aura
/// attached to a creature*). An action's
/// adjective (*transformed*) is declared on its `KeywordAction`, a status's on
/// its `Status`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjectiveEntry {
    #[serde(default)]
    pub position: Option<AdjectivePosition>,
    /// An attributive adjective whose nominal stands only under an overt
    /// determiner (*the only creature card*): it carries the grammar's
    /// `DeterminerNeed`.
    #[serde(default)]
    pub needs_determiner: bool,
    #[serde(default)]
    pub fused_head: Option<FusedHeadDetermination>,
    /// The adjective follows a specific sum (*fifty dollars net*; CGEL, Ch. 5
    /// §14.2(b), pp. 445-446): it is in the grammar's `SumAdjective` class.
    #[serde(default)]
    pub follows_sum: bool,
    /// The adjective names a Quality as a fused head, with no noun of its own
    /// (*protection from multicolored*; CGEL, Ch. 5 §9.3, p. 417): it is in
    /// the grammar's `QualityHead` feature. A color word is not: its noun
    /// is the Quality (*protection from red*; Ch. 19 §3.3, p. 1642).
    #[serde(default)]
    pub fused_quality: bool,
    pub lemma: String,
    #[serde(default)]
    pub comparative: Option<String>,
    #[serde(default)]
    pub superlative: Option<String>,
    /// The grade of a lemma that is itself comparative (*less*, *lesser*).
    #[serde(default)]
    pub grade: Option<AdjectiveGrade>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

/// An adjective's grade, as the grammar's `Grade` feature: inflectional, the
/// equality of *same*, which licenses an *as* Complement, or the non-scalar
/// inequality of *different* and *other*, which license a *than* Complement
/// (CGEL, Ch. 13 §1.1, p. 1100).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AdjectiveGrade {
    Comparative,
    Superlative,
    Equative,
    NonScalarInequality,
}

/// The determiner in a fused modifier-head NP: ordinary determination
/// (*each other*) or the definite article (*the same*; CGEL, Ch. 5 §9.3,
/// p. 416; Ch. 17, p. 1545).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum FusedHeadDetermination {
    Ordinary,
    DefiniteArticle,
}

/// The function to which an adjective is restricted (CGEL Ch. 6 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AdjectivePosition {
    Attributive,
    Predicative,
}

/// An adjective written as its lemma alone when it has no grades or frames.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum AdjectiveDecl {
    Lemma(String),
    Entry(AdjectiveEntry),
}

impl AdjectiveDecl {
    #[must_use]
    pub fn entry(&self) -> AdjectiveEntry {
        match self {
            Self::Lemma(lemma) => AdjectiveEntry {
                position: None,
                needs_determiner: false,
                fused_head: None,
                follows_sum: false,
                fused_quality: false,
                lemma: lemma.clone(),
                comparative: None,
                grade: None,
                superlative: None,
                frames: Vec::new(),
            },
            Self::Entry(entry) => entry.clone(),
        }
    }
}

/// Where in a clause an adverb occurs (CGEL Ch. 8 §§17–21).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum AdverbPosition {
    /// Before the subject.
    Front,
    /// Between the subject and the verb, or after the first auxiliary.
    Central,
    /// After the verb and its Complements.
    End,
    /// Inside a phrase, focusing it (*only one target*, *just beneath*)
    /// rather than modifying a clause.
    Phrasal,
    /// Multiplying a quantity (*twice the damage*). The grammar reads such an
    /// adverb as a clause's frequency `End` adverb too (*draw a card twice*),
    /// so it declares neither `End` nor `Phrasal`.
    Multiplying,
    /// Bounding a quantity: its determinative or numeral (*at least two*,
    /// *exactly one*; CGEL p. 1129).
    Bounding,
    /// Initial in a non-initial Conjunct, the sole marker of its
    /// Coordination (*Draw a card, then discard a card*; CGEL, Ch. 15 §2.10,
    /// pp. 1319–1320).
    Linking,
    /// The head of an idiomatic coordinate, after its coordinator, which
    /// stands for further conjuncts of the coordination's kind (*and so
    /// on*, "and other things of this kind"; CGEL, Ch. 17 §7.7.3(e),
    /// p. 1540).
    Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdverbEntry {
    pub lemma: String,
    pub positions: Vec<AdverbPosition>,
}

/// One complement frame of a preposition with its Host Category.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepositionFrameSpec {
    #[serde(default = "complement_relation")]
    pub relation: sylex_syntax::frames::SlotRelation,
    pub complement: FillSpec,
    pub hosts: Hosts,
    /// The grammar's Complement Constraints the frame names
    /// (`sylex_syntax::frames::PrepositionFrame::constraints`).
    #[serde(default)]
    pub constraints: std::collections::BTreeSet<dpsi::Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepositionEntry {
    pub lemma: String,
    pub frames: Vec<PrepositionFrameSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeterminativeEntry {
    #[serde(default)]
    pub noun_kind: Option<NounKind>,
    #[serde(default)]
    pub distribution: DeterminativeDistribution,
    #[serde(default)]
    pub countability: Option<sylex_lexicon::Countability>,
    #[serde(default)]
    pub number: Option<Number>,
    pub lemma: String,
    /// Distinguishes two determinatives of one lemma (*more*, the
    /// comparative of both *many* and *much*), naming the lexeme
    /// `det:LEMMA.SENSE`.
    #[serde(default)]
    pub sense: Option<String>,
    /// The forms the Onset of the word after it selects (*a*, *an*), when
    /// the lemma is not the only spelling.
    #[serde(default)]
    pub allomorphs: Option<BTreeMap<Onset, String>>,
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
}

/// The distribution of a determinative, including articles and distributive
/// quantifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum DeterminativeDistribution {
    #[default]
    Dependent,
    Independent,
    /// The indefinite article *a*, which selects a singular head (CGEL, Ch. 5
    /// §3.4, p. 353, set (a) [67]). It has one holder, *a*, and that is the
    /// whole class: never a bare quantified plural (*\*a three creatures*).
    /// `TemporalLocation` refuses it, an admission gate
    /// (`docs/design/english.md` §6); CGEL's examples of indefinite temporal
    /// location Noun Phrases take *some*, *one* or *another* (Ch. 8 §6.3,
    /// p. 698).
    IndefiniteArticle,
    Distributive,
    /// Deictic *this*, *that* and *last*, which with a period noun form
    /// a Noun Phrase Adjunct of temporal location (*this turn*; CGEL, Ch. 8
    /// §6.3, p. 698).
    Deictic,
    /// Selects a singular head or a quantified plural (*another three
    /// bodies*; CGEL, Ch. 5 §3.4, p. 353, set (c) [69–70]). It has one holder,
    /// *another*, and that is the whole class: the other members are
    /// sequences, *a* with *further* or *additional*, whose quantified plural
    /// the grammar reads with the modifier. Unlike *a*, it also determines a
    /// temporal location Noun Phrase (*another day*; Ch. 8 §6.3, p. 698).
    SingularOrQuantified,
    /// Free-choice *any* (CGEL, Ch. 5 §7.5, p. 381): with a quantificational
    /// noun the Noun Phrase takes the Number of its oblique (*any number of
    /// target creatures become ...*). It has one holder, *any*, and that is
    /// the whole class, as `NounDistribution::Serial` is *time*: the value
    /// routes *any* to the Productions that carry number transparency (CGEL,
    /// Ch. 5 §3.3, pp. 349–351), which keeps out *any number of target
    /// creatures becomes*, and keeps it out of the general ones, which would
    /// build a second tree of the same analysis
    /// (`docs/design/english.md` §8).
    FreeChoice,
    /// A fraction or multiplier before a determined Noun Phrase (*half their
    /// life*, *double that damage*; CGEL, Ch. 5 §12, p. 434).
    Multiplier,
    /// The Targeting Marker, a determiner on its own or the last word of
    /// one (*target creature*, *two target creatures*). It has one holder,
    /// *target*, and that is the whole class: a determinative that only
    /// determines a nominal, so it is kept apart from `Dependent`, which the
    /// fused and partitive Productions require (not *choose target*, *target
    /// of those*).
    Targeting,
    /// A degree determinative, *many* or *much*, which a degree word
    /// modifies (*that many*, *as much*, *how many*).
    Degree,
}

/// One cell of a pronoun paradigm.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PronounForm {
    #[serde(default = "invariant")]
    pub form: WordForm,
    pub person: Person,
    pub number: Number,
    #[serde(default)]
    pub case: Option<Case>,
    pub surface: String,
}

fn invariant() -> WordForm {
    WordForm::Invariant
}

/// A pronoun Lexeme: a personal, reflexive, relative or interrogative
/// pronoun, with the features Oracle gives it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PronounEntry {
    #[serde(default)]
    pub kind: PronounKind,
    pub lemma: String,
    pub forms: Vec<PronounForm>,
}

/// Personal, existential and relative pronouns have different clause
/// distributions: a relative pronoun introduces a relative clause.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum PronounKind {
    #[default]
    Personal,
    /// Dummy *there*, the subject of an existential clause (CGEL, Ch. 16
    /// §6.2, p. 1394). It has one holder, and that is the whole class: the
    /// Existential Production reads it and the Pronoun Production excludes it.
    Existential,
    Relative,
}

/// An affix or clitic, bound to its host, with its forms: each is the one
/// a value of its Adjacency Agreement dimension `T` selects.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundEntry<T: Ord> {
    /// The identity-bearing name, such as `genitive`.
    pub name: String,
    pub allomorphs: BTreeMap<T, String>,
    pub binding: Binding,
}

/// `lexicon/english/closed_class.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedClass {
    pub determinatives: Vec<DeterminativeEntry>,
    pub pronouns: Vec<PronounEntry>,
    /// Closed-class adverbs: interrogative *how* and degree *as*.
    pub adverbs: Vec<AdverbEntry>,
    pub coordinators: Vec<String>,
    pub subordinators: Vec<String>,
    /// Auxiliaries and modals, including their negative forms.
    pub auxiliaries: Vec<VerbEntry>,
    /// Prefixes, whose form the host after them selects.
    pub affixes: Vec<BoundEntry<Hyphenation>>,
    /// Suffixes, whose form the host before them selects.
    pub clitics: Vec<BoundEntry<GenitiveAlternant>>,
    /// Braced and other notation symbols, spelled exactly.
    pub symbols: Vec<SymbolEntry>,
}

/// `catalogs/types.ron` [CR#205.2a,205.4a].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Types {
    pub card_types: Vec<NounDecl>,
    pub supertypes: Vec<String>,
}

/// One subtype family of [CR#205.3].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum SubtypeFamily {
    Artifact,
    Battle,
    Creature,
    Dungeon,
    Enchantment,
    Land,
    Planeswalker,
    Spell,
}

/// `catalogs/subtypes.ron` [CR#205.3].
pub type Subtypes = BTreeMap<SubtypeFamily, Vec<NounDecl>>;

/// A verb a keyword catalog entry uses: its own [`VerbEntry`], or the lemma of
/// a shared Catalog verb declared in `catalogs/verbs.ron`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VerbDecl {
    Shared(String),
    Entry(Box<VerbEntry>),
}

/// A CR-enumerated catalog item realized by one or more Lexemes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordAction {
    /// The rule that defines it, such as `[CR#701.8]`.
    pub rule: Rule,
    /// The CR's name for it, such as `Destroy`.
    pub name: String,
    #[serde(default)]
    pub verbs: Vec<VerbDecl>,
    /// Nouns that realize it: the instruction *Monstrosity 3* [CR#701.37a],
    /// or *the Ring* that tempts you [CR#701.54a].
    #[serde(default)]
    pub nouns: Vec<NounDecl>,
    /// Explicit adjective Lexemes associated with the action.
    #[serde(default)]
    pub adjectives: Vec<AdjectiveDecl>,
}

/// One parameter a keyword ability takes on a keyword line.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum KeywordParameter {
    /// A number or `X`: *bushido 2*.
    Amount,
    /// A cost: mana-only after a space (*equip {2}*); when final, a cost
    /// containing words follows an em dash and ends in a period (*ward—Pay 3
    /// life.*). A final Cost lowers to one Production per spelling, carrying
    /// `FinalParameter: Cost` or `FinalParameter: SetOffCost`.
    Cost,
    /// A mana cost after an em dash: *suspend 4—{1}{U}*. When final, it
    /// carries `FinalParameter: SetOffCost`.
    DashCost,
    /// A condition after an em dash: *companion—…*.
    DashCondition,
    /// A Keyword Quality, after the preposition named, if any:
    /// *protection from red*, *enchant creature*.
    Quality(Option<String>),
    /// A Keyword Quality written before the keyword: *Goblin offering*.
    QualityBefore,
    /// A card name after the preposition named, if any: *partner with …*.
    Name(Option<String>),
    /// An ability after an em dash, which the keyword labels: *boast — {1}: …*.
    /// When final, it carries `FinalParameter: SetOffAbility`.
    DashAbility,
    /// Power and toughness after an em dash: *prototype {1}{B} — 1/1*.
    DashPowerToughness,
    /// Another keyword after an em dash: *modular—sunburst*.
    DashKeyword,
}

/// The noun stems a Bound Keyword Surface binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum StemClass {
    LandType,
    CreatureType,
    /// Any subtype.
    Subtype,
    CardType,
    Supertype,
}

/// A Bound Keyword Surface and the stems its Keyword Quality draws on.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundSurface {
    pub surface: String,
    pub stems: Vec<StemClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordAbility {
    /// Determinative Lexemes explicitly associated with this catalog item.
    #[serde(default)]
    pub determinatives: Vec<DeterminativeEntry>,
    pub rule: Rule,
    pub name: String,
    pub lemma: String,
    /// Alternative parameter lists; empty means the keyword stands alone.
    #[serde(default)]
    pub parameters: Vec<Vec<KeywordParameter>>,
    #[serde(default)]
    pub bound: Option<BoundSurface>,
    /// Whether a face with this keyword has its text changed by removing all
    /// text found within square brackets, as a cleave spell cast for its
    /// cleave cost does [CR#702.148a]; the brackets mark that text.
    #[serde(default)]
    pub bracketed_text_removable: bool,
    /// Keywords the rule defines as variants, each its own Keyword Lexeme:
    /// *megamorph* [CR#702.37b], *partner—Friends forever* [CR#702.124i].
    #[serde(default)]
    pub variants: Vec<KeywordVariant>,
    /// Verbs the rule defines for acting with the keyword (*cycle*, *crew*),
    /// or shared Catalog verbs from `catalogs/verbs.ron` (*beam*).
    #[serde(default)]
    pub verbs: Vec<VerbDecl>,
    /// Nouns the rule defines for the ability or its cost: *its {W} kicker*
    /// [CR#702.33f], *the gift was promised* [CR#702.174k].
    #[serde(default)]
    pub nouns: Vec<NounDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordVariant {
    pub lemma: String,
    #[serde(default)]
    pub parameters: Vec<Vec<KeywordParameter>>,
}

/// Where an ability word is listed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AbilityWordListing {
    /// In [CR#207.2c].
    Rules,
    /// Printed on Vintage-legal cards but omitted by the CR list.
    Printed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbilityWord {
    pub lemma: String,
    pub listed: AbilityWordListing,
}

/// A named kind of counter; its Lexeme is the compound noun `<kind> counter`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterKind {
    pub kind: String,
    /// The rule that names it, when the CR does.
    #[serde(default)]
    pub rule: Option<Rule>,
    /// Its Onset, where its first letter's is not it (*an hour counter*).
    #[serde(default)]
    pub onset: Option<sylex_lexicon::Onset>,
}

/// A designation and the nouns and adjectives that name it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Designation {
    pub rule: Rule,
    pub name: String,
    #[serde(default)]
    pub nouns: Vec<NounDecl>,
    #[serde(default)]
    pub adjectives: Vec<String>,
}

/// A phase or step [CR#500.1].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TurnPart {
    pub rule: Rule,
    pub noun: NounDecl,
}

/// A predefined token [CR#111.10]: the subtype that names it and any token
/// names it introduces.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub rule: Rule,
    pub name: String,
    pub subtype: String,
    #[serde(default)]
    pub names: Vec<String>,
}

/// A name in scope only on the cards that print it: a vote option
/// (`catalogs/vote_options.ron`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedName {
    pub name: String,
    pub cards: Vec<String>,
}

/// An Anchor Word [CR#614.12c] with the cards that print it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnchorWord {
    pub word: String,
    pub cards: Vec<String>,
}

/// A notation symbol with catalog-declared lexical classes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolEntry {
    pub surface: String,
    pub classes: Vec<String>,
    /// Its Onset, where its first letter's is not it: a letter is spoken by
    /// its name (*an X/X*).
    #[serde(default)]
    pub onset: Option<sylex_lexicon::Onset>,
}

fn complement_relation() -> sylex_syntax::frames::SlotRelation {
    sylex_syntax::frames::SlotRelation::Complement
}

/// Lexical realizations of the permanent status values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub rule: Rule,
    #[serde(default)]
    pub adjectives: Vec<AdjectiveDecl>,
    #[serde(default)]
    pub prepositions: Vec<PrepositionEntry>,
}

/// Lexically declared nominal distributions in the Oracle register.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum NounDistribution {
    #[default]
    Ordinary,
    /// A number-transparent quantificational noun (*any number of*; CGEL,
    /// Ch. 5 §3.3, pp. 349–351). It has one holder, *number*, and that is the
    /// whole class: the Productions that carry number transparency read it.
    Quantity,
    /// A mass noun accepting a direct cardinal measure without a unit noun.
    DirectMeasure,
    /// Used as a nominal head, with no attributive noun use. A non-count one
    /// is not a bare Predicative either: those uses are its adjective's
    /// (*His tie is brown*; CGEL, Ch. 19 §3.3, p. 1642). `noun:target` is
    /// one: its determiner use is the Targeting Marker, `det:target`.
    HeadOnly,
    /// Serial *time*, counting occasions: a Noun Phrase Adjunct with any
    /// determinative (*any time*, *an additional time*, *three times*),
    /// unlike a period noun (CGEL, Ch. 8 §6.3, p. 698; §9, pp. 713–716), and
    /// a multiplier (*three times X*; Ch. 5 §12, p. 434). It is the only
    /// such noun: *times* heads every bounding frequency Noun Phrase and
    /// cardinal multiplier (Ch. 8 §9, p. 715).
    Serial,
}
