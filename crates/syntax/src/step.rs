//! Productions as compiled, and the one step function shared by the chart
//! and by Admission: adding a child to a partial Production.

use dpsi::Ident;
use serde::Deserialize;
use serde::Serialize;
use sylex_lexicon::Category as LexicalCategory;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::WordForm;

use crate::compile::CompiledGrammar;
use crate::compile::LexemeFacts;
use crate::frames::SlotKind;
use crate::grammar::HostKind;
use crate::lexeme::Lexeme;

/// The most features one Category may carry, so a Summary is inline.
pub(crate) const MAX_CATEGORY_FEATURES: usize = 12;
/// The most feature variables one Production may unify, so a State is inline.
pub(crate) const MAX_PRODUCTION_VARS: usize = 16;
/// The most features a grammar may declare, so a leaf indexes its values.
pub(crate) const MAX_FEATURES: usize = 48;
/// The most conjuncts one coordination Production may mark distinct.
pub(crate) const MAX_DISTINCT_COORDINATORS: usize = 2;
/// The most features one coordination Production may resolve.
pub(crate) const MAX_RESOLUTIONS: usize = 4;
/// The most conditions the joint exclusions of one Production may list.
pub(crate) const MAX_JOINT_CONDITIONS: usize = 32;

/// One feature value, with the final byte value reserved for an unbound slot.
/// Binding validates the domain before any value reaches a chart State.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct FeatureValue(u8);

/// A run of values hashes a word per eight: the `[FeatureValue; N]` arrays of
/// a State and a Summary key the chart's indexes.
impl std::hash::Hash for FeatureValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.0);
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H) {
        for chunk in data.chunks(8) {
            let mut word = [0; 8];
            for (byte, value) in word.iter_mut().zip(chunk) {
                *byte = value.0;
            }
            state.write_u64(u64::from_le_bytes(word));
        }
    }
}

impl FeatureValue {
    pub(crate) const UNSET: Self = Self(u8::MAX);

    pub(crate) fn new(value: u16) -> Self {
        let value = u8::try_from(value).expect("validated feature domain");
        assert_ne!(value, u8::MAX, "validated feature domain");
        Self(value)
    }

    pub(crate) fn get(self) -> Option<u16> {
        (self != Self::UNSET).then_some(u16::from(self.0))
    }
}

/// A Complement that another frame of a phrase's head selects. A longer
/// frame's Preposition Phrase headed by the preposition, or phrase of the
/// slot's kind (*to*-infinitival in *choose to attack*), is not also an
/// unselected adjunct of the same head instance; a Preposition Phrase that a
/// frame selects in place of a predicative Complement (locative *on* after
/// *be*) is not also that head's predicative Complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Extension {
    /// A Preposition Phrase headed by the preposition, with a Complement of
    /// the kind where the slot names one.
    Preposition(Lexeme, Option<SlotKind>),
    Slot(SlotKind),
    Predicative(Lexeme),
}

impl Extension {
    /// Whether a frame selecting `self` selects `phrase`, an adjunct's or a
    /// following phrase's extension: a Preposition Phrase slot that names no
    /// Complement kind selects its preposition with any Complement, and a
    /// hollow *to*-infinitival slot (*cost {1} to activate*, CGEL, Ch. 14
    /// §6.3(c), p. 1250) selects a *to*-infinitival phrase.
    pub(crate) fn selects(self, phrase: Self) -> bool {
        match (self, phrase) {
            (Self::Preposition(p, None), Self::Preposition(q, _)) => p == q,
            (Self::Preposition(p, Some(k)), Self::Preposition(q, Some(j))) => p == q && k == j,
            (Self::Slot(SlotKind::HollowToInf), Self::Slot(SlotKind::ToInf)) => true,
            (selected, phrase) => selected == phrase,
        }
    }
}

/// The unfilled Gap a Constituent contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Gap {
    None,
    NP,
    /// An extracted adjunct, as in an adverbial or pied-piped relative clause.
    Adjunct,
    /// An extracted predicative Complement (*whose turn it is*).
    Pred,
    /// A keyword-phrase Complement shared by Right Node Raising (*can't have
    /// or gain trample*).
    Keyword,
    /// A *to*-infinitival Complement shared by Right Node Raising (*choose
    /// and choose to attack*).
    ToInf,
    /// A noun's Preposition Phrase Complement shared by Right Node Raising
    /// (*the top or bottom of their library*).
    PP,
    /// Nouns' Amount Complements shared, respectively, by a slash Measure
    /// (*base power and toughness 4/4*).
    Measure,
    /// A bare infinitival Complement shared by coordinated auxiliaries (*may
    /// and can draw cards*).
    BareInf,
    /// A preposition's Amount Complement shared by Right Node Raising (*less
    /// than or equal to X*).
    Amount,
}

impl Gap {
    /// This Gap's bit in a Category's mask of the Gaps it may contain.
    pub(crate) const fn bit(self) -> u16 {
        1 << self as u8
    }
}

/// How a child's Gap relates to its mother's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GapRule {
    /// The child contains no Gap.
    Closed,
    /// The child's Gap, if any, is the mother's.
    Transparent,
    /// Every shared child has the same Gap, which is the mother's (across the
    /// board).
    Shared,
    /// The child has exactly this Gap, filled within the mother.
    Require(Gap),
    /// The child has exactly this Gap, which is the mother's.
    Raise(Gap),
}

/// How heavy a child must be: one lexical leaf, more than one, or
/// postposable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Weight {
    Light,
    Heavy,
    /// A phrase that ends in a dependent of its head or is a coordination:
    /// heavy enough relative to a dependent of its verb to be postposed
    /// past it (CGEL, Ch. 16 §4, pp. 1382–1384; [1iv] *put on the table [a
    /// large blue bucket full of ice-cubes]*). A head-final phrase (*this
    /// turn*, *a card*) or a single word stays in place.
    Postposable,
}

/// A lexical leaf test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Test {
    Lexical(LexicalCategory),
    FreeLexical(LexicalCategory),
    /// A free lexeme of this Category whose Complement is optional: it
    /// declares no Subcategorization Frame, or an Intransitive one.
    FreeOptional(LexicalCategory),
    /// A free lexeme of this Category not every Frame of which is restricted
    /// to active voice: an Active-only verb has no passive participle.
    FreeNotActiveOnly(LexicalCategory),
    Numeral(sylex_lexicon::Numeral),
    Word(Lexeme),
    Class(usize),
    Words(std::collections::BTreeSet<Lexeme>),
    /// A lexeme that declares this Subcategorization Frame.
    Frame(usize),
    /// A free (unbound) lexeme that declares this Subcategorization Frame.
    FreeFrame(usize),
    /// A member of the lexical class that declares the Subcategorization
    /// Frame; with `free`, an unbound one.
    ClassFrame {
        class: usize,
        frame: usize,
        free: bool,
    },
    /// A preposition that declares this complement frame.
    PrepositionFrame(usize),
    /// Another card's name (`LeafFacts::other_card_name`), which no other
    /// test admits.
    OtherCardName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Symbol {
    Cat {
        category: usize,
        binding: Option<sylex_lexicon::Binding>,
        /// A selected `PP` slot: the phrase's preposition must be one of
        /// these, and its Complement of the kind where one is named.
        prepositions: Option<(Vec<Lexeme>, Option<SlotKind>)>,
        /// An unselected adjunct: a phrase with a preposition must admit this
        /// host.
        host: Option<HostKind>,
    },
    Leaf(Test),
    /// A mark, with the Onset of the word it is spoken as, if it is one: a
    /// sign is spoken *plus* or *minus* (*a +1/+1 counter*), and takes part
    /// in Adjacency Agreement as a word does. Any other mark is no word.
    Punct(char, sylex_lexicon::Binding, Option<sylex_lexicon::Onset>),
}

/// The Complements a Production's phrase omits at its right edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Omission {
    None,
    /// A non-verbal head's bare phrase omits the Complements its head's
    /// frames select (`Summary::omitted_complement`).
    Selected,
    /// It ends in an ellipsis site whose omitted Complement takes these word
    /// forms (`ProductionDecl::elliptical`), as a mask (`form_bit`).
    Ellipsis(u16),
}

/// A word form's bit in a mask of word forms.
pub(crate) fn form_bit(form: WordForm) -> u16 {
    1 << form as u16
}

/// The bit of `Summary::unlike_values` marking a conjunct with no value of
/// the unlike feature, whose own values take the bits below it.
pub(crate) const UNVALUED_CONJUNCT: u64 = 1 << 63;

/// The conjuncts of an unlike coordination: its `first` child and its
/// `last`, a conjunct or, when `rest`, a serial tail whose Summary gathers
/// its own conjuncts' values; `feature` is the unlike feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Unlike {
    pub first: usize,
    pub last: usize,
    pub feature: usize,
    pub rest: bool,
}

impl Unlike {
    pub(crate) fn new(first: usize, last: usize, feature: usize, rest: bool) -> Self {
        Self {
            first,
            last,
            feature,
            rest,
        }
    }
}

/// A child's grammatical function as the engine keys on it: its name,
/// resolved at bind time (the reserved child names of
/// [`crate::grammar::ProductionDecl`]). Any other name is inert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChildRole {
    Head,
    First,
    Last,
    Rest,
    /// An idiomatic coordinate (*and so on*), counted as a conjunct.
    Coordinate,
    Adjunct,
    Modifier,
    Supplement,
    Object,
    /// The shared child of Right Node Raising.
    Shared,
    /// A coordination of dependent Clusters sharing this phrase's head.
    Clusters,
    /// An inner Cluster whose dependents share its mother Cluster's host.
    Cluster,
    Coordinator,
    Marker,
    Subject,
    IndirectObject,
    Complement,
    PredicativeComplement,
    Determiner,
    /// Any other name, and punctuation's empty one.
    Inert,
}

impl ChildRole {
    /// The role a child's name gives it.
    pub(crate) fn of(name: &str) -> Self {
        match name {
            "head" => Self::Head,
            "first" => Self::First,
            "last" => Self::Last,
            "rest" => Self::Rest,
            "coordinate" => Self::Coordinate,
            "adjunct" => Self::Adjunct,
            "modifier" => Self::Modifier,
            "supplement" => Self::Supplement,
            "object" => Self::Object,
            "shared" => Self::Shared,
            "clusters" => Self::Clusters,
            "cluster" => Self::Cluster,
            "coordinator" => Self::Coordinator,
            "marker" => Self::Marker,
            "subject" => Self::Subject,
            "indirect_object" => Self::IndirectObject,
            "complement" => Self::Complement,
            "predicative_complement" => Self::PredicativeComplement,
            "determiner" => Self::Determiner,
            _ => Self::Inert,
        }
    }

    /// Whether it bears a dependent function in its mother (CGEL's
    /// functions, as the grammar names them): a head line does not run
    /// through one while its Production has another child.
    pub(crate) fn is_dependent(self) -> bool {
        matches!(
            self,
            Self::Subject
                | Self::Object
                | Self::IndirectObject
                | Self::Complement
                | Self::PredicativeComplement
                | Self::Adjunct
                | Self::Modifier
                | Self::Determiner
                | Self::Supplement
        )
    }

    /// Whether it is an unselected dependent: an Adjunct or a Modifier.
    pub(crate) fn is_unselected(self) -> bool {
        matches!(self, Self::Adjunct | Self::Modifier)
    }

    /// Whether it is in its mother's head line: the head, or a conjunct of
    /// a coordination.
    pub(crate) fn in_head_line(self) -> bool {
        matches!(self, Self::Head | Self::First | Self::Last | Self::Rest)
    }
}

/// One child position of a Production: the gates a child there must pass
/// that depend on nothing but the child, which [`Production::accept`] checks
/// before it copies and propagates its State, and, for a Category child,
/// where its Category carries each feature.
#[derive(Debug, Clone, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent gates of one child"
)]
pub(crate) struct ChildSlot {
    /// For a Category child, the offset in its `Summary::values` of each
    /// feature its Category carries, by feature id.
    offsets: Vec<Option<u8>>,
    /// (feature, value) the child may not carry.
    pub exclusions: Vec<(usize, u16)>,
    /// (feature, value) the child must carry, not merely admit.
    pub carried: Vec<(usize, u16)>,
    /// It must be light or heavy, or end in a dependent.
    pub weight: Option<Weight>,
    /// It may not be a coordination.
    pub uncoordinated: bool,
    /// It must be a coordination.
    pub coordinated: bool,
    /// Its `Summary::whole_coordination` must be this: a phrase that is
    /// itself a coordination, or one that is not.
    pub whole_coordination: Option<bool>,
    /// It may not be, nor be headed by, a Linked Coordination
    /// (`Summary::link`).
    pub unlinked: bool,
    /// It is a conjunct of its coordination other than the last: the only
    /// position a `Summary::nonfinal` phrase may fill.
    pub nonfinal_conjunct: bool,
    /// It opens with a capital (`ProductionDecl::capital`).
    pub capital: bool,
    /// A derived unlike Frame Slot: the mask of values outside which the
    /// coordination filling it has no conjunct's value of its `unlike`
    /// feature (`Grammar::unlike_slots`).
    pub unlike_mask: Option<u64>,
    /// An `unextended` child: the prepositions of a Preposition Phrase it
    /// may not end in (`ProductionDecl::unextended`).
    pub unextended: Option<std::collections::BTreeSet<Lexeme>>,
    /// An `unadjoined` child: the Categories of a trailing Adjunct it may
    /// not end in (`ProductionDecl::unadjoined`).
    pub unadjoined: Option<Vec<u16>>,
    /// It is set off before the head by a comma: a fronted dependent
    /// (*When …, …*), which ends at that comma.
    set_off_before_head: bool,
    /// Off the edge line, the Category of the trailing Adjunct the phrase
    /// ends in when it ends in this child, if a Summary's `final_adjunct`
    /// records it (`Production::track_final_adjunct`).
    trailing_adjunct: Option<u16>,
    /// The extent a Derivation keeps of it for a later child's Ranking Key
    /// (`Parse::anchors`).
    pub anchor: Option<Anchor>,
    /// What it adds to the Ranking Key (`CompiledGrammar::attachment_key`).
    pub attachment: AttachmentRole,
}

/// A child whose extent a later child's Ranking Key reads (`Anchors`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Anchor {
    /// The head, which unselected dependents or a Supplement follow.
    Head,
    /// The first conjunct, which a last one follows.
    First,
}

/// A child position's part in the Ranking Key, by its role and its
/// siblings' (`CompiledGrammar::attachment_key`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AttachmentRole {
    /// For the head, the unselected dependents (`adjunct`, `modifier`) after
    /// it, each of which it is the host of.
    pub dependents_after: u32,
    /// A head precedes it.
    pub after_head: bool,
    /// It is an unselected dependent before its head, after another one:
    /// it belongs inside the dependent before it.
    pub preposed_after_dependent: bool,
}

/// A Production's static structural facts, precomputed once binding has
/// settled it ([`Production::seal`]).
#[derive(Debug, Clone, Copy, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent facts of one Production"
)]
pub(crate) struct Shape {
    /// `CompiledGrammar::head_line_child`.
    pub head_line_child: Option<usize>,
    /// `CompiledGrammar::is_frame_production`.
    pub frame_production: bool,
    /// It begins with a fronted dependent of its head, which is its last
    /// child: one set off by a comma (*When …, …*) or a preposed Adjunct
    /// without one (*instead put …*) (`Summary::fronted`).
    pub fronts: bool,
    /// It hosts a comma-set post-dependent: its head comes first, followed
    /// by a comma.
    pub hosts_comma_set_dependent: bool,
    /// Its first child is the head of a Production of its own Category whose
    /// every later child is an Adjunct: a post-verbal Adjunct's host, which
    /// an auxiliary Verb Phrase never is (`Summary::auxiliary`).
    pub post_adjunct_host: bool,
    /// Its first child is the head of a Production of its own Category that
    /// leaves a coordination a coordination (`Summary::coordinated`): a
    /// post-adjunct host, or a post-dependent that itself requires a
    /// coordination (`ChildSlot::coordinated`).
    pub keeps_coordination: bool,
    /// Its last non-punctuation child is in the head line.
    pub ends_in_head_line: bool,
    /// Its last non-punctuation child is in the head line and it takes no
    /// `Determiner` child.
    pub undetermined_head_final: bool,
}

impl ChildSlot {
    fn new(grammar: &CompiledGrammar, symbol: &Symbol) -> Self {
        let offsets = match symbol {
            Symbol::Cat { category, .. } => {
                let mut offsets = vec![None; grammar.features.len()];
                for (offset, &feature) in grammar.category_features[*category].iter().enumerate() {
                    offsets[feature] = Some(u8::try_from(offset).expect("validated Category size"));
                }
                offsets
            }
            Symbol::Leaf(_) | Symbol::Punct(..) => Vec::new(),
        };
        Self {
            offsets,
            ..Self::default()
        }
    }

    /// A feature's value on a Constituent of `summary` in this Category
    /// slot.
    fn node_value(&self, summary: &Summary, feature: usize) -> Option<u16> {
        let offset = self.offsets[feature]?;
        summary.values[usize::from(offset)].get()
    }
}

/// A compiled Production.
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent production facts"
)]
pub(crate) struct Production {
    /// Each child's slot, by its position: its local admission gates and
    /// where its Category carries each feature.
    pub slots: Vec<ChildSlot>,
    /// The first `uncoordinated` child it declares, over which Reading
    /// Normal Form distributes a coordination of that child.
    pub distributed: Option<usize>,
    /// A frame-headed Production's id as binding built it: its phrase, its
    /// Valence Alternation and what its id adds after the frame name.
    pub frame_variant: Option<crate::compile::FrameVariant>,
    /// The Frame Slot it extracts, as its id numbers it (`/gapN`), which a
    /// derived phrase keeps.
    pub gap_slot: Option<usize>,
    /// Combinations of (child, feature, value) conditions that may not all
    /// hold: each exclusion is the run of `joint_conditions` up to its end
    /// index.
    pub joint_exclusions: Vec<usize>,
    pub joint_conditions: Vec<(usize, usize, u16)>,
    /// Features a coordination resolves: (feature, value when the conjuncts
    /// but the last and the last differ, the mother feature position a whole
    /// coordination resolves; a serial tail binds that position to its
    /// conjuncts but the last instead).
    pub resolutions: Vec<(usize, u16, Option<usize>)>,
    /// The children that carry the value of the conjuncts but the last.
    pub resolution_nonlast: Vec<usize>,
    /// The child that carries the last conjunct's value, as its own feature
    /// or (`true`) as a serial tail's recorded last value.
    pub resolution_last: Option<(usize, bool)>,
    /// Children in the head line: the head, or a conjunct of a coordination.
    pub head_line: Vec<bool>,
    /// Children through which the phrase ends in what they end in
    /// (`Summary::final_adjunct`): the head, and a last Complement of the
    /// Production's own Category after it (a modal's bare infinitival). A
    /// coordination's last conjunct is not one: a dependent after the
    /// coordination may scope over every conjunct.
    pub edge_line: Vec<bool>,
    pub id: Ident,
    pub lhs: usize,
    pub rhs: Vec<Symbol>,
    /// The name of each child: its grammatical function in the mother.
    pub names: Vec<Ident>,
    /// The role each child's name gives it.
    pub roles: Vec<ChildRole>,
    pub gaps: Vec<GapRule>,
    pub cost: u32,
    /// (feature, variable) bindings of each child.
    pub child_vars: Vec<Vec<(usize, usize)>>,
    /// (feature, variable) bindings of the mother, in category feature order.
    pub mother_vars: Vec<Option<usize>>,
    pub introduces: Gap,
    pub frame: Option<usize>,
    pub host_check: Option<(usize, usize, HostKind)>,
    /// The Frame Slot kind of a non-prepositional adjunct, for same-head
    /// complement priority.
    pub adjunct_slot: Option<SlotKind>,
    /// The (Frame Slot kind, feature, value) conditions under which the
    /// `host_check` dependent could fill no Frame Slot of that kind, by the
    /// grammar's `slot_equations`: same-head complement priority does not
    /// rule it out, since no frame could select it (*attack [with their hands
    /// revealed]*, whose Absolute Clause no `PP` slot takes).
    pub adjunct_unfit: Vec<(SlotKind, usize, u16)>,
    /// The predicative Complement child of a frame-headed phrase.
    pub predicative: Option<usize>,
    pub coordinator: Option<usize>,
    pub distinct_coordinator: Vec<usize>,
    pub requires_coordination: Option<usize>,
    /// The coordinator a correlative coordination's serial tail must end with.
    pub paired_coordinator: Option<Lexeme>,
    /// It is a coordination delimited before its coordinator.
    pub delimits: bool,
    /// It is a serial tail of a coordination (*…, [B, and C]*): its `first`
    /// conjunct is a middle one, after the initial conjunct.
    pub tail: bool,
    /// It builds the part of a frame's phrase before an interleaved
    /// dependent, which only its `/interleaved` Production takes.
    pub interleaved_prefix: bool,
    /// A polysyndetic tail child that must repeat the mother's coordinator.
    pub chained: Option<usize>,
    /// The preposition of the Preposition Phrase Complement it extracts.
    pub gap_preposition: Option<Lexeme>,
    /// The shared child of Right Node Raising, which must be headed by the
    /// preposition of the Gap it fills.
    pub fills_preposition: Option<usize>,
    /// Coordinators a plain serial coordination's tail may not end with.
    pub foreign_coordinators: Vec<Lexeme>,
    /// The conjuncts of an unlike coordination, which may not all share a
    /// feature value. Its Summary gathers their values
    /// (`Summary::unlike_values`); the whole requires two or more.
    pub unlike: Option<Unlike>,
    /// An auxiliary Verb Phrase's Complement: the child of an uncoordinated
    /// Production after its head, last, and of a function Category aliasing
    /// the Production's own Category, the participial Verb Phrase Complement
    /// of perfect *have* or of passive or progressive *be*
    /// (`Summary::auxiliary`).
    pub auxiliary: Option<usize>,
    /// The Complements its phrase omits at its right edge.
    pub omission: Omission,
    /// It opens with a dash only its sentence's end closes
    /// (`Summary::unclosed`).
    pub unclosed: bool,
    /// Its phrase is only a conjunct of a coordination other than the last
    /// (`Summary::nonfinal`).
    pub nonfinal: bool,
    /// The child whose `Summary::head_forms` the mother takes: its head, or
    /// the first conjunct of a coordination.
    pub lexical_head: Option<usize>,
    /// The identity of the Linked Coordination it builds, which has no
    /// Coordinator leaf: it stands for one as the coordination's own
    /// (`Summary::coordinator`) and keeps each chain to one tree
    /// (`Summary::link`).
    pub link: Option<LinkedCoordination>,
    /// The child whose engine fields (preposition, extensions) the mother
    /// inherits.
    pub head: Option<usize>,
    /// It builds a Complement Cluster of this frame (`Summary::cluster`).
    pub cluster: Option<u16>,
    /// The child that must be a coordination of Complement Clusters of this
    /// frame, the head's (`grammar::ClustersDecl`).
    pub clusters: Option<(usize, u16)>,
    /// Its part in Complement Cluster remnants (`grammar::ClustersDecl`).
    pub remnant: RemnantRole,
    /// A coordination whose later conjuncts have its initial conjunct's
    /// lexical head, overt or elided (`SchemaDecl::Coordination::parallel`).
    pub parallel: bool,
    /// Its static structural facts.
    pub shape: Shape,
    /// The State before any child, its variables bound to their constants.
    start: State,
}

pub(crate) type Key = (Option<usize>, usize);

/// A Production's part in Complement Cluster remnants
/// (`grammar::ClustersDecl::remnants`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RemnantRole {
    None,
    /// Its lexical head may be elided from a later Cluster's remnant
    /// (`Summary::shared_head`).
    SharesHead,
    /// It is a remnant, this head elided (`Summary::remnant`).
    Elides(Elided),
}

/// A Linked Coordination's identity: its serial tail Category, which
/// names the one schema whose drafts build it (`Production::link`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct LinkedCoordination(pub u16);

/// What a coordination's Summary names as its coordinator
/// (`Summary::coordinator`): its Coordinator's Lexeme, or the identity of
/// the Linked Coordination, which has no Coordinator, standing for one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum CoordinatorId {
    Lexeme(Lexeme),
    Linked(LinkedCoordination),
}

/// The head a remnant elides (`Summary::remnant`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Elided {
    /// This lexeme, as a keyword elided from its parameters.
    Lexeme(Lexeme),
    /// The head of this frame, as a verb elided from its Complements: any
    /// lexeme that declares the frame (`grammar::SchemaDecl::Valence`
    /// `remnants`).
    Frame(u16),
    /// The head of this preposition frame, as a preposition elided from its
    /// Complement: any preposition that declares the frame
    /// (`grammar::AdjunctClustersDecl`).
    PrepositionFrame(u16),
}

impl Elided {
    /// Whether `head`, the lexical head shared by an initial conjunct
    /// (`Summary::shared_head`), is the head this remnant elides.
    fn recovered_by(self, grammar: &CompiledGrammar, head: Lexeme) -> bool {
        match self {
            Self::Lexeme(elided) => elided == head,
            Self::Frame(frame) => grammar.facts(head).frames.contains(&usize::from(frame)),
            Self::PrepositionFrame(frame) => grammar
                .facts(head)
                .preposition_frames
                .contains(&usize::from(frame)),
        }
    }
}

/// Builds a Production from resolved equations by unifying paths into
/// variables.
#[expect(clippy::too_many_arguments, reason = "one internal call shape")]
#[expect(clippy::too_many_lines, reason = "one Production literal")]
pub(crate) fn resolve(
    grammar: &CompiledGrammar,
    id: Ident,
    lhs: usize,
    children: Vec<(Ident, Symbol, GapRule)>,
    equations: Vec<(Key, Result<Key, u16>)>,
    cost: u32,
    introduces: Gap,
    gap_followed: bool,
    frame: Option<usize>,
    host_check: Option<(usize, usize, HostKind)>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
) -> Result<Production, String> {
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut keys: Vec<Key> = Vec::new();
    let mut parent: Vec<usize> = Vec::new();
    let mut constant: Vec<Option<u16>> = Vec::new();
    let var = |keys: &mut Vec<Key>,
               parent: &mut Vec<usize>,
               constant: &mut Vec<Option<u16>>,
               key: Key| {
        if let Some(i) = keys.iter().position(|k| *k == key) {
            return i;
        }
        keys.push(key);
        parent.push(parent.len());
        constant.push(None);
        keys.len() - 1
    };
    for (path, target) in equations {
        let a = var(&mut keys, &mut parent, &mut constant, path);
        match target {
            Ok(other) => {
                let b = var(&mut keys, &mut parent, &mut constant, other);
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                if ra != rb {
                    match (constant[ra], constant[rb]) {
                        (Some(x), Some(y)) if x != y => {
                            return Err("contradictory feature equations".to_owned());
                        }
                        (Some(x), _) | (_, Some(x)) => constant[rb] = Some(x),
                        _ => {}
                    }
                    parent[ra] = rb;
                }
            }
            Err(value) => {
                let r = find(&mut parent, a);
                if constant[r].is_some_and(|c| c != value) {
                    return Err("contradictory feature requirements".to_owned());
                }
                constant[r] = Some(value);
            }
        }
    }
    // Renumber roots densely.
    let mut roots = Vec::new();
    let slot_of: Vec<usize> = (0..keys.len())
        .map(|i| {
            let r = find(&mut parent, i);
            roots.iter().position(|&x| x == r).unwrap_or_else(|| {
                roots.push(r);
                roots.len() - 1
            })
        })
        .collect();
    let constants: Vec<Option<u16>> = roots.iter().map(|&r| constant[r]).collect();
    if constants.len() > MAX_PRODUCTION_VARS {
        return Err(format!(
            "unifies {} feature variables, more than {MAX_PRODUCTION_VARS}",
            constants.len()
        ));
    }
    if grammar.category_features[lhs].len() > MAX_CATEGORY_FEATURES {
        return Err(format!(
            "its mother carries {} features, more than {MAX_CATEGORY_FEATURES}",
            grammar.category_features[lhs].len()
        ));
    }
    if distinct_coordinator.len() > MAX_DISTINCT_COORDINATORS {
        return Err(format!(
            "marks {} conjuncts distinct, more than {MAX_DISTINCT_COORDINATORS}",
            distinct_coordinator.len()
        ));
    }
    let mut vars = [FeatureValue::UNSET; MAX_PRODUCTION_VARS];
    for (slot, value) in vars.iter_mut().zip(constants) {
        *slot = value.map_or(FeatureValue::UNSET, FeatureValue::new);
    }
    let mut child_vars = vec![Vec::new(); children.len()];
    let mut mother_vars = vec![None; grammar.category_features[lhs].len()];
    for (i, key) in keys.iter().enumerate() {
        match key {
            (Some(child), feature) => child_vars[*child].push((*feature, slot_of[i])),
            (None, feature) => {
                let position = grammar.category_features[lhs]
                    .iter()
                    .position(|f| f == feature)
                    .expect("validated mother feature");
                mother_vars[position] = Some(slot_of[i]);
            }
        }
    }
    let roles: Vec<ChildRole> = children
        .iter()
        .map(|(name, ..)| ChildRole::of(name))
        .collect();
    let head = roles.iter().position(|&role| role == ChildRole::Head);
    let lexical_head = head.or_else(|| roles.iter().position(|&role| role == ChildRole::First));
    let head_line = roles.iter().map(|role| role.in_head_line()).collect();
    let edge_line = (0..children.len()).map(|i| head == Some(i)).collect();
    let names = children.iter().map(|(name, ..)| *name).collect();
    let slots = children
        .iter()
        .map(|(_, symbol, _)| ChildSlot::new(grammar, symbol))
        .collect();
    let (rhs, gaps) = children.into_iter().map(|(_, s, g)| (s, g)).unzip();
    if introduces != Gap::None && grammar.category_gap[lhs] & introduces.bit() == 0 {
        return Err("introduces a gap on a category that cannot contain one".to_owned());
    }
    Ok(Production {
        slots,
        distributed: None,
        frame_variant: None,
        gap_slot: None,
        joint_exclusions: Vec::new(),
        joint_conditions: Vec::new(),
        resolutions: Vec::new(),
        resolution_nonlast: Vec::new(),
        resolution_last: None,
        edge_line,
        head_line,
        adjunct_slot: None,
        adjunct_unfit: Vec::new(),
        predicative: None,
        paired_coordinator: None,
        delimits: false,
        tail: false,
        interleaved_prefix: false,
        chained: None,
        gap_preposition: None,
        fills_preposition: None,
        foreign_coordinators: Vec::new(),
        unlike: None,
        auxiliary: None,
        omission: Omission::None,
        unclosed: false,
        nonfinal: false,
        lexical_head,
        link: None,
        cluster: None,
        clusters: None,
        remnant: RemnantRole::None,
        parallel: false,
        shape: Shape::default(),
        id,
        lhs,
        rhs,
        names,
        roles,
        gaps,
        cost,
        child_vars,
        mother_vars,
        introduces,
        frame,
        host_check,
        coordinator,
        distinct_coordinator,
        requires_coordination,
        head,
        start: State {
            first_glues: None,
            last_glues: false,
            first_edge: None,
            last_edge: None,
            opening: Opening::Empty,
            vars,
            gap: None,
            // A Gap the Production introduces before a later child is not
            // at its right edge.
            gap_right_edge: !gap_followed,
            shared_gap_right_edge: true,
            gap_preposition: None,
            conjoined: false,
            leaves: 0,
            head_final: false,
            bare_head: true,
            final_preposition: None,
            final_adjunct: None,
            omitted_complement: 0,
            ellipsis: 0,
            head_forms: 0,
            after_unlike_omissions: false,
            conjunct_omitted: None,
            unlike_omissions: false,
            preposition: None,
            extensions: 0,
            coordinator: None,
            coordinated_head: false,
            head_coordination: false,
            only_head_coordination: false,
            conjunct_coordinators: [None; MAX_DISTINCT_COORDINATORS],
            other_preposition: None,
            mixed_prepositions: false,
            conjunct_hosts: u8::MAX,
            plain_hosts: u8::MAX,
            conjoined_host: false,
            unlike_values: 0,
            nonlast: [FeatureValue::UNSET; MAX_RESOLUTIONS],
            last: [FeatureValue::UNSET; MAX_RESOLUTIONS],
            joint_hits: 0,
            host_extensions: 0,
            adjunct_extension: None,
            internal_comma: false,
            open_comma: false,
            forced: false,
            unwritten_period: false,
            unclosed: false,
            headed_link: None,
            cluster: None,
            bare: Bare::NONE,
            head_lexeme: None,
            remnant: None,
            shared_head: None,
            cluster_preposition: None,
            remnant_seen: false,
            remnant_anchor: None,
        },
    })
}

/// Whether a phrase begins with a fronted dependent of its head, which is its
/// last child (`Summary::fronted`). The order of the variants is part of
/// `Summary`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Fronting {
    /// It does not.
    None,
    /// One set off by a comma (*When …, …*, *Until end of turn, …*), or a
    /// dependent other than a Predicate Adjunct preposed without one. It is
    /// never the initial conjunct of a coordination undelimited before its
    /// coordinator: the fronted dependent scopes over the whole coordination.
    Wide,
    /// A Predicate Adjunct preposed without a comma (*instead put …*,
    /// *instead puts …*). Its scope is ambiguous between the initial conjunct
    /// of a coordination and the whole coordination (*Yesterday Ed was taken
    /// ill and the lecture's been cancelled*; CGEL, Ch. 15 §1.3.4,
    /// pp. 1285–1286), so it may be that initial conjunct.
    Ambiguous,
}

/// The finite grammatical summary of a completed Constituent: everything a
/// parent may inspect. Two Constituents with equal summaries are packed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[expect(clippy::struct_excessive_bools, reason = "independent packed facts")]
pub(crate) struct Summary {
    pub first_glues: bool,
    pub last_glues: bool,
    /// The left edge of its first word, which the word before it agrees
    /// with ([`sylex_lexicon::RightEdge::agrees`]); `None` when it has no
    /// word.
    pub first_edge: Option<sylex_lexicon::LeftEdge>,
    /// The right edge of its last word, which the word after it agrees with.
    pub last_edge: Option<sylex_lexicon::RightEdge>,
    /// How it opens, which its position checks.
    pub opening: Opening,
    /// Feature values in Category feature order; slots past the Category's
    /// features stay unset.
    pub values: [FeatureValue; MAX_CATEGORY_FEATURES],
    pub gap: Gap,
    /// Its Gap is at its right edge: no child follows it, here or in the
    /// child it comes from. Right Node Raising shares only such a Gap.
    pub gap_right_edge: bool,
    /// The preposition of an extracted Preposition Phrase Complement.
    pub gap_preposition: Option<Lexeme>,
    /// Its lexical leaves, counted to two.
    pub leaves: u8,
    /// It ends in its head (through conjuncts of a coordination), not in a
    /// dependent of it: its last child is in the head line and itself ends in
    /// its head (a leaf does).
    pub head_final: bool,
    /// It ends in its head, undetermined (and, for a coordination, every
    /// conjunct does): no `Determiner` stands on its head line, and its last
    /// child is in the head line and itself `bare_head`.
    pub bare_head: bool,
    /// The preposition of the Preposition Phrase dependent it ends in, through
    /// its head line, when that preposition closes some `unextended` child.
    pub final_preposition: Option<Lexeme>,
    /// The Category of the trailing Adjunct it ends in, through its edge line
    /// (`Production::edge_line`), when that Category closes some
    /// `unadjoined` child.
    pub final_adjunct: Option<u16>,
    /// The Complements omitted by the bare non-verbal phrase it ends in,
    /// through any of its last children, as an index into
    /// [`CompiledGrammar::extension_sets`] (0: none). A Preposition Phrase
    /// right after it headed by one of their prepositions is that head's
    /// Complement, never a dependent of another host
    /// (`docs/design/english.md` §8).
    pub omitted_complement: u16,
    /// The word forms the omitted Complement of the ellipsis site it ends in
    /// takes, through its last child, as a mask (0: none;
    /// `ProductionDecl::elliptical`): before any punctuation, only a
    /// coordinator or a head it is a dependent of follows it, whose
    /// `head_forms` are none of these.
    pub ellipsis: u16,
    /// The word forms of every word spelled like its lexical head, reached
    /// through its head or a coordination's first conjunct, as a mask
    /// (`LeafFacts::forms`).
    pub head_forms: u16,
    /// It is a coordination of undetermined bare-head phrases whose conjuncts
    /// omit unlike Complements, the last conjunct's `omitted_complement`: a
    /// Preposition Phrase right after it may be a dependent of the whole
    /// coordination, which cannot share it as a Complement (*the type and
    /// amount of mana*), but of no other host.
    pub unlike_omissions: bool,
    /// The preposition and complement frame heading a Preposition Phrase.
    pub preposition: Option<(Lexeme, usize)>,
    /// A coordination's second preposition (*into your hand or onto the
    /// battlefield*).
    pub other_preposition: Option<Lexeme>,
    /// A serial tail's last conjunct's value of each resolved feature.
    pub last_resolved: [FeatureValue; MAX_RESOLUTIONS],
    /// An unlike coordination's or serial tail's conjuncts' values of the
    /// unlike feature, as a mask (`Production::unlike`); 0 for any other
    /// phrase.
    pub unlike_values: u64,
    /// Complements that would extend this phrase's selected frame, as an
    /// index into [`CompiledGrammar::extension_sets`].
    pub extensions: u16,
    /// The coordinator of a coordination, or the identity of the Linked
    /// Coordination that stands for one.
    pub coordinator: Option<CoordinatorId>,
    /// It is a coordination, or a post-dependent Production over one that
    /// leaves it a coordination (`Shape::keeps_coordination`) for the
    /// `coordinated` and `uncoordinated` children (*[draw a card and discard a
    /// card] at …* is not an uncoordinated Verb Phrase).
    pub coordinated: bool,
    /// It is a coordination, or its head is one, through head children: a
    /// coordinated phrase is heavy (`Weight::Postposable`).
    pub head_coordination: bool,
    /// It is a coordination, or a Production whose only child is its head
    /// over one (*[all permanents or a color ...]* under
    /// `NonNumericComplement`): never a phrase that only contains one, as a
    /// Noun Phrase over a coordinated Nominal (*a creature or planeswalker*).
    pub whole_coordination: bool,
    /// The Host Categories it may modify unselected: a Preposition Phrase's
    /// frame's, every conjunct's for a coordination, any for the rest.
    pub hosts: u8,
    /// No Nominal hosts it unselected: a Preposition Phrase whose frame's
    /// Host Categories exclude a Nominal (*at random*), or a phrase that is
    /// only such a phrase (`Bare::unforced`).
    pub nominal_unhosted: bool,
    /// Whether it begins with a fronted dependent of its head, and how that
    /// dependent scopes over a coordination it would begin. Such a Clause
    /// hosts no comma-set post-dependent, which attaches inside the fronted
    /// one (`docs/design/english.md` §8).
    pub fronted: Fronting,
    /// It contains a delimiting comma that no bracket closes: it is, or has at
    /// any depth, the host of a comma-set post-dependent (*[this creature
    /// dies, if …]*, *[X is half …, rounded down]*). Only a stronger indicator
    /// absorbs such a comma's closing mark (CGEL, Ch. 20 §3.2.5, p. 1745
    /// [25]), so it runs to the end of its sentence, and a dependent set off
    /// before its head by a comma ends at that comma, so it never contains
    /// one: an intervening *if* follows the trigger condition, never inside
    /// it (`docs/oracle-style-guide.md` §3, Commas and the serial comma; §8,
    /// Intervening if; [CR#603.4]). A dependent after its head, which runs to
    /// the end of its sentence, may contain one (*unless [its controller pays
    /// {X}, where X is …]*). The comma of a coordination delimited before its
    /// coordinator separates coordinates rather than delimiting one, and
    /// needs no closing mark (CGEL, Ch. 20 §3.1, pp. 1736–1737 [3]–[4];
    /// §3.2.1, p. 1739), so it sets no bit (*If [A are attacking, and you
    /// both own and control them], …*).
    pub internal_comma: bool,
    /// It ends in a comma-set post-dependent, through its last child: what
    /// follows it begins with a mark, which closes that comma, since a comma
    /// that sets off a dependent is closed by the next mark or runs to the
    /// end of its sentence (CGEL, Ch. 20 §3.2.5, p. 1745 [25]). No sibling
    /// follows it unpunctuated: *from black if [you control a Swamp, from
    /// red] if …* is no Cluster.
    pub open_comma: bool,
    /// It is an uncoordinated auxiliary Verb Phrase (`Production::auxiliary`):
    /// it hosts no post-verbal Adjunct, which attaches to the lexical
    /// participial Verb Phrase it ends in (`docs/design/english.md` §8).
    pub auxiliary: bool,
    /// It ends in a period that is not written: a quotation's final period
    /// (`chart::quotations`), or one after a word whose spelling ends in
    /// terminal punctuation (*Exile Blood for the Blood God!*): no other
    /// period follows it there. A keyword's
    /// verbal cost ends in its own period (`docs/oracle-style-guide.md` §3,
    /// Periods), so without this *have "Ward—Pay 2 life."* would also read as
    /// *Ward—Pay 2 life..*, a cost sentence and a keyword line each ending in
    /// the one unwritten period.
    pub unwritten_period: bool,
    /// It ends, through its last child, in a phrase opened by a dash that
    /// only its sentence's end closes (`ProductionDecl::unclosed`): nothing
    /// but a period follows it.
    pub unclosed: bool,
    /// It is only a conjunct of a coordination other than the last
    /// (`SchemaDecl::Adjunct::nonfinal`): no other position takes it.
    pub nonfinal: bool,
    /// The identity of the Linked Coordination it is, or that its head line
    /// ends in through `head` children (*then [draw a card, then discard a
    /// card]*). No Conjunct of a Linked Coordination carries the
    /// coordination's own: a chain linked by one connective has one tree,
    /// its serial (`Production::link`).
    pub link: Option<LinkedCoordination>,
    /// The frame, plus one, of the Complement Cluster it is, or of the first
    /// one its head line holds (0: none). The Clusters of one coordination
    /// are parallel: their frames have one shape
    /// (`CompiledGrammar::cluster_shapes`), and the first is the head's
    /// (`grammar::ClustersDecl`).
    pub cluster: u16,
    /// Another frame, plus one, of a Cluster its head line holds (0: none):
    /// the head must declare it too (*has trample ..., and "{B}: Regenerate
    /// this creature" ...*).
    pub cluster_other: u16,
    /// The bare Complement Clusters, of a single Slot and no Adjunct, it is
    /// or its head line holds, and the Clusters with Adjuncts beside them
    /// (`Bare`).
    pub bare: Bare,
    /// The head elided from the Complement Cluster remnant it is or begins
    /// with, or that every Cluster of a serial tail's remnants elides
    /// (`grammar::ClustersDecl::remnants`): a Cluster after the first may
    /// begin with a remnant only of the head of the initial Cluster's first
    /// Slot (CGEL, Ch. 15 §4.3, p. 1343, note 65).
    pub remnant: Option<Elided>,
    /// The lexical head of a phrase a later Cluster's remnant may elide,
    /// through its head line (`RemnantRole::SharesHead`).
    pub shared_head: Option<Lexeme>,
    /// The preposition of an Adjunct Cluster's last Preposition Phrase, or
    /// the one every Cluster of a parallel coordination shares
    /// (`grammar::AdjunctClustersDecl`).
    pub cluster_preposition: Option<Lexeme>,
}

/// The bare Complement Clusters a phrase is or holds, of a single Slot and no
/// Adjunct, and the Clusters with Adjuncts beside them. A bare Cluster is
/// licensed only where it alone breaks the parallelism of a coordination
/// (CGEL, Ch. 15 §4.3, pp. 1341–1342, note 64): beside at least two Clusters
/// with Adjuncts, all ending in as many, and never last (*exiles [the top
/// card of their library], [a card at random from their graveyard], and [a
/// card at random from their hand]*). A coordination of single Objects is the
/// coordination of the Objects, never also of Clusters
/// (`docs/design/english.md` §8). Each Cluster with Adjuncts beside it
/// holds one that no Nominal hosts (*at random*), so it is a Cluster of
/// necessity: where every Adjunct could be its Object's Modifier, the
/// coordination of Objects is parallel, and a bare Cluster beside them is
/// the breach CGEL stars (Ch. 15 §4.3, p. 1341, [17iia]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Bare {
    /// It holds a bare Cluster.
    held: bool,
    /// Its last Cluster is bare.
    last: bool,
    /// The Clusters with Adjuncts it holds, up to two.
    adjoined: u8,
    /// How many Adjuncts each of those ends in (0: none held; `MIXED`: not
    /// all as many).
    adjuncts: u8,
    /// One of those holds no Adjunct that a Nominal cannot host.
    unforced: bool,
}

impl Bare {
    pub(crate) const NONE: Self = Self {
        held: false,
        last: false,
        adjoined: 0,
        adjuncts: 0,
        unforced: false,
    };
    const MIXED: u8 = u8::MAX;

    /// A Cluster of `children` dependents, `adjuncts` of them Adjuncts,
    /// `forced` if one of those no Nominal hosts.
    fn cluster(children: usize, adjuncts: usize, forced: bool) -> Self {
        let bare = children == 1;
        Self {
            held: bare,
            last: bare,
            adjoined: u8::from(adjuncts > 0),
            adjuncts: u8::try_from(adjuncts).unwrap_or(Self::MIXED),
            unforced: adjuncts > 0 && !forced,
        }
    }

    /// These Clusters, then `next`'s.
    fn then(self, next: Self) -> Self {
        Self {
            held: self.held || next.held,
            last: next.last,
            adjoined: (self.adjoined + next.adjoined).min(2),
            adjuncts: match (self.adjuncts, next.adjuncts) {
                (0, n) | (n, 0) => n,
                (a, b) if a == b => a,
                _ => Self::MIXED,
            },
            unforced: self.unforced || next.unforced,
        }
    }

    /// Whether a coordination of these Clusters licenses its bare ones.
    fn licensed(self) -> bool {
        !self.held
            || (!self.last && self.adjoined >= 2 && self.adjuncts != Self::MIXED && !self.unforced)
    }
}

/// A lexical leaf as a child sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeafFacts {
    pub binding: sylex_lexicon::Binding,
    pub integer: Option<i32>,
    pub notation: Option<sylex_lexicon::Numeral>,
    pub lexeme: Option<Lexeme>,
    pub category: LexicalCategory,
    /// Feature values by feature id.
    pub features: [Option<u16>; MAX_FEATURES],
    /// The word forms of every Word spelled like it, ignoring case, as a
    /// mask (`form_bit`).
    pub forms: u16,
    /// It is another card's name: a card name [CR#201.1] the scope of the
    /// face whose text is parsed does not hold [CR#201.5]. Only
    /// [`Test::OtherCardName`] admits it, and that admits nothing else.
    pub other_card_name: bool,
}

pub(crate) enum ChildView<'a> {
    Node(&'a Summary),
    /// A lexical leaf, with its Adjacency Agreement edges and how it opens.
    Leaf(&'a LeafFacts, sylex_lexicon::Edges, Opening),
    Punct,
}

/// How a Constituent opens, which the capital its position calls for
/// checks (`ProductionDecl::capital`): a child that opens with a capital
/// opens so, any other child but the first with an atom in lower case, and
/// that first child as its mother does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Opening {
    /// It has no atom.
    Empty,
    /// Its first atom is a word that takes an initial capital, written
    /// without it.
    Lower,
    /// Its first atom is a word that takes an initial capital, written
    /// with it.
    Capital,
    /// Nothing outside it decides how it opens: its first atom is a mark or
    /// a word with one spelling, or the child it opens with opens with a
    /// capital.
    Fixed,
}

impl Opening {
    /// How a word written in `case` opens (`LexicalMatch::case`).
    pub(crate) fn of(case: Option<sylex_lexicon::LetterCase>) -> Self {
        match case {
            None => Self::Fixed,
            Some(sylex_lexicon::LetterCase::Lower) => Self::Lower,
            Some(sylex_lexicon::LetterCase::Capital) => Self::Capital,
        }
    }

    /// How a mother opens with a child that opens with `child` after children
    /// that open with `self`, at a position that opens with a capital or not;
    /// `None` where the child's case is not the one its position calls for.
    pub(crate) fn then(self, child: Self, capital: bool) -> Option<Self> {
        match (self, capital) {
            (Self::Empty, false) => Some(child),
            (Self::Empty, true) => match child {
                Self::Empty => Some(Self::Empty),
                Self::Lower => None,
                Self::Capital | Self::Fixed => Some(Self::Fixed),
            },
            (_, true) => (child != Self::Lower).then_some(self),
            (_, false) => (child != Self::Capital).then_some(self),
        }
    }
}

/// The partial state of a Production whose first children have been found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[expect(clippy::struct_excessive_bools, reason = "independent packed facts")]
pub(crate) struct State {
    pub first_glues: Option<bool>,
    pub last_glues: bool,
    /// The left edge of the first word so far (`Summary::first_edge`).
    first_edge: Option<sylex_lexicon::LeftEdge>,
    /// The right edge of the last word so far (`Summary::last_edge`).
    last_edge: Option<sylex_lexicon::RightEdge>,
    /// How the children so far open it (`Summary::opening`).
    opening: Opening,
    vars: [FeatureValue; MAX_PRODUCTION_VARS],
    gap: Option<Gap>,
    gap_right_edge: bool,
    shared_gap_right_edge: bool,
    gap_preposition: Option<Lexeme>,
    /// A conjunct has been taken into the coordination.
    conjoined: bool,
    /// Lexical leaves so far, counted to two.
    leaves: u8,
    /// The last head-line child so far ends in its head
    /// (`Production::track_head_final`).
    pub head_final: bool,
    /// Every head-line child so far is `Summary::bare_head`
    /// (`Production::track_head_final`).
    bare_head: bool,
    /// The preposition of the Preposition Phrase dependent the last child
    /// so far ends in (`Summary::final_preposition`).
    final_preposition: Option<Lexeme>,
    /// The trailing Adjunct the last child so far ends in
    /// (`Summary::final_adjunct`).
    final_adjunct: Option<u16>,
    /// The Complements omitted by the bare non-verbal phrase the last child
    /// so far ends in (`Summary::omitted_complement`).
    omitted_complement: u16,
    /// The omitted Complement forms of the ellipsis site the last child so
    /// far ends in (`Summary::ellipsis`).
    ellipsis: u16,
    /// The `lexical_head` child's `Summary::head_forms`.
    head_forms: u16,
    /// The last child so far is a coordination with unlike omissions
    /// (`Summary::unlike_omissions`).
    after_unlike_omissions: bool,
    /// The Complements the first conjunct so far omits.
    conjunct_omitted: Option<u16>,
    /// Some conjunct so far omits other Complements than the first.
    unlike_omissions: bool,
    preposition: Option<(Lexeme, usize)>,
    extensions: u16,
    coordinator: Option<CoordinatorId>,
    /// The head of a post-adjunct host's Production is a coordination
    /// (`Summary::coordinated`).
    coordinated_head: bool,
    /// The head child's `Summary::head_coordination`.
    head_coordination: bool,
    /// The head child's `Summary::whole_coordination`.
    only_head_coordination: bool,
    conjunct_coordinators: [Option<CoordinatorId>; MAX_DISTINCT_COORDINATORS],
    other_preposition: Option<Lexeme>,
    /// The conjuncts of a coordination have more than two prepositions, or
    /// one has none.
    mixed_prepositions: bool,
    /// The Host Categories every conjunct so far admits, or the head admits.
    conjunct_hosts: u8,
    /// The Host Categories some conjunct so far admits on its own, or the
    /// head admits (`frames::conjoined_hosts`).
    plain_hosts: u8,
    /// A conjunct admits a Predicate host only as a conjunct
    /// (`frames::conjoined_hosts`).
    conjoined_host: bool,
    /// The `unlike` feature's values of the conjuncts so far, as a mask.
    unlike_values: u64,
    /// The value of each resolved feature on the conjuncts but the last, and
    /// on the last.
    nonlast: [FeatureValue; MAX_RESOLUTIONS],
    last: [FeatureValue; MAX_RESOLUTIONS],
    /// The joint-exclusion conditions (as bits) that hold so far.
    joint_hits: u32,
    host_extensions: u16,
    adjunct_extension: Option<Extension>,
    /// A child so far carries `Summary::internal_comma` up.
    internal_comma: bool,
    /// The last child so far ends in a comma-set post-dependent
    /// (`Summary::open_comma`).
    open_comma: bool,
    /// A Cluster's Adjunct so far, or a sole child, is one no Nominal hosts
    /// (`Summary::nominal_unhosted`).
    forced: bool,
    /// The last child so far ends in a period not written
    /// (`Summary::unwritten_period`).
    pub unwritten_period: bool,
    /// The last child so far ends in an unclosed dash phrase
    /// (`Summary::unclosed`).
    unclosed: bool,
    /// The `head` child's `Summary::link`.
    headed_link: Option<LinkedCoordination>,
    /// The `Summary::cluster` and `Summary::cluster_other` of the head-line
    /// children so far.
    cluster: Option<(u16, u16)>,
    /// `Summary::bare` of the head-line children so far.
    bare: Bare,
    /// The lexeme of the lexical `head` child of a Production that requires
    /// a coordination of Complement Clusters (`Production::clusters`).
    head_lexeme: Option<Lexeme>,
    /// The head elided from the remnant it is or begins with
    /// (`Summary::remnant`).
    remnant: Option<Elided>,
    /// `Summary::shared_head` so far.
    shared_head: Option<Lexeme>,
    /// `Summary::cluster_preposition` so far.
    cluster_preposition: Option<Lexeme>,
    /// A coordination's initial conjunct has been taken (`remnant_anchor`).
    remnant_seen: bool,
    /// The `Summary::shared_head` of a coordination's initial conjunct that
    /// is no remnant (`Summary::remnant`).
    remnant_anchor: Option<Lexeme>,
}

impl Production {
    /// The position, among its symbols, of the last child before `dot`: a
    /// mark is no child.
    pub(crate) fn child_before(&self, dot: usize) -> Option<usize> {
        self.rhs[..dot]
            .iter()
            .rposition(|symbol| !matches!(symbol, Symbol::Punct(..)))
    }

    pub(crate) fn boundaries(&self, position: usize, child: &ChildView<'_>) -> (bool, bool) {
        use crate::reading::glues_after;
        use crate::reading::glues_before;
        let (mut first, mut last) = match child {
            ChildView::Node(summary) => (summary.first_glues, summary.last_glues),
            ChildView::Leaf(leaf, ..) => (glues_before(leaf.binding), glues_after(leaf.binding)),
            ChildView::Punct => match self.rhs[position] {
                Symbol::Punct(_, binding, _) => (glues_before(binding), glues_after(binding)),
                _ => (false, false),
            },
        };
        if let Symbol::Cat {
            binding: Some(binding),
            ..
        } = self.rhs[position]
        {
            first |= glues_before(binding);
            last |= glues_after(binding);
        }
        (first, last)
    }

    pub(crate) fn spacing_matches(
        &self,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
        gap: &[char],
    ) -> bool {
        if state.first_glues.is_none() {
            return true;
        }
        let (first, _) = self.boundaries(position, child);
        if state.last_glues || first { gap.is_empty() } else { gap == [' '] }
    }

    pub fn start(&self) -> State {
        self.start
    }

    /// Records the head's and an adjunct's frame facts for same-head
    /// complement priority, or `None` when a predicative Complement is a
    /// Preposition Phrase a rival frame selects.
    fn priority(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        summary: &Summary,
    ) -> Option<()> {
        if self.predicative == Some(position)
            && let Some((p, _)) = summary.preposition
            && grammar
                .extension_set(next.extensions)
                .contains(&Extension::Predicative(p))
        {
            return None;
        }
        if self.head == Some(position) {
            next.head_coordination = summary.head_coordination;
            next.only_head_coordination = summary.whole_coordination;
            next.preposition = summary.preposition;
            next.other_preposition = summary.other_preposition;
            next.extensions = summary.extensions;
            next.conjunct_hosts = summary.hosts;
            next.plain_hosts = summary.hosts;
        }
        if let Some((host, adjunct, _)) = self.host_check {
            if position == host {
                next.host_extensions = summary.extensions;
            }
            if position == adjunct {
                let (extension, kind) = match (summary.preposition, self.adjunct_slot) {
                    (Some((p, frame)), _) => (
                        Some(Extension::Preposition(
                            p,
                            Some(grammar.preposition_frames[frame].complement.kind()),
                        )),
                        Some(SlotKind::PP),
                    ),
                    (None, Some(kind)) => (Some(Extension::Slot(kind)), Some(kind)),
                    (None, None) => (None, None),
                };
                next.adjunct_extension = extension.filter(|_| {
                    !self.adjunct_unfit.iter().any(|&(k, feature, value)| {
                        Some(k) == kind
                            && self.slots[position].node_value(summary, feature) == Some(value)
                    })
                });
            }
        }
        Some(())
    }

    /// Tracks whether the last head-line child so far ends in its head: a
    /// head-line child sets it (a leaf to true, a phrase to its own
    /// `head_final`), and every other child, punctuation and a coordinator
    /// leaf included, leaves it as it was. The item before a coordination's
    /// last conjunct therefore records whether the first ended in its head
    /// (the Ranking Key's Attachment Class reads it there); the last conjunct
    /// then sets it. [`Production::finish`] reports it only when the last
    /// non-punctuation child is in the head line.
    fn track_head_final(&self, next: &mut State, position: usize, child: &ChildView<'_>) {
        if !self.head_line[position] {
            return;
        }
        match child {
            ChildView::Node(summary) => {
                next.head_final = summary.head_final;
                next.bare_head &= summary.bare_head;
            }
            ChildView::Leaf(..) => next.head_final = true,
            ChildView::Punct => {}
        }
    }

    /// The preposition of the Preposition Phrase dependent the phrase so far
    /// ends in.
    fn track_final_preposition(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) {
        match child {
            ChildView::Node(summary) => {
                next.final_preposition = if self.head_line[position] {
                    summary.final_preposition
                } else {
                    summary
                        .preposition
                        .map(|(p, _)| p)
                        .filter(|p| grammar.unextended_prepositions.contains(p))
                };
            }
            ChildView::Leaf(..) => next.final_preposition = None,
            ChildView::Punct => {}
        }
    }

    /// The Category of the trailing Adjunct the phrase so far ends in. A
    /// trailing Adjunct is the last child, named `adjunct`, of a Production
    /// whose head precedes it with no comma between: an unselected post-head
    /// dependent, not one set off.
    fn track_final_adjunct(&self, next: &mut State, position: usize, child: &ChildView<'_>) {
        match child {
            ChildView::Node(summary) => {
                next.final_adjunct = if self.edge_line[position] {
                    summary.final_adjunct
                } else {
                    self.slots[position].trailing_adjunct
                };
            }
            ChildView::Leaf(..) => next.final_adjunct = None,
            ChildView::Punct => {}
        }
    }

    /// The omitted Complement forms of the ellipsis site the phrase so far
    /// ends in, or `None` if child `position` comes right after one and is
    /// neither the head the phrase ending in it is a dependent of nor a
    /// coordinator, or could be the omitted Complement: a word spelled like
    /// its lexical head has one of those forms (`ProductionDecl::elliptical`;
    /// CGEL Ch. 17 §7.1, p. 1519). Punctuation ends the site's reach.
    fn track_ellipsis(
        &self,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        let head_forms = match child {
            ChildView::Node(summary) => summary.head_forms,
            ChildView::Leaf(leaf, ..) => leaf.forms,
            ChildView::Punct => 0,
        };
        if self.lexical_head == Some(position) {
            next.head_forms = head_forms;
        }
        match child {
            ChildView::Punct => next.ellipsis = 0,
            _ if next.ellipsis != 0
                && ((self.head != Some(position) && self.coordinator != Some(position))
                    || head_forms & next.ellipsis != 0) =>
            {
                return None;
            }
            ChildView::Node(summary) => next.ellipsis = summary.ellipsis,
            ChildView::Leaf(..) => next.ellipsis = 0,
        }
        Some(())
    }

    /// The Complement Cluster frames the head line so far holds, or `None`
    /// if child `position` is in the head line with Clusters of another shape
    /// (the Clusters of one coordination are parallel; CGEL, Ch. 15 §4.3,
    /// p. 1341), or is the coordination of Clusters its Production requires
    /// and does not begin with one of the head's frame, holds one of a
    /// frame the head does not declare (`Summary::cluster`), or holds a bare
    /// Cluster it does not license (`Bare::licensed`).
    fn track_cluster(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        let (cluster, other, bare) = match child {
            ChildView::Node(summary) => (summary.cluster, summary.cluster_other, summary.bare),
            ChildView::Leaf(..) => (0, 0, Bare::NONE),
            ChildView::Punct => return Some(()),
        };
        let shape = |c: u16| (c != 0).then(|| grammar.cluster_shapes[usize::from(c - 1)]);
        if let Some((at, frame)) = self.clusters
            && at == position
        {
            let declared = next.head_lexeme.map(|head| &grammar.facts(head).frames)?;
            if cluster != frame + 1
                || !bare.licensed()
                || (other != 0
                    && (!declared.contains(&usize::from(other - 1))
                        || shape(other) != shape(cluster)))
            {
                return None;
            }
        }
        if self.head_line[position] {
            next.bare = next.bare.then(bare);
            next.cluster = Some(match next.cluster {
                None => (cluster, other),
                Some((first, held)) => {
                    if shape(first) != shape(cluster) {
                        return None;
                    }
                    let mut held = held;
                    for c in [cluster, other] {
                        if c == 0 || c == first || c == held {
                            continue;
                        }
                        if held != 0 {
                            return None;
                        }
                        held = c;
                    }
                    (first, held)
                }
            });
        }
        Some(())
    }

    /// The remnant facts of the phrase so far (`Summary::remnant`,
    /// `Summary::shared_head`), or `None` if the child is a remnant that
    /// begins a coordination, or elides another head than the initial
    /// conjunct's.
    fn track_remnant(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        let (remnant, shared, preposition, cluster_preposition) = match child {
            ChildView::Node(summary) => (
                summary.remnant,
                summary.shared_head,
                summary.preposition.map(|(p, _)| p),
                summary.cluster_preposition,
            ),
            ChildView::Leaf(leaf, ..) => (None, leaf.lexeme, None, None),
            ChildView::Punct => return Some(()),
        };
        if self.remnant == RemnantRole::SharesHead && self.head == Some(position) {
            next.shared_head = shared;
            return Some(());
        }
        if self.cluster.is_some() {
            if position == 0 {
                next.remnant = remnant;
                next.shared_head = shared;
            } else if position + 1 == self.rhs.len() {
                next.cluster_preposition = preposition;
            }
            return Some(());
        }
        if !self.head_line[position] || matches!(child, ChildView::Leaf(..)) {
            return Some(());
        }
        if self.head_line.iter().filter(|h| **h).count() < 2 {
            if self.head == Some(position) {
                next.remnant = remnant;
                next.shared_head = shared;
            }
            return Some(());
        }
        if !next.remnant_seen {
            // A coordination's initial conjunct is no remnant; a serial
            // tail's first conjunct is a middle one and may be.
            if remnant.is_some() && !self.tail {
                return None;
            }
            next.remnant_seen = true;
            next.remnant = remnant;
            next.remnant_anchor = if remnant.is_some() { None } else { shared };
            if self.parallel {
                // A parallel coordination shares its conjuncts' head and
                // last preposition.
                next.shared_head = next.remnant_anchor;
                next.cluster_preposition = cluster_preposition;
            }
            return Some(());
        }
        if self.parallel && cluster_preposition != next.cluster_preposition {
            return None;
        }
        match (remnant, next.remnant_anchor, next.remnant) {
            (None, Some(anchor), _) if self.parallel => (shared == Some(anchor)).then_some(()),
            (None, None, Some(first)) if self.parallel => shared
                .is_some_and(|head| first.recovered_by(grammar, head))
                .then_some(()),
            (None, None, None) if self.parallel => None,
            (None, ..) => Some(()),
            (Some(elided), Some(head), _) => elided.recovered_by(grammar, head).then_some(()),
            (Some(elided), None, Some(first)) => (elided == first).then_some(()),
            (Some(_), None, None) => None,
        }
    }

    /// The Complements omitted by the bare non-verbal phrase the phrase so
    /// far ends in, or `None` if the child is a Preposition Phrase right after
    /// one and headed by a preposition it omits: that phrase is the bare
    /// head's own Complement, which no other host may take. Only a coordination
    /// whose conjuncts omit unlike Complements may take it as its own
    /// dependent.
    fn track_omitted_complement(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<()> {
        match child {
            ChildView::Node(summary) => {
                if self.coordinator.is_some() && self.head_line[position] {
                    let first = *next
                        .conjunct_omitted
                        .get_or_insert(summary.omitted_complement);
                    next.unlike_omissions |= summary.unlike_omissions
                        || !grammar.like_omissions(first, summary.omitted_complement);
                }
                if next.omitted_complement != 0 && !next.after_unlike_omissions {
                    let omitted = grammar.extension_set(next.omitted_complement);
                    let heads = [
                        summary.preposition.map(|(p, frame)| {
                            Extension::Preposition(
                                p,
                                Some(grammar.preposition_frames[frame].complement.kind()),
                            )
                        }),
                        summary
                            .other_preposition
                            .map(|p| Extension::Preposition(p, None)),
                    ];
                    if heads.into_iter().flatten().any(|phrase| {
                        omitted.iter().any(|selected| match (selected, &phrase) {
                            // A second conjunct's Complement kind is not
                            // recorded: its preposition is the phrase's.
                            (Extension::Preposition(p, _), Extension::Preposition(q, None)) => {
                                p == q
                            }
                            _ => selected.selects(phrase),
                        })
                    }) {
                        return None;
                    }
                }
                next.omitted_complement = summary.omitted_complement;
                next.after_unlike_omissions = summary.unlike_omissions;
            }
            ChildView::Leaf(..) | ChildView::Punct => {
                next.omitted_complement = 0;
                next.after_unlike_omissions = false;
            }
        }
        Some(())
    }

    /// The lexical leaves a child adds, counted to two.
    fn leaves(child: &ChildView<'_>) -> u8 {
        match child {
            ChildView::Node(summary) => summary.leaves,
            ChildView::Leaf(..) => 1,
            ChildView::Punct => 0,
        }
    }

    /// Whether a child is the weight its slot requires.
    fn weighs(weight: Option<Weight>, child: &ChildView<'_>) -> bool {
        let leaves = Self::leaves(child);
        match weight {
            Some(Weight::Light) => leaves == 1,
            Some(Weight::Heavy) => leaves >= 2,
            Some(Weight::Postposable) => {
                matches!(child, ChildView::Node(s) if !s.head_final || s.head_coordination)
            }
            None => true,
        }
    }

    /// Takes child `position`'s Gap into `next` under its Gap Rule, or
    /// `None` if the rule rejects it.
    fn take_gap(&self, next: &mut State, position: usize, gap: Gap) -> Option<()> {
        let open = next.gap.is_some_and(|g| g != Gap::None);
        match self.gaps[position] {
            GapRule::Closed => (gap == Gap::None).then_some(()),
            GapRule::Require(rule) => (gap == rule).then_some(()),
            GapRule::Raise(rule) => {
                if gap != rule || open {
                    return None;
                }
                next.gap = Some(rule);
                Some(())
            }
            GapRule::Transparent if gap == Gap::None => Some(()),
            GapRule::Transparent => {
                if open {
                    return None;
                }
                next.gap = Some(gap);
                Some(())
            }
            GapRule::Shared => match next.gap {
                Some(g) if g != gap => None,
                _ => {
                    next.gap = Some(gap);
                    Some(())
                }
            },
        }
    }

    /// Takes a conjunct's Host Categories and prepositions into a
    /// coordination: Preposition Phrases keep up to two prepositions (*from
    /// black and from red*, *into your hand or onto the battlefield*).
    fn conjoin(next: &mut State, summary: &Summary) {
        let (hosts, conjoined) = crate::frames::conjunct_hosts(summary.hosts);
        next.conjunct_hosts &= hosts;
        next.conjoined_host |= conjoined;
        next.plain_hosts =
            if next.conjoined { next.plain_hosts | summary.hosts } else { summary.hosts };
        // Conjuncts that would take the same Complements leave the
        // coordination extendable by them (*the top or bottom of their
        // library* takes its *of*-phrase as a shared Complement, never as an
        // Adjunct).
        if !next.conjoined {
            next.extensions = summary.extensions;
            next.conjoined = true;
        } else if next.extensions != summary.extensions {
            next.extensions = 0;
        }
        let own = [
            summary.preposition.map(|(p, _)| p),
            summary.other_preposition,
        ];
        if own[0].is_none() {
            next.mixed_prepositions = true;
        }
        for q in own.into_iter().flatten() {
            match (next.preposition, next.other_preposition) {
                (None, _) => next.preposition = summary.preposition,
                (Some((p, _)), _) if p == q => {}
                (_, Some(o)) if o == q => {}
                (_, None) => next.other_preposition = Some(q),
                _ => next.mixed_prepositions = true,
            }
        }
    }

    /// Gathers the unlike feature's values of the conjuncts, a serial tail's
    /// from its Summary; `None` if the whole coordination's valued conjuncts
    /// are all of one kind: unlike conjuncts need only not all be alike
    /// (CGEL, Ch. 15 §3.1, p. 1323 [1]; §3.2, p. 1326).
    fn unlike_conjuncts(
        &self,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) -> Option<()> {
        let Some(unlike) = self.unlike else {
            return Some(());
        };
        if position != unlike.first && position != unlike.last {
            return Some(());
        }
        next.unlike_values |= match child {
            ChildView::Node(summary) if unlike.rest && position == unlike.last => {
                summary.unlike_values
            }
            // A conjunct with no value is of no kind a derived unlike slot
            // selects (`unlike_slot_admits`), and differs from none.
            _ => lookup(unlike.feature).map_or(UNVALUED_CONJUNCT, |value| 1u64 << value),
        };
        if position == unlike.last
            && !self.tail
            && (next.unlike_values & !UNVALUED_CONJUNCT).count_ones() < 2
        {
            return None;
        }
        Some(())
    }

    /// Whether a slot that is a derived unlike Frame Slot admits the
    /// coordination: none of its conjuncts' values of the unlike feature is
    /// outside the slot's mask, and every conjunct has one
    /// (`ChildSlot::unlike_mask`; the mask never holds
    /// [`UNVALUED_CONJUNCT`]).
    fn unlike_slot_admits(slot: &ChildSlot, summary: &Summary) -> bool {
        slot.unlike_mask
            .is_none_or(|mask| summary.unlike_values != 0 && summary.unlike_values & !mask == 0)
    }

    /// Records the joint-exclusion conditions child `position` meets, or
    /// `None` once every condition of one exclusion holds.
    fn joint_exclusion(
        &self,
        next: &mut State,
        position: usize,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) -> Option<()> {
        if self.joint_conditions.is_empty() {
            return Some(());
        }
        for (k, &(at, feature, value)) in self.joint_conditions.iter().enumerate() {
            if at == position && lookup(feature) == Some(value) {
                next.joint_hits |= 1 << k;
            }
        }
        let mut start = 0;
        for &end in &self.joint_exclusions {
            let mask = u32::try_from((1u64 << end) - (1u64 << start)).expect("validated");
            if next.joint_hits & mask == mask {
                return None;
            }
            start = end;
        }
        Some(())
    }

    /// Takes a child's values of the resolved features into the
    /// coordination: those of the conjuncts but the last (which agree by
    /// equation), or the last conjunct's.
    fn resolve(
        &self,
        next: &mut State,
        position: usize,
        child: &ChildView<'_>,
        lookup: &dyn Fn(usize) -> Option<u16>,
    ) {
        let nonlast = self.resolution_nonlast.contains(&position);
        let last = match self.resolution_last {
            Some((at, from_tail)) if at == position => Some(from_tail),
            _ => None,
        };
        for (i, &(feature, ..)) in self.resolutions.iter().enumerate() {
            if nonlast && let Some(value) = lookup(feature) {
                next.nonlast[i] = FeatureValue::new(value);
            }
            match (last, child) {
                (Some(true), ChildView::Node(summary)) => next.last[i] = summary.last_resolved[i],
                (Some(false), _) => {
                    next.last[i] = lookup(feature).map_or(FeatureValue::UNSET, FeatureValue::new);
                }
                _ => {}
            }
        }
    }

    /// The engine facts a lexical leaf child adds.
    fn accept_leaf(
        &self,
        grammar: &CompiledGrammar,
        next: &mut State,
        position: usize,
        leaf: &LeafFacts,
    ) {
        if self.coordinator == Some(position) {
            next.coordinator = leaf.lexeme.map(CoordinatorId::Lexeme);
        }
        // Only a head whose Complement Clusters it must declare is recorded,
        // so that homographs elsewhere still pack.
        if self.clusters.is_some() && self.head == Some(position) {
            next.head_lexeme = leaf.lexeme;
        }
        if let Symbol::Leaf(Test::PrepositionFrame(frame)) = &self.rhs[position] {
            next.preposition = leaf.lexeme.map(|l| (l, *frame));
        }
        let key = match (&self.rhs[position], leaf.lexeme) {
            (
                Symbol::Leaf(
                    Test::Frame(frame) | Test::FreeFrame(frame) | Test::ClassFrame { frame, .. },
                ),
                Some(lexeme),
            ) => Some((lexeme, Some(*frame))),
            (
                Symbol::Leaf(
                    Test::FreeLexical(_) | Test::FreeOptional(_) | Test::FreeNotActiveOnly(_),
                ),
                Some(lexeme),
            ) => Some((lexeme, None)),
            _ => None,
        };
        if let Some(key) = key {
            next.extensions = grammar.extensions.get(&key).copied().unwrap_or(0);
        }
        if self.host_check.is_some_and(|(host, _, _)| host == position) {
            next.host_extensions = next.extensions;
        }
    }

    /// Whether the Production begins with a fronted dependent of its head
    /// (`Shape::fronts`).
    pub(crate) fn fronts(&self) -> bool {
        self.shape.fronts
    }

    /// Whether a fronted dependent's scope rules this child out: a fronted
    /// dependent scopes over a comma-set post-dependent of its Clause, never
    /// inside the post-dependent's host, and, unless it is a Predicate
    /// Adjunct preposed without a comma (`Fronting::Ambiguous`), over
    /// a whole coordination, never its initial conjunct
    /// (`docs/design/english.md` §8). The initial conjunct of a
    /// coordination delimited before its coordinator may have its own (*[If
    /// you do, …], and [if you don't, …]*), and so may a middle conjunct of a
    /// serial one, since a fronted dependent there cannot scope over the
    /// whole (*C1, [if …, C2], and C3*).
    fn comma_scope_excludes(&self, position: usize, summary: &Summary) -> bool {
        let first_conjunct = self.coordinator.is_some()
            && !self.delimits
            && !self.tail
            && self.roles[position] == ChildRole::First;
        match summary.fronted {
            Fronting::None => false,
            Fronting::Wide => first_conjunct || (position == 0 && self.hosts_comma_set_dependent()),
            Fronting::Ambiguous => position == 0 && self.hosts_comma_set_dependent(),
        }
    }

    /// Whether the Production hosts a comma-set post-dependent
    /// (`Shape::hosts_comma_set_dependent`).
    pub(crate) fn hosts_comma_set_dependent(&self) -> bool {
        self.shape.hosts_comma_set_dependent
    }

    /// Whether child `position` is set off before the head by a comma.
    fn set_off_before_head(&self, position: usize) -> bool {
        self.slots[position].set_off_before_head
    }

    /// Whether child `position` is a post-verbal Adjunct's host
    /// (`Shape::post_adjunct_host`).
    pub(crate) fn post_adjunct_host(&self, position: usize) -> bool {
        position == 0 && self.shape.post_adjunct_host
    }

    /// Precomputes the Production's static structural and Ranking Key
    /// facts, once binding has settled it: `link` says it forms a Linked
    /// Conjunct, and `unadjoined` holds the Categories a Summary's
    /// `final_adjunct` records.
    pub(crate) fn seal(&mut self, link: bool, unadjoined: &std::collections::BTreeSet<u16>) {
        let count = self.rhs.len();
        let punct = |at: usize| matches!(self.rhs[at], Symbol::Punct(..));
        let comma = |at: usize| matches!(self.rhs[at], Symbol::Punct(',', ..));
        let children: Vec<usize> = (0..count).filter(|&at| !punct(at)).collect();
        let roles = &self.roles;
        let named = |role: ChildRole| children.iter().copied().find(|&at| roles[at] == role);
        let only = match children.as_slice() {
            [only] => Some(*only),
            _ => None,
        };
        let head_line_child = if link {
            children.iter().copied().find(|&at| Some(at) != self.head)
        } else {
            self.head
                .or(self.coordinator)
                .or(only)
                .or_else(|| named(ChildRole::Coordinator))
                .or_else(|| named(ChildRole::Marker))
                .or_else(|| {
                    children
                        .iter()
                        .copied()
                        .find(|&at| !roles[at].is_dependent())
                })
                .or_else(|| children.first().copied())
        };
        assert!(
            head_line_child.is_none_or(|at| !punct(at)),
            "production `{}`: its head line runs through a mark, which is no child",
            self.id
        );
        let post_head_of_lhs = self.head == Some(0)
            && self.coordinator.is_none()
            && count > 1
            && matches!(self.rhs[0], Symbol::Cat { category, .. } if category == self.lhs);
        let post_adjunct_host =
            post_head_of_lhs && (1..count).all(|at| punct(at) || roles[at] == ChildRole::Adjunct);
        let keeps_coordination = post_adjunct_host
            || (post_head_of_lhs
                && self.slots[0].coordinated
                && (1..count).all(|at| punct(at) || roles[at].is_unselected()));
        let shape = Shape {
            head_line_child,
            frame_production: self.frame.is_some() || children.len() == 1,
            fronts: self.coordinator.is_none()
                && self.head == Some(count - 1)
                && ((count > 2 && comma(count - 2))
                    || self
                        .host_check
                        .is_some_and(|(host, adjunct, _)| adjunct < host)),
            hosts_comma_set_dependent: self.head == Some(0) && count > 2 && comma(1),
            post_adjunct_host,
            keeps_coordination,
            ends_in_head_line: children.last().is_some_and(|&at| self.head_line[at]),
            undetermined_head_final: children.last().is_some_and(|&at| self.head_line[at])
                && !roles.contains(&ChildRole::Determiner),
        };
        let mut slots = std::mem::take(&mut self.slots);
        for (at, slot) in slots.iter_mut().enumerate() {
            let role = roles[at];
            let (before, after) = (&roles[..at], &roles[at + 1..]);
            slot.set_off_before_head = self
                .head
                .is_some_and(|head| at < head && (at + 1..head).any(comma));
            slot.trailing_adjunct = if role == ChildRole::Adjunct
                && self.coordinator.is_none()
                && self.head.is_some_and(|head| head < at)
                && !punct(at - 1)
                && let Symbol::Cat { category, .. } = self.rhs[at]
            {
                u16::try_from(category)
                    .ok()
                    .filter(|c| unadjoined.contains(c))
            } else {
                None
            };
            slot.anchor = match role {
                ChildRole::Head
                    if after.iter().any(|r| {
                        matches!(
                            r,
                            ChildRole::Adjunct | ChildRole::Modifier | ChildRole::Supplement
                        )
                    }) =>
                {
                    Some(Anchor::Head)
                }
                ChildRole::First if after.contains(&ChildRole::Last) => Some(Anchor::First),
                _ => None,
            };
            slot.attachment = AttachmentRole {
                dependents_after: u32::try_from(after.iter().filter(|r| r.is_unselected()).count())
                    .expect("a Production has few children"),
                after_head: before.contains(&ChildRole::Head),
                preposed_after_dependent: role.is_unselected()
                    && before.iter().any(|r| r.is_unselected())
                    && after.contains(&ChildRole::Head),
            };
        }
        self.slots = slots;
        self.shape = shape;
    }

    /// Whether child `position`'s slot admits the phrase: a selected `PP`
    /// slot its preposition, an unselected adjunct its Host Categories.
    fn slot_admits(&self, grammar: &CompiledGrammar, position: usize, summary: &Summary) -> bool {
        let Symbol::Cat {
            prepositions, host, ..
        } = &self.rhs[position]
        else {
            return true;
        };
        if let Some((list, complement)) = prepositions {
            let Some((prep, frame)) = summary.preposition.as_ref() else {
                return false;
            };
            let kind_selected = grammar.kind_selected[*frame];
            let frame = &grammar.preposition_frames[*frame];
            if !list.contains(prep)
                || complement.map_or(kind_selected, |kind| frame.complement.kind() != kind)
                || summary
                    .other_preposition
                    .is_some_and(|o| !list.contains(&o))
            {
                return false;
            }
        }
        !host.is_some_and(|kind| summary.hosts & crate::frames::host_bit(kind) == 0)
    }

    /// Whether `coordinator` is one a plain serial coordination's tail may
    /// not end with (`Production::foreign_coordinators`).
    fn foreign_coordinator(&self, coordinator: CoordinatorId) -> bool {
        match coordinator {
            CoordinatorId::Lexeme(lexeme) => self.foreign_coordinators.contains(&lexeme),
            CoordinatorId::Linked(_) => false,
        }
    }

    /// Whether child `position` may be, or be headed by, the Linked
    /// Coordination it is (`Summary::link`): not an `unlinked` child, nor a
    /// Conjunct of a Linked Coordination of the same identity.
    fn link_admits(&self, slot: &ChildSlot, position: usize, summary: &Summary) -> bool {
        let same_chain = self.link == summary.link && self.distinct_coordinator.contains(&position);
        summary.link.is_none() || !(slot.unlinked || same_chain)
    }

    /// A feature's value on a child in `slot`.
    fn child_value(slot: &ChildSlot, child: &ChildView<'_>, feature: usize) -> Option<u16> {
        match child {
            ChildView::Node(summary) => slot.node_value(summary, feature),
            ChildView::Leaf(leaf, ..) => leaf.features[feature],
            ChildView::Punct => None,
        }
    }

    /// Whether child `position` passes the gates of its slot, which depend
    /// on the child alone, not on the children before it.
    fn admits(&self, grammar: &CompiledGrammar, position: usize, child: &ChildView<'_>) -> bool {
        let slot = &self.slots[position];
        if !Self::weighs(slot.weight, child)
            || slot
                .exclusions
                .iter()
                .any(|&(feature, value)| Self::child_value(slot, child, feature) == Some(value))
            || slot
                .carried
                .iter()
                .any(|&(feature, value)| Self::child_value(slot, child, feature) != Some(value))
        {
            return false;
        }
        let ChildView::Node(summary) = child else {
            return true;
        };
        // A free child does not begin with an atom glued to the one before.
        let free = matches!(
            self.rhs[position],
            Symbol::Cat {
                binding: Some(sylex_lexicon::Binding::Free),
                ..
            }
        );
        let foreign = summary
            .coordinator
            .is_some_and(|c| self.foreign_coordinator(c));
        let unpaired = self
            .paired_coordinator
            .is_some_and(|p| summary.coordinator != Some(CoordinatorId::Lexeme(p)));
        let closed_preposition = slot.unextended.as_ref().is_some_and(|closed| {
            summary
                .final_preposition
                .is_some_and(|p| closed.contains(&p))
        });
        let closed_adjunct = slot
            .unadjoined
            .as_ref()
            .is_some_and(|closed| summary.final_adjunct.is_some_and(|c| closed.contains(&c)));
        let rejected = (free && summary.first_glues)
            || closed_preposition
            || closed_adjunct
            || !self.slot_admits(grammar, position, summary)
            || !Self::unlike_slot_admits(slot, summary)
            || (summary.nonfinal && !slot.nonfinal_conjunct)
            || (slot.uncoordinated && summary.coordinated)
            || !self.link_admits(slot, position, summary)
            || (slot.coordinated && !summary.coordinated)
            || slot
                .whole_coordination
                .is_some_and(|whole| whole != summary.whole_coordination)
            // A comma that no bracket closes stays inside every phrase that
            // contains it, to the end of its sentence.
            || (summary.internal_comma && self.set_off_before_head(position))
            || self.comma_scope_excludes(position, summary)
            || (summary.auxiliary && self.post_adjunct_host(position))
            || (self.requires_coordination == Some(position)
                && (summary.coordinator.is_none() || !summary.gap_right_edge))
            || (self.coordinator == Some(position) && (foreign || unpaired));
        !rejected
    }

    /// Adds child `position` to `state`, or `None` if it does not fit. The
    /// gates of its slot ([`Self::admits`]) are checked before the State is
    /// copied.
    pub fn accept(
        &self,
        grammar: &CompiledGrammar,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<State> {
        if !self.admits(grammar, position, child) {
            return None;
        }
        let slot = &self.slots[position];
        let mut next = *state;
        if state.gap.is_some_and(|gap| gap != Gap::None) {
            next.gap_right_edge = false;
        }
        if state.unclosed && !matches!(self.rhs[position], Symbol::Punct('.', ..)) {
            return None;
        }
        next.unclosed = matches!(child, ChildView::Node(summary) if summary.unclosed);
        let (first, last) = self.boundaries(position, child);
        // A comma that sets off a post-dependent is closed by the mark after
        // it, a comma or a stronger indicator (*choose up to X, where X is
        // … —*), or else runs to the end of its sentence: no sibling follows
        // it unpunctuated (CGEL, Ch. 20 §3.2.5, p. 1745 [25]).
        if state.open_comma
            && !first
            && !matches!(self.rhs[position], Symbol::Punct(_, binding, _) if !crate::reading::glues_after(binding))
        {
            return None;
        }
        next.open_comma = false;
        next.first_glues.get_or_insert(first);
        next.last_glues = last;
        // Adjacency Agreement holds between each word and the next, whatever
        // Constituents they end and begin. A mark its Production declares
        // spoken (*a +8/+8 creature*) is a word; an empty child and any other
        // mark have none.
        let (first_edge, last_edge) = match child {
            ChildView::Node(summary) => (summary.first_edge, summary.last_edge),
            ChildView::Leaf(_, edges, _) => (Some(edges.left), Some(edges.right)),
            ChildView::Punct => {
                let Symbol::Punct(_, _, spoken) = self.rhs[position] else {
                    unreachable!("a punctuation child stands at a punctuation symbol");
                };
                spoken
                    .map(|onset| sylex_lexicon::Edges::plain(Some(onset)))
                    .map_or((None, None), |edges| (Some(edges.left), Some(edges.right)))
            }
        };
        if let (Some(before), Some(after)) = (state.last_edge, first_edge)
            && !before.agrees(after)
        {
            return None;
        }
        next.first_edge = state.first_edge.or(first_edge);
        next.last_edge = last_edge.or(state.last_edge);
        // The capitals of Oracle text are positional: a child opens with
        // one where the Production says so, and in lower case after the
        // mother's first atom.
        let opening = match child {
            ChildView::Node(summary) => summary.opening,
            ChildView::Leaf(_, _, opening) => *opening,
            ChildView::Punct => Opening::Fixed,
        };
        next.opening = state.opening.then(opening, slot.capital)?;
        next.unwritten_period =
            matches!(child, ChildView::Node(summary) if summary.unwritten_period);
        next.leaves = (next.leaves + Self::leaves(child)).min(2);
        self.track_head_final(&mut next, position, child);
        self.track_final_preposition(grammar, &mut next, position, child);
        self.track_final_adjunct(&mut next, position, child);
        self.track_omitted_complement(grammar, &mut next, position, child)?;
        self.track_ellipsis(&mut next, position, child)?;
        self.track_cluster(grammar, &mut next, position, child)?;
        self.track_remnant(grammar, &mut next, position, child)?;
        let lookup = |feature: usize| Self::child_value(slot, child, feature);
        self.unlike_conjuncts(&mut next, position, child, &lookup)?;
        self.joint_exclusion(&mut next, position, &lookup)?;
        self.resolve(&mut next, position, child, &lookup);
        for &(feature, var) in &self.child_vars[position] {
            if let Some(value) = lookup(feature) {
                match next.vars[var].get() {
                    Some(bound) if bound != value => return None,
                    _ => next.vars[var] = FeatureValue::new(value),
                }
            }
        }
        match child {
            ChildView::Node(summary) => {
                if summary.gap != Gap::None {
                    next.gap_right_edge = summary.gap_right_edge;
                    if matches!(self.gaps[position], GapRule::Shared) {
                        next.shared_gap_right_edge &= summary.gap_right_edge;
                    }
                }
                self.take_gap(&mut next, position, summary.gap)?;
                if let Some(p) = summary.gap_preposition {
                    if next.gap_preposition.is_some_and(|q| q != p) {
                        return None;
                    }
                    next.gap_preposition = Some(p);
                }
                if self.fills_preposition == Some(position)
                    && summary.preposition.map(|(p, _)| p) != next.gap_preposition
                {
                    return None;
                }
                if self.head == Some(position) {
                    next.headed_link = summary.link;
                }
                if position == 0 && self.shape.keeps_coordination {
                    next.coordinated_head = summary.coordinated;
                }
                next.internal_comma |= summary.internal_comma;
                next.open_comma = summary.open_comma
                    || (self.hosts_comma_set_dependent() && position == self.rhs.len() - 1);
                if summary.nominal_unhosted
                    && (self.rhs.len() == 1
                        || (self.cluster.is_some() && self.roles[position] == ChildRole::Adjunct))
                {
                    next.forced = true;
                }
                if self.distinct_coordinator.contains(&position)
                    && let Some(c) = summary.coordinator
                    && let Some(slot) = next.conjunct_coordinators.iter_mut().find(|s| s.is_none())
                {
                    *slot = Some(c);
                }
                if self.coordinator.is_some() && matches!(self.gaps[position], GapRule::Shared) {
                    Self::conjoin(&mut next, summary);
                }
                if self.chained == Some(position) && summary.coordinator != next.coordinator {
                    return None;
                }
                if self.coordinator == Some(position) {
                    next.coordinator = summary.coordinator;
                }
                self.priority(grammar, &mut next, position, summary)?;
            }
            ChildView::Leaf(leaf, ..) => self.accept_leaf(grammar, &mut next, position, leaf),
            ChildView::Punct => {}
        }
        Some(next)
    }

    /// The mother's feature values once every child has been accepted, and
    /// the last conjunct's value of each resolved feature a serial tail
    /// records.
    fn mother_values(
        &self,
        state: &State,
    ) -> (
        [FeatureValue; MAX_CATEGORY_FEATURES],
        [FeatureValue; MAX_RESOLUTIONS],
    ) {
        let mut values = [FeatureValue::UNSET; MAX_CATEGORY_FEATURES];
        for (value, var) in values.iter_mut().zip(&self.mother_vars) {
            *value = var.map_or(FeatureValue::UNSET, |v| state.vars[v]);
        }
        let mut last_resolved = [FeatureValue::UNSET; MAX_RESOLUTIONS];
        for (i, &(_, differ, at)) in self.resolutions.iter().enumerate() {
            match at {
                // The whole takes the common value, or `differ`.
                Some(at) => {
                    values[at] = match (state.nonlast[i].get(), state.last[i].get()) {
                        (Some(a), Some(b)) if a != b => FeatureValue::new(differ),
                        (None, _) => state.last[i],
                        _ => state.nonlast[i],
                    };
                }
                // A serial tail records its last conjunct's value.
                None => last_resolved[i] = state.last[i],
            }
        }
        (values, last_resolved)
    }

    /// The mother's summary once every child has been accepted.
    pub fn finish(&self, grammar: &CompiledGrammar, state: &State) -> Option<Summary> {
        if state.coordinator.is_some() && state.conjunct_coordinators.contains(&state.coordinator) {
            return None;
        }
        // It is itself a coordination.
        let coordination = self.link.is_some() || state.coordinator.is_some();
        if self.host_check.is_some()
            && let Some(e) = &state.adjunct_extension
            && grammar
                .extension_set(state.host_extensions)
                .iter()
                .any(|selected| selected.selects(*e))
        {
            return None;
        }
        let gap = if self.introduces == Gap::None {
            state.gap.unwrap_or(Gap::None)
        } else {
            if state.gap.is_some_and(|g| g != Gap::None) {
                return None;
            }
            self.introduces
        };
        if gap != Gap::None && grammar.category_gap[self.lhs] & gap.bit() == 0 {
            return None;
        }
        let coordinated = self.coordinator.is_some();
        let (values, last_resolved) = self.mother_values(state);
        let hosts = match state.preposition {
            Some((_, frame)) if !coordinated => grammar.preposition_frames[frame].hosts.mask(),
            _ => crate::frames::conjoined_hosts(
                state.conjunct_hosts,
                state.plain_hosts,
                state.conjoined_host,
                state.other_preposition.is_none() && !state.mixed_prepositions,
            ),
        };
        Some(Summary {
            first_glues: state.first_glues.unwrap_or(false),
            last_glues: state.last_glues,
            first_edge: state.first_edge,
            last_edge: state.last_edge,
            opening: state.opening,
            values,
            gap,
            gap_right_edge: state.gap_right_edge && state.shared_gap_right_edge,
            gap_preposition: match gap {
                Gap::PP if self.introduces == Gap::PP => self.gap_preposition,
                Gap::PP => state.gap_preposition,
                _ => None,
            },
            leaves: state.leaves,
            head_final: state.head_final && self.shape.ends_in_head_line,
            bare_head: state.bare_head && self.shape.undetermined_head_final,
            final_preposition: state.final_preposition,
            final_adjunct: state.final_adjunct,
            ellipsis: match self.omission {
                Omission::Ellipsis(forms) => forms,
                _ => state.ellipsis,
            },
            head_forms: state.head_forms,
            omitted_complement: if self.omission == Omission::Selected {
                state.extensions
            } else {
                state.omitted_complement
            },
            unlike_omissions: coordinated
                && state.bare_head
                && state.unlike_omissions
                && state.omitted_complement != 0,
            preposition: if coordinated && state.mixed_prepositions {
                None
            } else {
                state.preposition
            },
            other_preposition: if state.mixed_prepositions {
                None
            } else {
                state.other_preposition
            },
            last_resolved,
            unlike_values: if self.unlike.is_some() { state.unlike_values } else { 0 },
            extensions: state.extensions,
            coordinator: self.link.map(CoordinatorId::Linked).or(state.coordinator),
            coordinated: coordination || state.coordinated_head,
            head_coordination: coordination || state.coordinated_head || state.head_coordination,
            whole_coordination: coordination
                || (self.rhs.len() == 1 && self.head == Some(0) && state.only_head_coordination),
            hosts,
            nominal_unhosted: hosts & crate::frames::host_bit(crate::grammar::HostKind::Nominal)
                == 0
                || (self.rhs.len() == 1 && state.forced),
            fronted: if !self.fronts() {
                Fronting::None
            } else if self.host_check.is_some_and(|(host, adjunct, kind)| {
                adjunct < host && kind == HostKind::Predicate && !self.set_off_before_head(adjunct)
            }) {
                Fronting::Ambiguous
            } else {
                Fronting::Wide
            },
            internal_comma: state.internal_comma || self.hosts_comma_set_dependent(),
            open_comma: state.open_comma,
            auxiliary: self.auxiliary.is_some(),
            unwritten_period: state.unwritten_period,
            unclosed: self.unclosed || state.unclosed,
            nonfinal: self.nonfinal,
            link: self.link.or(state.headed_link),
            cluster: self
                .cluster
                .map_or(state.cluster.map_or(0, |(first, _)| first), |frame| {
                    frame + 1
                }),
            cluster_other: if self.cluster.is_some() {
                0
            } else {
                state.cluster.map_or(0, |(_, other)| other)
            },
            bare: if self.cluster.is_some() {
                Bare::cluster(
                    self.rhs.len(),
                    self.roles
                        .iter()
                        .filter(|&&role| role == ChildRole::Adjunct)
                        .count(),
                    state.forced,
                )
            } else {
                state.bare
            },
            remnant: match self.remnant {
                RemnantRole::Elides(head) => Some(head),
                _ => state.remnant,
            },
            shared_head: state.shared_head,
            cluster_preposition: state.cluster_preposition,
        })
    }
}

impl CompiledGrammar {
    /// The extension list an interned index names.
    pub(crate) fn extension_set(&self, index: u16) -> &[Extension] {
        &self.extension_sets[usize::from(index)]
    }

    /// Whether two bare nouns' omitted Complements are alike where a
    /// Preposition Phrase after their coordination could be one: in the
    /// prepositions they omit. An omitted Amount is no such Complement
    /// (*number and kind [of counters]*, where *number* also takes one).
    pub(crate) fn like_omissions(&self, a: u16, b: u16) -> bool {
        let prepositions = |index| {
            self.extension_set(index)
                .iter()
                .filter(|e| matches!(e, Extension::Preposition(..)))
        };
        a == b || prepositions(a).eq(prepositions(b))
    }

    /// Whether a lexical leaf satisfies a test.
    pub(crate) fn leaf_matches(&self, test: &Test, leaf: &LeafFacts) -> bool {
        let facts = leaf.lexeme.map(|l| self.facts(l));
        self.leaf_matches_with_facts(test, leaf, facts)
    }

    /// The same test when the caller already has the lexeme's bound facts.
    pub(crate) fn leaf_matches_with_facts(
        &self,
        test: &Test,
        leaf: &LeafFacts,
        facts: Option<&LexemeFacts>,
    ) -> bool {
        // Another card's name fills only a position that opts in to it.
        if leaf.other_card_name != (*test == Test::OtherCardName) {
            return false;
        }
        match test {
            Test::OtherCardName => true,
            Test::Lexical(category) => leaf.category == *category,
            Test::FreeLexical(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| f.binding == sylex_lexicon::Binding::Free)
            }
            Test::FreeOptional(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| {
                        f.binding == sylex_lexicon::Binding::Free && f.optional_complement
                    })
            }
            Test::FreeNotActiveOnly(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| {
                        f.binding == sylex_lexicon::Binding::Free && !f.active_only
                    })
            }
            Test::Numeral(notation) => leaf.notation == Some(*notation),
            Test::Word(id) => leaf.lexeme == Some(*id),
            Test::Words(words) => leaf.lexeme.is_some_and(|id| words.contains(&id)),
            Test::Class(class) => leaf
                .lexeme
                .is_some_and(|l| self.classes[*class].contains(&l)),
            Test::Frame(frame) => facts.is_some_and(|f| f.frames.contains(frame)),
            Test::FreeFrame(frame) => facts.is_some_and(|f| {
                f.frames.contains(frame) && f.binding == sylex_lexicon::Binding::Free
            }),
            Test::ClassFrame { class, frame, free } => {
                leaf.lexeme
                    .is_some_and(|l| self.classes[*class].contains(&l))
                    && facts.is_some_and(|f| {
                        f.frames.contains(frame)
                            && (!free || f.binding == sylex_lexicon::Binding::Free)
                    })
            }
            Test::PrepositionFrame(frame) => {
                facts.is_some_and(|f| f.preposition_frames.contains(frame))
            }
        }
    }

    /// The Adjacency Agreement edges of a lexical value this grammar's
    /// lexicon licenses.
    pub(crate) fn leaf_edges(&self, reading: &LexicalReading) -> Option<sylex_lexicon::Edges> {
        match reading {
            LexicalReading::Word(value) => self.value_facts(value).map(|v| v.edges),
            LexicalReading::Numeral { .. } | LexicalReading::FlavorWord { .. } => {
                sylex_lexicon::Edges::non_word(reading)
            }
        }
    }

    /// Projects a lexical alternative into the leaf a Production sees, in
    /// the text of a face whose own card names are `scope`, or `None` if the
    /// lexicon licenses no such value. Its features were resolved at bind
    /// time (`crate::projection`).
    pub(crate) fn leaf_facts(
        &self,
        reading: &LexicalReading,
        scope: &sylex_lexicon::CardScope,
    ) -> Option<LeafFacts> {
        match reading {
            LexicalReading::Word(value) => {
                let lexeme = self.lexeme(&value.lexeme)?;
                let facts = self.facts(lexeme);
                let licensed = self.value_facts(value)?;
                let mut features = [None; MAX_FEATURES];
                for &(feature, value) in &licensed.leaf_features {
                    features[feature] = Some(value);
                }
                Some(LeafFacts {
                    binding: facts.binding,
                    integer: None,
                    notation: None,
                    lexeme: Some(lexeme),
                    category: facts.category,
                    features,
                    forms: licensed.forms,
                    other_card_name: facts.card_name
                        && !value
                            .features
                            .name
                            .is_some_and(|form| scope.holds(value.lexeme, form)),
                })
            }
            LexicalReading::Numeral {
                value, notation, ..
            } => Some(LeafFacts {
                binding: sylex_lexicon::Binding::Free,
                integer: Some(*value),
                notation: Some(*notation),
                lexeme: None,
                category: LexicalCategory::Numeral,
                features: self.projection.numeral_features(*value, *notation),
                forms: 0,
                other_card_name: false,
            }),
            LexicalReading::FlavorWord { .. } => Some(LeafFacts {
                binding: sylex_lexicon::Binding::Free,
                integer: None,
                notation: None,
                lexeme: None,
                category: LexicalCategory::FlavorWord,
                features: [None; MAX_FEATURES],
                forms: 0,
                other_card_name: false,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;
    use std::hash::Hasher;

    use super::FeatureValue;

    /// Records each write a value's hash makes.
    #[derive(Default)]
    struct Writes(Vec<String>);

    impl Hasher for Writes {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0.push(format!("bytes {bytes:?}"));
        }

        fn write_u8(&mut self, value: u8) {
            self.0.push(format!("u8 {value}"));
        }

        fn write_u64(&mut self, value: u64) {
            self.0.push(format!("u64 {value:#018x}"));
        }

        fn write_usize(&mut self, value: usize) {
            self.0.push(format!("usize {value}"));
        }
    }

    #[test]
    fn a_feature_value_array_hashes_a_word_per_eight_values() {
        let mut values = [FeatureValue::UNSET; 10];
        values[0] = FeatureValue::new(1);
        values[9] = FeatureValue::new(2);
        let mut writes = Writes::default();
        values.hash(&mut writes);
        assert_eq!(
            writes.0,
            [
                "usize 10",
                "u64 0xffffffffffffff01",
                "u64 0x00000000000002ff",
            ]
        );
    }
}
