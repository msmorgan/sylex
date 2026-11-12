//! Subcategorization Frames: the Complements each head licenses.

use std::collections::BTreeMap;

use dpsi::Ident;
use serde::Deserialize;
use serde::Serialize;
use sylex_lexicon::LexemeId;

use crate::LoadError;

/// The kind of a Subcategorization Frame, named by the shape of its slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum FrameFamily {
    Intransitive,
    Monotransitive,
    Ditransitive,
    TransitiveWithPP,
    TransitiveWithComplement,
    Prepositional,
    Copular,
    Catenative,
    /// An object and a predicative complement, as in *turn it face up*.
    ComplexTransitive,
    /// An object and a non-finite complement, as in *cause you to sacrifice*.
    ComplexCatenative,
    /// A content clause, as in *decide whether to flip again*.
    ContentClause,
    /// A numeric Amount, as in *scry 2*.
    Amount,
    /// An Object followed by an Amount, as in *amass Zombies 2*.
    TransitiveAmount,
    /// A Cost, as in *waterbend {3}*.
    Cost,
    /// A fixed preposition with the verb's other Complements, as in the
    /// phrasal-copular *come up heads* (CGEL, Ch. 4 §6.3, p. 288).
    Idiom,
    NounComplement,
    AdjectiveComplement,
    DeterminativeComplement,
}

/// The grammatical relation a Frame Slot bears to its head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotRelation {
    Object,
    IndirectObject,
    Complement,
    PredicativeComplement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum ClauseKind {
    Finite,
    Verbless,
    /// A non-finite or verbless clause with its Subject, the Complement of
    /// *with* and *without* (*with your hand revealed*; CGEL, Ch. 14 §8.3,
    /// p. 1262, and §10(a), p. 1267).
    Absolute,
    /// An interrogative content clause (*whether ...*, *which creatures
    /// attack*, *how each player votes*).
    Interrogative,
}

/// What fills a Frame Slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotFill {
    NP,
    /// A Preposition Phrase headed by one of the listed prepositions; with a
    /// [`SlotKind`], only one whose preposition takes a Complement of that
    /// kind (*do the same for [Vampire]*, a cited word, never *for flying*).
    PP(Vec<LexemeId>, #[serde(default)] Option<SlotKind>),
    Pred,
    /// A card's name, in a position where another card's name stands: the
    /// predicative complement of *name* (*cards named Relentless Rats*), the
    /// complement of *among* after *from* (*from among Disenchant, ..., and
    /// Black Lotus*), or the complement of *into* after *meld* (*meld them
    /// into Brisela, Voice of Nightmares*).
    Name,
    AdjP,
    /// An adverbial: an Adverb Phrase or a Noun Phrase of frequency, as
    /// *twice* or *three times* in *more than twice*.
    AdvP,
    LocativeNP,
    /// An ordinal position from an end of an ordered pile, the location a
    /// verb of placement selects (*put that card third from the top*).
    Position,
    /// A slash measure as a Noun Phrase, taken only where a head selects
    /// one (*gets +2/+2*).
    MeasureNP,
    Clause(ClauseKind),
    /// A declarative content clause permitting the subordinator *that*:
    /// whether a content clause may be marked is a property of the head
    /// that selects it, not of the clause.
    ExpandableClause,
    /// A Clause set off by a dash after its head's Object, giving that
    /// Object's content (*faces a villainous choice — They discard their
    /// hand, or you draw three cards*): the dash is the slot's, not the
    /// clause's.
    SetOffClause,
    ToInf,
    /// A *to*-infinitival with an object Gap its verb's Subject fills
    /// (*costs {1} to cast*).
    HollowToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    /// A word cited rather than used, as *left or right* in *choose left or
    /// right*: a metalinguistic Noun Phrase (CGEL, Ch. 5 §8.2, p. 401),
    /// taken only where a head selects one.
    Mention,
    Nominal,
    /// A numeral or variable naming how many, as in *scry 2* or *scry X*.
    Amount,
    /// A cost to pay, as in *waterbend {3}*.
    Cost,
    /// No Complement: an intransitive preposition, as *out* in *phase out*.
    Intransitive,
    /// An unlike coordination of the kinds of an unlike slot the grammar
    /// declares (`Grammar::unlike_slots`), restricted to those the head
    /// selects in this position (*has flying and "{T}: Add {G}."*,
    /// *equal to [2] or [the greatest power among ...]*). It is never
    /// declared: a head whose frames select two or more of the slot's kinds
    /// in one position selects it there, since each conjunct fills the
    /// position alone (CGEL, Ch. 15 §3.1, p. 1323, rule [1]).
    #[serde(skip)]
    Unlike(UnlikeFill),
}

/// The Fill of a derived unlike Frame Slot: the grammar's unlike slot, by
/// its index in `Grammar::unlike_slots` and its name, and the kinds the
/// head selects of it, in the slot's order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct UnlikeFill {
    pub slot: u8,
    pub name: Ident,
    pub kinds: Vec<SlotKind>,
}

/// A [`SlotFill`] without its markers: the key a grammar maps to a Category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotKind {
    TypeModifier,
    NonManaCost,
    Quality,
    Ability,
    Condition,
    Measure,
    Name,
    NP,
    PP,
    Pred,
    AdjP,
    AdvP,
    LocativeNP,
    Position,
    MeasureNP,
    FiniteClause,
    ExpandableClause,
    SetOffClause,
    VerblessClause,
    AbsoluteClause,
    InterrogativeClause,
    ToInf,
    HollowToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    Mention,
    Nominal,
    Amount,
    Cost,
    Intransitive,
    /// A derived unlike Frame Slot ([`SlotFill::Unlike`]): the grammar's
    /// unlike slot of this index, never a declared kind.
    #[serde(skip)]
    Unlike(u8),
}

impl SlotFill {
    #[must_use]
    pub fn kind(&self) -> SlotKind {
        match self {
            Self::NP => SlotKind::NP,
            Self::PP(..) => SlotKind::PP,
            Self::Pred => SlotKind::Pred,
            Self::Name => SlotKind::Name,
            Self::AdjP => SlotKind::AdjP,
            Self::AdvP => SlotKind::AdvP,
            Self::LocativeNP => SlotKind::LocativeNP,
            Self::Position => SlotKind::Position,
            Self::MeasureNP => SlotKind::MeasureNP,
            Self::Clause(ClauseKind::Finite) => SlotKind::FiniteClause,
            Self::ExpandableClause => SlotKind::ExpandableClause,
            Self::SetOffClause => SlotKind::SetOffClause,
            Self::Clause(ClauseKind::Verbless) => SlotKind::VerblessClause,
            Self::Clause(ClauseKind::Absolute) => SlotKind::AbsoluteClause,
            Self::Clause(ClauseKind::Interrogative) => SlotKind::InterrogativeClause,
            Self::ToInf => SlotKind::ToInf,
            Self::HollowToInf => SlotKind::HollowToInf,
            Self::BareInf => SlotKind::BareInf,
            Self::Gerund => SlotKind::Gerund,
            Self::KeywordPhrase => SlotKind::KeywordPhrase,
            Self::QuotedAbility => SlotKind::QuotedAbility,
            Self::Mention => SlotKind::Mention,
            Self::Nominal => SlotKind::Nominal,
            Self::Amount => SlotKind::Amount,
            Self::Cost => SlotKind::Cost,
            Self::Intransitive => SlotKind::Intransitive,
            Self::Unlike(fill) => SlotKind::Unlike(fill.slot),
        }
    }
}

/// One position in a Subcategorization Frame.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct FrameSlot {
    pub relation: SlotRelation,
    pub fill: SlotFill,
}

/// An ordered list of the Complements a head licenses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct Frame {
    pub family: FrameFamily,
    pub slots: Vec<FrameSlot>,
    /// The frame is restricted to active voice, so it derives no passive
    /// Valence Alternation: *A packet of cigarettes costs around seven
    /// dollars*, *Jill has the best qualifications* (CGEL, Ch. 16 §10.1.2,
    /// p. 1432, [17]).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub active_only: bool,
}

impl Frame {
    /// Whether the slot list has the shape its family names.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        use SlotRelation::Complement;
        use SlotRelation::IndirectObject;
        use SlotRelation::Object;
        use SlotRelation::PredicativeComplement;
        let shape: Vec<_> = self.slots.iter().map(|s| (s.relation, &s.fill)).collect();
        match self.family {
            FrameFamily::Intransitive => shape.is_empty(),
            FrameFamily::Monotransitive => matches!(
                shape[..],
                [(
                    Object,
                    SlotFill::NP
                        | SlotFill::KeywordPhrase
                        | SlotFill::QuotedAbility
                        | SlotFill::Mention
                        | SlotFill::MeasureNP
                )]
            ),
            FrameFamily::Ditransitive => matches!(
                shape[..],
                [(IndirectObject, SlotFill::NP), (Object, SlotFill::NP)]
            ),
            FrameFamily::TransitiveWithComplement => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (
                        Complement,
                        SlotFill::LocativeNP
                            | SlotFill::Position
                            | SlotFill::HollowToInf
                            | SlotFill::SetOffClause
                    )
                ]
            ),
            FrameFamily::TransitiveWithPP => {
                matches!(
                    shape[..],
                    [(Object, SlotFill::NP), (Complement, SlotFill::PP(..))]
                )
            }
            FrameFamily::Prepositional => matches!(shape[..], [(Complement, SlotFill::PP(..))]),
            FrameFamily::Copular => matches!(shape[..], [(PredicativeComplement, SlotFill::Pred)]),
            FrameFamily::Catenative => matches!(
                shape[..],
                [(
                    Complement,
                    SlotFill::ToInf | SlotFill::BareInf | SlotFill::Gerund
                )]
            ),
            FrameFamily::ComplexTransitive => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (PredicativeComplement, SlotFill::Pred | SlotFill::Name)
                ]
            ),
            FrameFamily::ComplexCatenative => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (Complement, SlotFill::ToInf | SlotFill::BareInf)
                ]
            ),
            FrameFamily::ContentClause => {
                matches!(
                    shape[..],
                    [(
                        Complement,
                        SlotFill::Clause(ClauseKind::Finite | ClauseKind::Interrogative)
                            | SlotFill::ExpandableClause
                    )]
                )
            }
            FrameFamily::Amount => matches!(shape[..], [(Complement, SlotFill::Amount)]),
            FrameFamily::TransitiveAmount => matches!(
                shape[..],
                [(Object, SlotFill::NP), (Complement, SlotFill::Amount)]
            ),
            FrameFamily::Cost => matches!(shape[..], [(Complement, SlotFill::Cost)]),
            FrameFamily::Idiom => {
                shape
                    .iter()
                    .any(|slot| matches!(slot, (Complement, SlotFill::PP(..))))
                    && shape.iter().all(|slot| {
                        matches!(
                            slot,
                            (Complement, SlotFill::PP(..))
                                | (PredicativeComplement, SlotFill::Pred)
                        )
                    })
            }
            FrameFamily::NounComplement
            | FrameFamily::AdjectiveComplement
            | FrameFamily::DeterminativeComplement => {
                matches!(shape[..], [(Complement, fill)] if *fill != SlotFill::Intransitive)
            }
        }
    }
}

/// Which hosts an unselected Preposition Phrase may modify (its Host Category).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Hosts {
    ComplementOnly,
    Nominal,
    Predicate,
    NominalAndPredicate,
    /// A Nominal's, and a Predicate's only as a conjunct of a coordination
    /// of Preposition Phrases of its one preposition, beside a conjunct that
    /// is a Predicate's on its own (*enters with flying and with X +1/+1
    /// counters on it*, never *enters with flying*).
    NominalAndConjoinedPredicate,
    /// Only a Supplement to a Clause or line (*, where X is its power*).
    Supplement,
}

impl Hosts {
    /// The Host Categories admitted, as a set of [`HostKind`] bits.
    #[must_use]
    pub(crate) fn mask(self) -> u8 {
        use crate::grammar::HostKind;
        let conjoined =
            if self == Self::NominalAndConjoinedPredicate { CONJOINED_PREDICATE } else { 0 };
        [HostKind::Nominal, HostKind::Predicate, HostKind::Supplement]
            .into_iter()
            .filter(|kind| self.admits(*kind))
            .fold(conjoined, |mask, kind| mask | host_bit(kind))
    }

    #[must_use]
    pub fn admits(self, kind: crate::grammar::HostKind) -> bool {
        use crate::grammar::HostKind;
        matches!(
            (self, kind),
            (
                Self::Nominal | Self::NominalAndPredicate | Self::NominalAndConjoinedPredicate,
                HostKind::Nominal
            ) | (
                Self::Predicate | Self::NominalAndPredicate,
                HostKind::Predicate
            ) | (Self::Supplement, HostKind::Supplement)
        )
    }
}

impl Hosts {
    /// The Host Categories both admit.
    #[must_use]
    pub(crate) fn meet(self, other: Self) -> Self {
        match self.mask() & other.mask() & !CONJOINED_PREDICATE {
            0 => Self::ComplementOnly,
            1 => Self::Nominal,
            2 => Self::Predicate,
            3 => Self::NominalAndPredicate,
            _ => Self::Supplement,
        }
    }
}

/// The bit of a set of admitted hosts that admits a Predicate host for a
/// coordination with a conjunct that admits one on its own
/// ([`Hosts::NominalAndConjoinedPredicate`]).
pub(crate) const CONJOINED_PREDICATE: u8 = 8;

/// The hosts a coordination admits: those every conjunct admits (`all`), a
/// conjunct that admits a Predicate host only as a conjunct counting as one
/// that does. With such a conjunct (`conjoined`), it admits a Predicate host
/// only if some conjunct admits it on its own (`plain`) and every conjunct is
/// a Preposition Phrase of one preposition (`one_preposition`).
pub(crate) fn conjoined_hosts(all: u8, plain: u8, conjoined: bool, one_preposition: bool) -> u8 {
    let predicate = host_bit(crate::grammar::HostKind::Predicate);
    if conjoined && (plain & predicate == 0 || !one_preposition) {
        all & !predicate
    } else {
        all
    }
}

/// A conjunct's hosts as a coordination counts them, and whether it admits a
/// Predicate host only as a conjunct ([`conjoined_hosts`]).
pub(crate) fn conjunct_hosts(hosts: u8) -> (u8, bool) {
    let predicate = host_bit(crate::grammar::HostKind::Predicate);
    if hosts & CONJOINED_PREDICATE == 0 || hosts & predicate != 0 {
        (hosts, false)
    } else {
        (hosts | predicate, true)
    }
}

/// A [`HostKind`]'s bit in a set of admitted hosts.
pub(crate) fn host_bit(kind: crate::grammar::HostKind) -> u8 {
    use crate::grammar::HostKind;
    match kind {
        HostKind::Nominal => 1,
        HostKind::Predicate => 2,
        HostKind::Supplement => 4,
    }
}

/// One complement frame of a preposition with its Host Category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct PrepositionFrame {
    #[serde(default = "complement_relation")]
    pub relation: SlotRelation,
    pub complement: SlotFill,
    pub hosts: Hosts,
    /// Only a Frame Slot that names this Complement's kind selects the
    /// phrase (`PP(prepositions, Some(kind))`), never one that names its
    /// preposition alone: a cited word after *for* is taken where a head
    /// selects one (*do the same for Vampire*), not by a head that selects
    /// *for* with a Noun Phrase (*vote for blue*), where it would be a second
    /// tree of the bare noun.
    #[serde(default)]
    pub named_only: bool,
    /// The Complement is plural (*among*; CGEL, Ch. 7 §5.1(a), p. 636): a
    /// coordination itself, through a Production whose only child is its
    /// head but not under a determiner or modifier, or else a Noun Phrase
    /// that requires Plural (`Require`, which an open Number passes). A
    /// keyword phrase, slash measure or name has no Number, so only a
    /// coordination of it is plural.
    #[serde(default)]
    pub plural_complement: bool,
}

/// Subcategorization Frames by lexeme. Prepositions declare theirs in
/// `prepositions`, every other head in `frames`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrameCatalog {
    /// Ordered parameter patterns, including the position of the head.
    #[serde(default)]
    pub parameters: BTreeMap<LexemeId, Vec<Vec<ParameterPart>>>,
    #[serde(default)]
    pub frames: BTreeMap<LexemeId, Vec<Frame>>,
    #[serde(default)]
    pub prepositions: BTreeMap<LexemeId, Vec<PrepositionFrame>>,
}

impl FrameCatalog {
    /// # Errors
    /// Malformed RON or a frame whose slots do not have its family's shape.
    pub fn from_ron_str(text: &str) -> Result<Self, LoadError> {
        let catalog: Self =
            ron::from_str(text).map_err(|error| LoadError::Format(error.to_string()))?;
        for (lexeme, frames) in &catalog.frames {
            if let Some(frame) = frames.iter().find(|frame| !frame.is_well_formed()) {
                return Err(LoadError::Invalid(format!(
                    "frame {frame:?} of {lexeme} does not have the shape of its family"
                )));
            }
        }
        Ok(catalog)
    }
}

/// One component of a catalog-declared parameter pattern.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ParameterPart {
    Head,
    Slot(SlotKind),
    Word(LexemeId),
    Words(Vec<LexemeId>),
    /// A word set with recursive premodification, projected as a constituent.
    ModifiedWords(Vec<LexemeId>, SlotKind, SlotKind),
    Mark(char, sylex_lexicon::Binding),
    /// A value the pattern gives a feature of its phrase, as no word: the
    /// kind of a keyword form's final Keyword Parameter.
    Feature(Ident, Ident),
}

fn complement_relation() -> SlotRelation {
    SlotRelation::Complement
}
