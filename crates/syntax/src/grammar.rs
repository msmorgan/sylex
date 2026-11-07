//! The grammar data file: Schemas and Productions as authored, before binding.

use std::collections::BTreeMap;

use dpsi::Ident;
use serde::Deserialize;

use crate::LoadError;
use crate::frames::SlotKind;

/// An authored grammar data file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grammar {
    /// The Category a whole input is parsed as.
    pub root: Ident,
    /// Finite feature domains by feature name. The Gap feature is built in.
    pub features: BTreeMap<Ident, Vec<Ident>>,
    pub categories: BTreeMap<Ident, CategoryDecl>,
    /// Closed Words by grammar name, each naming one core closed-class lexeme
    /// id.
    pub closed_words: BTreeMap<Ident, Ident>,
    /// Lexical Classes by grammar name, each listing core closed-class lexeme
    /// ids.
    pub lexical_classes: BTreeMap<Ident, Vec<Ident>>,
    /// Classes whose membership is declared by the lexical environment.
    pub typed_classes: Vec<Ident>,
    /// The Category that fills each kind of Frame Slot.
    pub slot_categories: BTreeMap<SlotKind, Ident>,
    /// Equations on every Frame Slot child of that kind in a derived valence
    /// phrase, over the child `slot`: what fills the slot as a Complement
    /// (a Noun Phrase Complement is never nominative).
    pub slot_equations: BTreeMap<SlotKind, Vec<Equation>>,
    /// `(Category, feature, value)`: a dependent of that Category carrying
    /// that value attaches high by its Attachment Class, as a comma-set
    /// Supplement does (`docs/decisions/english.md` §5): it counts its
    /// distance from its host's head reversed.
    pub high_attachment: Vec<(Ident, Ident, Ident)>,
    pub productions: Vec<ProductionDecl>,
    pub schemas: Vec<SchemaDecl>,
    /// Correlative pairs: a marker (a Closed Word or Lexical Class) that may
    /// precede the first conjunct of a coordination joined by the paired
    /// Closed Word (*both … and*, *either … or*, *neither … nor*).
    pub correlatives: Vec<(Ident, Ident)>,
    /// Idiomatic coordinates: a Closed Word coordinator and the Category of
    /// the pro-form it marks as the last coordinate of a coordination, which
    /// stands for further conjuncts of the coordination's kind (*and so on*,
    /// "and other things of this kind"; CGEL, Ch. 17 §7.7.3(e), p. 1540). It
    /// is no conjunct: it shares no feature with the overt conjuncts and
    /// takes no part in their parallelism (`compile.rs`, `idiomatic`).
    pub idiomatic_coordinates: Vec<(Ident, Ident)>,
}

impl Grammar {
    /// Parses `.fug` source (the language [`crate::fug`] describes).
    ///
    /// # Errors
    /// Malformed `.fug` source, naming the line and column.
    pub fn from_fug_str(text: &str) -> Result<Self, LoadError> {
        crate::fug::parse(text)
    }
}

/// A syntactic Category and the features its summary carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryDecl {
    /// The grammatical Category represented by this restricted production set.
    #[serde(default)]
    pub category: Option<Ident>,
    #[serde(default)]
    pub features: Vec<Ident>,
    /// Whether a Constituent of this Category may contain an unfilled Gap.
    #[serde(default)]
    pub gap: bool,
    /// The Gap kinds a Constituent of this gap-capable Category never
    /// contains, as a non-finite Complement never passes up a predicative
    /// Gap (*can't be __*).
    #[serde(default)]
    pub closed_gaps: Vec<Ident>,
    /// Whether this Category names a grammatical function, as `Predicative`
    /// names the predicative Complement: its head child bears that function
    /// in the same phrase too.
    #[serde(default)]
    pub function: bool,
}

/// One authored Production.
///
/// # Reserved child names (the contract)
///
/// A child's name is its grammatical function (`Production::names`), and the
/// engine keys behaviour on the names below. Any other name is inert. A
/// Production that names its semantic head something else forgoes every
/// `head` item, so renaming a head to dodge implicit agreement also loses its
/// head lexeme in Readings and its place in the head line: comment each such
/// rename in the grammar.
///
/// - `head`:
///   - implicit agreement with the mother on every feature both carry
///     (`compile.rs`, `add`; see `Equation`);
///   - Preposition and extension inheritance (`step.rs`, `resolve`);
///   - the head line (`step.rs`, `Production::head_line`), shared with `first`,
///     `last` and `rest`;
///   - the Reading head lexeme (`reading.rs`, `head_word`);
///   - the Linked Coordination it is headed by (`step.rs`, `Summary::link`);
///   - the host words counted for Right Association (`reading.rs`);
///   - wrapping by a function Category (`reading.rs`, `locate`);
///   - comma scope: a dependent set off before the head by a comma, which
///     contains no unclosed delimiting comma (`step.rs`,
///     `set_off_before_head`), and a comma-set post-dependent after a first
///     head (`step.rs`, `comma_scope_excludes`).
/// - `first`, `last`, `rest`:
///   - the head line;
///   - conjunct balance and the coordination Attachment Class (`reading.rs`);
///   - the fronted-scope rule, which `first` keys on only outside a serial tail
///     (`step.rs`, `comma_scope_excludes`, `Production::tail`).
/// - `adjunct`, `modifier`: unselected dependents, counted for Right
///   Association and the preposed-dependent Attachment Class (`reading.rs`).
///   `adjunct` also marks the post-verbal Adjunct host that an auxiliary Verb
///   Phrase never is (`step.rs`, `post_adjunct_host`).
/// - `supplement`: reversed Right Association (`reading.rs`).
/// - `object`, `shared`: the Reading functions of postposed Modifiers
///   (`object`) and of Right Node Raising (`shared`) (`reading.rs`, `locate`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecl {
    pub name: Ident,
    pub lhs: Ident,
    pub rhs: Vec<SymbolDecl>,
    #[serde(default)]
    pub equations: Vec<Equation>,
    #[serde(default = "default_cost")]
    pub cost: u32,
    /// Children that must span more than one lexical leaf.
    #[serde(default)]
    pub heavy: Vec<Ident>,
    /// Children that must be exactly one lexical leaf, alone or projected.
    #[serde(default)]
    pub light: Vec<Ident>,
    /// A child that does not end, through its head line, in a Preposition
    /// Phrase dependent that this Production, or a sibling Production with
    /// the same children up to and including it, selects as a `PP`
    /// Complement right after it: the phrase is that Complement, never the
    /// child's postmodifier (an existential's Extension after its Displaced
    /// Subject, CGEL Ch. 16 §6.2.1, p. 1394). A child that no such Complement
    /// follows is a load error.
    #[serde(default)]
    pub unextended: Vec<Ident>,
    /// Children that do not end, through their edge line, in a trailing
    /// Adjunct of one of these Categories: a dependent after the child that
    /// is coordinate with such an Adjunct is a basic coordination of the two,
    /// never also attached beside it (`docs/decisions/english.md` §8).
    #[serde(default)]
    pub unadjoined: Vec<(Ident, Vec<Ident>)>,
    /// Children through which a Gap may pass to the mother. A Gap passes
    /// up through no other child: an auxiliary marks its Verb Phrase
    /// Complement transparent, so extraction works across every verb form
    /// without a Production per form. `child.Gap = ..` fills the
    /// child's Gap within the mother, unless the child is also transparent:
    /// then the mother takes that Gap over (a purpose adjunct whose Object a
    /// relative clause binds). `$.Gap = ..` introduces one.
    #[serde(default)]
    pub transparent: Vec<Ident>,
    /// The phrase ends in an ellipsis site: its head omits every Complement
    /// it selects (a stranded auxiliary, CGEL Ch. 17 §7.1, p. 1519). The
    /// list, which may not be empty, names the word forms of the verb that
    /// heads the omitted Complement. Before any punctuation, a Constituent that
    /// ends in one is followed only by a coordinator or by the head it is a
    /// dependent of (a Subject's predicate), and only when no word spelled
    /// like that follower's lexical head has one of those forms: a phrase
    /// right after the site that could be the omitted Complement is that
    /// Complement, so no ellipsis is there (p. 1519), as with a bare noun's
    /// omitted Complement (`docs/decisions/english.md`
    /// §8).
    #[serde(default)]
    pub elliptical: Option<Vec<sylex_lexicon::WordForm>>,
    /// The phrase opens with a single dash that only the end of its
    /// sentence closes (CGEL Ch. 20 §5, p. 1751: the closing member of the
    /// pair is absorbed into the higher-level boundary). A Constituent that
    /// ends in one is followed by nothing but a period.
    #[serde(default)]
    pub unclosed: bool,
    /// Lexical children that cost one more where analysis did not admit
    /// their leaf through the scope of the face whose text is parsed (one of
    /// the face's own names, or a card-scoped spelling such as its Shortened
    /// Name [CR#201.5c]): another card's full name. A Production Cost
    /// preference, never an admission condition.
    #[serde(default)]
    pub scoped: Vec<Ident>,
    /// Children that are not themselves a coordination, where a coordination
    /// one level up is the same analysis (a chain of bare imperatives).
    #[serde(default)]
    pub uncoordinated: Vec<Ident>,
    /// Children that neither are nor are headed by a Linked Coordination: a
    /// subordinate clause, which a sole connective never links (CGEL, Ch. 15
    /// §2.10, p. 1320).
    #[serde(default)]
    pub unlinked: Vec<Ident>,
    /// Children that are themselves a coordination: a dependent that scopes
    /// over a whole coordination where a single conjunct has a site of its
    /// own (CGEL, Ch. 15 §1.3.4, pp. 1285–1286).
    #[serde(default)]
    pub coordinated: Vec<Ident>,
    /// Features of the mother that it does not share with its `head` child
    /// and that the Production's equations value instead: a degree modifier
    /// that changes its head's grade (an analytic superlative, CGEL, Ch. 13
    /// §4.3, p. 1122).
    #[serde(default)]
    pub unshared: Vec<Ident>,
}

pub(crate) fn default_cost() -> u32 {
    1
}

/// One right-hand-side element. Every element but `Punct` names its child.
///
/// A child's name is its grammatical function; the engine reserves some names
/// (see `ProductionDecl`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum SymbolDecl {
    /// A Constituent of a Category.
    Cat(Ident, Ident),
    /// A constituent attached at its outer boundaries (for symbol sequences).
    Attached(Ident, Ident, sylex_lexicon::Binding),
    /// A Constituent of a Category whose first atom is a free word, never a
    /// bound clitic.
    Free(Ident, Ident),
    /// A numeral in a specific notation.
    Numeral(Ident, sylex_lexicon::Numeral),
    /// Any word of a lexical Category.
    Lex(Ident, sylex_lexicon::Category),
    FreeLex(Ident, sylex_lexicon::Category),
    /// The Closed Word of that grammar name.
    Word(Ident, Ident),
    /// Any member of the Lexical Class of that grammar name.
    Class(Ident, Ident),
    /// A Constituent of a Category whose preposition, if any, admits the Host
    /// Category: a Preposition Phrase that could also modify such a host.
    Hosted(Ident, Ident, HostKind),
    /// A Preposition Phrase headed by one of the named Closed Words, as a
    /// Frame Slot `PP([...])` selects one.
    Selected(Ident, Ident, Vec<Ident>),
    /// A Constituent of a Category set in quotation marks, which the engine
    /// supplies: double at even nesting depth, single at odd ("If quoted rules
    /// text itself quotes an ability, use straight single quotation marks";
    /// `docs/oracle-style-guide.md` §3, Quotation marks and apostrophes). A
    /// period or comma that follows a closing quotation mark is written
    /// inside it, and the Constituent's own final period is not written: the
    /// punctuation is derived from the structure (CGEL, Ch. 20 §6,
    /// pp. 1753–1755; style guide §3, Periods).
    Quoted(Ident, Ident),
    /// Literal punctuation: it may decide an analysis but is not stored in a
    /// Reading.
    Punct(char),
    /// Structural punctuation with explicit attachment to adjacent atoms.
    Mark(char, sylex_lexicon::Binding),
}

/// A feature equation over `$.Feature` (the mother) or `child.Feature` paths.
/// A child named `head` implicitly agrees with the mother on every feature both
/// carry; see the reserved child names on `ProductionDecl`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Equation {
    Agree(Ident, Ident),
    Require(Ident, Ident),
    /// Excludes a value on a child without requiring an inapplicable feature.
    Exclude(Ident, Ident),
    /// Requires a child to carry the value itself: an unvalued feature, which
    /// `Require` would value, does not satisfy it.
    Carry(Ident, Ident),
    /// Excludes a combination of values: it fails only when every listed
    /// child carries its listed value. In a Coordination schema it is over
    /// `first` and `last`, the last conjunct and the one before it.
    ExcludeTogether(Vec<(Ident, Ident)>),
}

/// The Categories of a Valence schema's Complement Clusters: right
/// nonce-constituent coordination, whose coordinates are sequences of a
/// shared head's dependents that form constituents only by being coordinated
/// (CGEL, Ch. 15 §4.3, pp. 1341–1343, [20]).
///
/// A Cluster is the Frame Slots of one frame, then up to two Adjuncts. A single
/// Slot with no Adjunct is a Cluster only where it alone breaks parallelism,
/// never last and before or between at least two Clusters that end in as many
/// Adjuncts: a coordination of single Slots is their own coordination
/// (`docs/decisions/english.md` §8).
/// The coordinates are parallel: every Cluster of one coordination is of the
/// head's frame (p. 1341, [17ii]). The head takes the coordination as the
/// child `clusters`, which bears no function; each dependent in a Cluster
/// bears its own in the head's phrase too (`Constituent::also`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClustersDecl {
    /// The Category of one Cluster.
    pub cluster: Ident,
    /// The Category of a coordination of Clusters, which a Coordination
    /// schema declares.
    pub coordination: Ident,
    /// The Category of the Adjuncts that may end a Cluster.
    #[serde(default)]
    pub adjunct: Option<Ident>,
    /// Remnant Categories by Slot Kind: a Cluster after the first may fill
    /// its first Slot, of that kind, with a phrase of the Category whose head
    /// is elided, the head of the first Cluster's first Slot, shared with the
    /// verb on the left (*We'll be in Paris for a week and Bonn for three
    /// days*, *in* elided; CGEL, Ch. 15 §4.3, p. 1343, note 65). A schema
    /// declares the remnant phrases (`SchemaDecl::Parameters::remnant`).
    #[serde(default)]
    pub remnants: Vec<(SlotKind, Ident)>,
    /// Clusters of Adjuncts alone, after a head whose frame has no Slot.
    #[serde(default)]
    pub adjunct_clusters: Option<AdjunctClustersDecl>,
}

/// Clusters of Adjuncts alone, after a head whose frame has no Slot: two
/// Preposition Phrases that may modify a Predicate (*enters [with a +1/+1
/// counter on it for each other red creature you control] and [a +1/+1 counter
/// on it for each other green creature you control]*). A Cluster after the
/// first may begin with a remnant, the Preposition Phrase's Complement with its
/// preposition elided (CGEL, Ch. 15 §4.3, p. 1343, note 65: *We'll be in Paris
/// for a week and Bonn for three days*, "a PP+PP-coordination, with ellipsis of
/// *in*"). The coordinates are parallel in structure (p. 1341): the
/// coordination's Coordination schema is `parallel`, so every Cluster has the
/// initial one's prepositions, the first overt or elided
/// (`Summary::shared_head`), the last overt (`Summary::cluster_preposition`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjunctClustersDecl {
    /// The Category of one Cluster.
    pub cluster: Ident,
    /// The Category of a coordination of Clusters.
    pub coordination: Ident,
    /// The Category of the Preposition Phrase that begins a Cluster.
    pub preposition: Ident,
    /// The Category of a Preposition Phrase's remnant, its preposition
    /// elided (`SchemaDecl::Valence::remnants` of the Preposition schema).
    pub remnant: Ident,
}

/// An instance of one of the engine's built-in Schema kinds.
///
/// Binding compiles each Schema into Productions: authors write the active
/// Subcategorization Frames and what each Schema changes, and the engine
/// derives the alternations, Gap variants, coordination shapes and feature
/// summaries (`docs/decisions/english.md` §1). A Schema derives nothing its
/// grammar does not declare: a missing declaration yields no derivation or a
/// load error, never a default.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SchemaDecl {
    /// An attributive lexical head retains its phrasal Category as a Modifier.
    ///
    /// The premodified phrase passes up its head's Gap, so a premodified
    /// conjunct shares its Complement (*the noted number and kind of
    /// counters*). A verb whose every frame is Active-only
    /// ([`crate::frames::Frame`]) has no past participle Modifier.
    Premodification {
        head: sylex_lexicon::Category,
        modifier: Ident,
        host: Ident,
        forms: Vec<sylex_lexicon::WordForm>,
        #[serde(default)]
        prefix: bool,
        /// Equations on the modifier one of the `forms` heads, over its
        /// lexical child `head` (a past participle that a declared Adjective
        /// spells is not one).
        #[serde(default)]
        equations: Vec<(sylex_lexicon::WordForm, Vec<Equation>)>,
    },
    /// Catalog-declared ordered parameter patterns around a lexical head.
    ///
    /// Slots select grammar Categories; markers, bound stems and head
    /// identities stay in the Catalog, which the grammar never enumerates.
    Parameters {
        head: sylex_lexicon::Category,
        phrase: Ident,
        #[serde(default)]
        binding: Option<sylex_lexicon::Binding>,
        /// Coordinators of a pattern's final marked parameter, repeated
        /// with its marker (*protection from black and from red*).
        #[serde(default)]
        coordinators: Option<Ident>,
        /// The Category of each pattern's phrase without the head it begins
        /// with: a Complement Cluster's remnant, its head elided
        /// (`ClustersDecl::remnants`).
        #[serde(default)]
        remnant: Option<Ident>,
    },
    /// Frame-driven phrases headed by every lexeme of a lexical Category.
    ///
    /// For each frame of each head lexeme it derives a phrase of the head and
    /// its Frame Slots that records the Selected Frame. For a verb it also
    /// derives:
    /// - every object-Gap variant, and a predicative-Gap variant of a `Copular`
    ///   frame only (never of the objective predicative of *turn it face up*);
    ///   the extracting verb is a free word, never a reduced clitic (CGEL, Ch.
    ///   18 §6.2.1, p. 1615);
    /// - with `passive`, verbal passives with and without the Internalised
    ///   Complement, which may also precede later selected Preposition Phrase
    ///   Complements (*dealt by a source to a player*) and strands its
    ///   preposition like any other, and a `Prepositional` frame's passive,
    ///   which strands its Preposition Phrase;
    /// - with `adjectival`, Adjectival Passives of each frame whose passive
    ///   leaves no Complement.
    ///
    /// No passive or Gap is derived from an Active-only frame, from an Amount
    /// or Cost slot (a Complement, never an Object), or from a `MeasureNP` or
    /// keyword-phrase Object. A non-verbal head's `Intransitive` frame derives
    /// no phrase of its own: the bare phrase is it. A non-verbal head's bare
    /// phrase omits the Complements its frames select, and a Preposition
    /// Phrase right after it headed by one of their prepositions is that
    /// head's Complement, never a dependent of another host.
    Valence {
        head: sylex_lexicon::Category,
        phrase: Ident,
        #[serde(default)]
        cost: u32,
        /// The feature, carried by the phrase, that names the Slot Kind of
        /// its first Frame Slot: a non-verbal head's bare phrase names
        /// `Intransitive`, and a stranded Preposition Phrase the Slot Kind of
        /// the Complement it strands.
        #[serde(default)]
        complement_kind: Option<Ident>,
        /// Equations on every active phrase of a frame, over `$` and the
        /// lexical child `head` (a verb's Voice).
        #[serde(default)]
        active: Vec<Equation>,
        /// Derive passive phrases from active verb frames, naming the Closed
        /// Word that marks the Internalised Complement (verbs only).
        #[serde(default)]
        passive: Option<Ident>,
        /// Equations on every derived verbal passive, over `$` and the lexical
        /// child `head`: its Voice and its past participle. Required with
        /// `passive`.
        #[serde(default)]
        verbal_passive: Vec<Equation>,
        /// Equations on every derived prepositional passive, over `$` and the
        /// lexical child `head` (*each card looked at*). Required with
        /// `passive`.
        #[serde(default)]
        prepositional_passive: Vec<Equation>,
        /// Equations on the Internalised Complement of every derived
        /// passive, over the child `internalised`: what its Preposition
        /// Phrase may contain (CGEL, Ch. 16 §10.1.1, p. 1428).
        #[serde(default)]
        internalised: Vec<Equation>,
        /// Derive Adjectival Passives of this Category from verb frames whose
        /// passive leaves no Complement but the optional `by` phrase.
        #[serde(default)]
        adjectival: Option<Ident>,
        /// Equations on every derived Adjectival Passive, over `$` and the
        /// lexical child `head`. Required with `adjectival`.
        #[serde(default)]
        adjectival_passive: Vec<Equation>,
        /// Adjunct category permitted before a selected PP or shifted Object.
        #[serde(default)]
        interleaved: Option<Ident>,
        /// Adjunct category permitted only before the Internalised Complement
        /// of a passive, a non-core Complement (CGEL, Ch. 4 §3.2.1, p. 241),
        /// beside the `interleaved` one.
        #[serde(default)]
        interleaved_internalised: Option<Ident>,
        /// Postpose the Object to the end of the phrase: past selected
        /// Preposition Phrase Complements, and, when it is heavy, past a
        /// predicative Complement or an `interleaved` Adjunct alone (CGEL,
        /// Ch. 16 §4, pp. 1382–1384).
        #[serde(default)]
        heavy_shift: bool,
        /// Equations over the child `object` that a heavy postposed Object
        /// satisfies, beside ending in a dependent or being a coordination.
        #[serde(default)]
        shifted_object: Vec<Equation>,
        /// Function Category of a Modifier postposed out of the Object past
        /// the selected Preposition Phrase Complements after it, at the end
        /// of the phrase (*put a number of counters [on it] [equal to …]*;
        /// CGEL, Ch. 12 §4, p. 1066). It bears Modifier in the Object too,
        /// and agrees with the Object in each feature they both carry.
        #[serde(default)]
        postposed: Option<Ident>,
        /// A non-verbal head that declares a Frame realizes one of them in
        /// this phrase: only a head declaring none, or an Intransitive one,
        /// forms its bare phrase.
        #[serde(default)]
        obligatory: bool,
        /// A Preposition Phrase passes up no Gap from inside its Complement:
        /// only its stranded phrase has a Gap, the Complement's own. A load
        /// error on any other head.
        #[serde(default)]
        closed_complement: bool,
        /// `(phrase feature, slot feature)` pairs: the phrase's feature
        /// agrees with that feature of each Frame Slot child that carries it
        /// (a Preposition Phrase takes its Complement's Number).
        #[serde(default)]
        agree: Vec<(Ident, Ident)>,
        /// Complement Clusters: a verb heads a coordination of parallel
        /// sequences of its Frame Slots, each sequence optionally ending in
        /// an Adjunct (*put one of them into your hand and the rest into your
        /// graveyard*; *has lifelink if you control a Cleric and deathtouch
        /// if you control a Rogue*; CGEL, Ch. 15 §4.3, pp. 1341–1343).
        #[serde(default)]
        clusters: Option<ClustersDecl>,
        /// Verb Phrase remnants by Slot Kind: a verb's phrase of a frame
        /// whose first Slot is of that kind also stands without its head, in
        /// the paired Category, as a middle conjunct of a Verb Phrase
        /// coordination, the head elided and recovered from the initial
        /// conjunct's (*is enchanted, [equipped], and has a counter on it*:
        /// coordination at the level of the final conjunct, with ellipsis at
        /// the beginning of the medial ones; CGEL, Ch. 15 §3.6,
        /// pp. 1335–1336, [31ii]). A Coordination schema admits the remnants
        /// (`SchemaDecl::Coordination::remnant`).
        #[serde(default)]
        remnants: Vec<(SlotKind, Ident)>,
    },
    /// Unselected attachment of an adjunct Category to a host, limited to
    /// Preposition Phrases whose (preposition, complement frame) entry admits
    /// the [`HostKind`].
    ///
    /// Same-head complement priority is derived from the frames: a
    /// Preposition Phrase, or an adjunct of a Frame Slot's Category, does not
    /// attach as a Predicate adjunct to a head that has a longer frame
    /// selecting it (*choose to attack* has no purpose reading). A host
    /// delimited from a post-adjunct by a comma passes up no Gap, so it never
    /// closes a relative clause (CGEL, Ch. 12 §4, p. 1058).
    Adjunct {
        host: Ident,
        adjunct: Ident,
        side: Side,
        host_kind: HostKind,
        #[serde(default)]
        delimiter: Option<char>,
        #[serde(default = "default_cost")]
        cost: u32,
        /// Equations over the `head` (host) and the adjunct's function child.
        #[serde(default)]
        equations: Vec<Equation>,
        /// The phrase it builds is only a conjunct of a coordination other
        /// than the last, never a child in any other position: a comma-set
        /// Supplement anchored to a Verb Phrase conjunct before the next
        /// (*searches … for up to X cards, where X is …, puts them …*), where
        /// the last conjunct's Supplement is the Clause's (CGEL, Ch. 15 §5.1,
        /// p. 1351).
        #[serde(default)]
        nonfinal: bool,
    },
    /// Binary and serial coordination of a Category, with Right Node Raising
    /// when the Category may contain a Gap.
    ///
    /// It derives the binary coordination, the Oxford-comma serial one, the
    /// delimited polysyndetic chain (*A, or B, or C*) and across-the-board
    /// Gaps, with one tree per chain joined by one coordinator. Right Node
    /// Raising, a Gap shared at each conjunct's right edge and filled once
    /// after the coordination, is derived for a Category's first
    /// Coordination schema. Every schema whose coordinators admit a
    /// [`Grammar::correlatives`] coordinator also derives the coordination
    /// after its marker, and the first one whose coordinators admit a
    /// [`Grammar::idiomatic_coordinates`] coordinator, with no `unlike`
    /// feature, derives the binary and serial ends with its pro-form. A
    /// coordination of Preposition Phrases keeps up to two prepositions for
    /// selection and admits only the Host Categories every conjunct admits.
    Coordination {
        category: Ident,
        /// The Closed Word or Lexical Class of its Coordinators. Exactly one
        /// of `coordinators` and `link` is given.
        #[serde(default)]
        coordinators: Option<Ident>,
        /// A Linked Coordination, which has no Coordinator: the Category of
        /// its Linked Conjunct, the one after each link, marked by its own
        /// initial connective adverb instead (*Draw a card, then discard a
        /// card*; CGEL, Ch. 15 §2.10, pp. 1319–1320). Its serial middle
        /// Conjuncts are ordinary ones, which may begin with the connective
        /// too (*A, then B, then C*).
        #[serde(default)]
        link: Option<Ident>,
        #[serde(default = "default_cost")]
        cost: u32,
        /// Features the conjuncts need not share, which the coordination
        /// leaves unvalued unless its `equations` value them.
        #[serde(default)]
        unshared: Vec<Ident>,
        /// Equations over the whole coordination (`$`): its resolved
        /// features (an *and*-coordination of Noun Phrases is plural).
        #[serde(default)]
        equations: Vec<Equation>,
        /// Features the coordination takes from its last conjunct, which the
        /// others need not share (proximity: *one or more cards*).
        #[serde(default)]
        proximate: Vec<Ident>,
        /// Features every conjunct but the last shares, which the
        /// coordination resolves with its last conjunct's: their common value,
        /// or the listed one when they differ (an *and*-coordination of Noun
        /// Phrases is determined only when every conjunct is). A conjunct
        /// without a value does not decide it.
        #[serde(default)]
        resolve: Vec<(Ident, Ident)>,
        /// Punctuation that may also precede the coordinator of a binary
        /// coordination (two independent clauses joined by *, and*).
        #[serde(default)]
        delimiter: Option<char>,
        /// The binary coordination exists only with the delimiter (a
        /// declarative and then an imperative: *…, and put …*).
        #[serde(default)]
        delimited: bool,
        /// Punctuation that sets off the final coordinate of a binary
        /// coordination sharing a Gap by Right Node Raising, before its
        /// coordinator and before the shared constituent (*attracted to,
        /// and overawed by, the new lodger*; CGEL, Ch. 20 §3.2.5, p. 1746,
        /// [26iii]; Ch. 15 §4.4, pp. 1343–1345). A load error where the
        /// schema derives no Right Node Raising.
        #[serde(default)]
        raising_delimiter: Option<char>,
        /// Whether serial coordinations are derived: by default for like
        /// conjuncts, not for unlike ones.
        #[serde(default)]
        serial: Option<bool>,
        /// The Category of the conjuncts, when the coordination is of a
        /// function its conjuncts fill (a depictive *tapped and attacking*).
        #[serde(default)]
        conjunct: Option<Ident>,
        /// A feature whose value the two conjuncts of a binary coordination
        /// must not share: a coordination of unlike categories in one
        /// function (CGEL, Ch. 15 §3.2, pp. 1326–1329); the conjuncts of
        /// serial and polysyndetic ones, derived only with `serial`, are not
        /// all of one value.
        #[serde(default)]
        unlike: Option<Ident>,
        /// The Category of a phrase whose head is elided that may stand as a
        /// middle conjunct of a serial coordination, its head recovered from
        /// the initial conjunct's (`SchemaDecl::Valence::remnants`). The
        /// initial and last conjuncts are no remnants (CGEL, Ch. 15 §3.6,
        /// pp. 1335–1336): a coordination whose later conjuncts all were is
        /// the coordination of their Complements under the one head (*is a
        /// creature, an artifact, and an enchantment*; §3.2, pp. 1326–1327).
        #[serde(default)]
        remnant: Option<Ident>,
        /// Features a remnant shares with the initial conjunct, which the
        /// other conjuncts need not share: those of the element it elides
        /// (an elided Subject's Number and Person, *you gain 4 life, [draw
        /// four cards], and he deals 4 damage*, never *[draws four cards]*).
        #[serde(default)]
        recovered: Vec<Ident>,
        /// Every conjunct after the initial has the initial conjunct's
        /// lexical head (`Summary::shared_head`), overt or elided in a
        /// remnant, and its last preposition (`Summary::cluster_preposition`;
        /// `AdjunctClustersDecl`).
        #[serde(default)]
        parallel: bool,
    },
}

impl SchemaDecl {
    /// A Coordination's Coordinators or, for a Linked Coordination, its
    /// Linked Conjunct Category: what names its serial tail.
    pub(crate) fn joined_by(&self) -> Option<&str> {
        match self {
            Self::Coordination {
                coordinators, link, ..
            } => coordinators.as_deref().or(link.as_deref()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Side {
    Before,
    After,
}

/// The kind of host an Adjunct schema attaches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum HostKind {
    Nominal,
    Predicate,
    /// A Clause or line that a comma-set Supplement is anchored to (CGEL,
    /// Ch. 15 §5.1, p. 1350).
    Supplement,
}
