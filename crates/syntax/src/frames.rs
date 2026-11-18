//! Subcategorization Frames: the Complements each head licenses.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
    /// An object and a non-finite complement, as in *cause you to sacrifice*
    /// or *have counters put on it*.
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
    /// A finite Clause that locates its host's situation at its own time: a
    /// temporal preposition's Complement (*when*, *whenever*, *until*,
    /// *before*, *after*, *while*, *as*; CGEL, Ch. 8 §6, p. 700).
    Temporal,
    /// A comparative clause, the Complement of *than* or equative *as*: a
    /// finite Clause reduced at the position its comparative governor
    /// corresponds to (*as many cards as they discarded this way*, *more
    /// cards than you do*; CGEL, Ch. 13 §2.1, p. 1108).
    Comparative,
    /// A manner clause, the Complement of manner *as*: a comparative clause
    /// whose implicit manner element is compared with the phrase's, with a
    /// Gap (*divided as you choose*; CGEL, Ch. 13 §5.5, p. 1149, [34]).
    Manner,
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
    PastPart,
    KeywordPhrase,
    QuotedAbility,
    /// A word cited rather than used, as *left or right* in *choose left or
    /// right*: a metalinguistic Noun Phrase (CGEL, Ch. 5 §8.2, p. 401),
    /// taken only where a head selects one.
    Mention,
    /// The two words a vote offers [CR#701.38a,701.38b], as *sickness or
    /// psychosis* in *votes for sickness or psychosis*: taken only where a
    /// head selects it.
    OptionChoice,
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
    TemporalClause,
    ComparativeClause,
    MannerClause,
    ExpandableClause,
    SetOffClause,
    VerblessClause,
    AbsoluteClause,
    InterrogativeClause,
    ToInf,
    HollowToInf,
    BareInf,
    Gerund,
    PastPart,
    KeywordPhrase,
    QuotedAbility,
    Mention,
    Nominal,
    Amount,
    Cost,
    Intransitive,
    /// A Set Name, the parameter a noun takes before it (*the Antiquities
    /// expansion*).
    SetName,
    OptionChoice,
    /// A word a vote offers, cited: the parameter a noun takes before it
    /// (*each numbers vote* [CR#701.38b]).
    OptionMention,
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
            Self::Clause(ClauseKind::Temporal) => SlotKind::TemporalClause,
            Self::Clause(ClauseKind::Comparative) => SlotKind::ComparativeClause,
            Self::Clause(ClauseKind::Manner) => SlotKind::MannerClause,
            Self::ExpandableClause => SlotKind::ExpandableClause,
            Self::SetOffClause => SlotKind::SetOffClause,
            Self::Clause(ClauseKind::Verbless) => SlotKind::VerblessClause,
            Self::Clause(ClauseKind::Absolute) => SlotKind::AbsoluteClause,
            Self::Clause(ClauseKind::Interrogative) => SlotKind::InterrogativeClause,
            Self::ToInf => SlotKind::ToInf,
            Self::HollowToInf => SlotKind::HollowToInf,
            Self::BareInf => SlotKind::BareInf,
            Self::Gerund => SlotKind::Gerund,
            Self::PastPart => SlotKind::PastPart,
            Self::KeywordPhrase => SlotKind::KeywordPhrase,
            Self::QuotedAbility => SlotKind::QuotedAbility,
            Self::Mention => SlotKind::Mention,
            Self::OptionChoice => SlotKind::OptionChoice,
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

/// The voices a verb frame is realized in.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize,
)]
pub enum FrameVoice {
    /// Both: the active, and every passive the frame's shape derives.
    #[default]
    ActiveOrPassive,
    /// Active only, deriving no passive Valence Alternation: *A packet of
    /// cigarettes costs around seven dollars*, *Jill has the best
    /// qualifications* (CGEL, Ch. 16 §10.1.2, p. 1432, [17]).
    ActiveOnly,
    /// Passive only, with no active: a verb restricted to the passive with
    /// a particular complementation, as *say* with an object and an
    /// infinitival (*Kim is said to be a manic depressive*; CGEL, Ch. 16
    /// §10.1.2, p. 1435, [31]), or *deal* with two objects (*this creature
    /// is dealt damage*).
    PassiveOnly,
}

impl FrameVoice {
    /// Whether the frame has an active.
    #[must_use]
    pub fn active(self) -> bool {
        self != Self::PassiveOnly
    }

    /// Whether the frame derives its passives.
    #[must_use]
    pub fn passive(self) -> bool {
        self != Self::ActiveOnly
    }

    /// The voices both admit, or `None` when they share none.
    #[must_use]
    pub fn meet(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::ActiveOnly, Self::PassiveOnly) | (Self::PassiveOnly, Self::ActiveOnly) => None,
            (Self::ActiveOrPassive, voice) | (voice, Self::ActiveOrPassive) => Some(voice),
            (Self::ActiveOnly, Self::ActiveOnly) => Some(Self::ActiveOnly),
            (Self::PassiveOnly, Self::PassiveOnly) => Some(Self::PassiveOnly),
        }
    }

    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's `skip_serializing_if` passes a reference"
    )]
    fn is_default(&self) -> bool {
        *self == Self::ActiveOrPassive
    }
}

/// An ordered list of the Complements a head licenses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct Frame {
    pub family: FrameFamily,
    pub slots: Vec<FrameSlot>,
    #[serde(default, skip_serializing_if = "FrameVoice::is_default")]
    pub voice: FrameVoice,
    /// A noun's Complement names the one argument a genitive
    /// subject-determiner names, so the two compete and its phrase takes no
    /// genitive determiner: *the flood's consequences* or *the consequences
    /// of the flood* (CGEL, Ch. 5, §16.5.3, pp. 475–476, [56ii], [59]).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub genitive_alternant: bool,
}

impl Frame {
    /// Whether the slot list has the shape its family names, and a
    /// Passive-only frame is one that passivizes: its first slot is an
    /// Object Noun Phrase, or it is a Prepositional frame, whose
    /// preposition's object is the passive's subject.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        self.has_family_shape() && (self.voice != FrameVoice::PassiveOnly || self.passivizes())
    }

    /// Whether the frame's shape derives a passive: verbal from an Object
    /// Noun Phrase first, or prepositional from a Prepositional frame.
    #[must_use]
    pub fn passivizes(&self) -> bool {
        match self.slots.first() {
            Some(FrameSlot {
                relation: SlotRelation::Object | SlotRelation::IndirectObject,
                fill: SlotFill::NP,
            }) => true,
            Some(FrameSlot {
                fill: SlotFill::PP(..),
                ..
            }) => self.family == FrameFamily::Prepositional,
            _ => false,
        }
    }

    fn has_family_shape(&self) -> bool {
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
                    (
                        Complement,
                        SlotFill::ToInf | SlotFill::BareInf | SlotFill::PastPart
                    )
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
    /// The grammar's Complement Constraints it names
    /// (`crate::grammar::ConstraintDecl`): what
    /// its Complement satisfies beyond its kind (*among*'s is plural), and
    /// whether only a Frame Slot naming that kind selects the phrase (*do
    /// the same for Vampire*, never *vote for [blue]* as a cited word).
    #[serde(default)]
    pub constraints: BTreeSet<Ident>,
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
            for frame in frames {
                if !frame.has_family_shape() {
                    return Err(LoadError::Invalid(format!(
                        "frame {frame:?} of {lexeme} does not have the shape of its family"
                    )));
                }
                if !frame.is_well_formed() {
                    return Err(LoadError::Invalid(format!(
                        "frame {frame:?} of {lexeme} is Passive-only but has no passive"
                    )));
                }
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
    /// A word set with recursive premodification, projected as a constituent:
    /// stems that may stand bare, stems that require at least one premodifier,
    /// the premodifier slot, and the projected constituent slot.
    ModifiedWords {
        bare: Vec<LexemeId>,
        modified_only: Vec<LexemeId>,
        modifier: SlotKind,
        target: SlotKind,
    },
    Mark(char, sylex_lexicon::Binding),
    /// A value the pattern gives a feature of its phrase, as no word: the
    /// kind of a keyword form's final Keyword Parameter, or whether the form
    /// carries a Keyword Parameter at all.
    Feature(Ident, Ident),
}

fn complement_relation() -> SlotRelation {
    SlotRelation::Complement
}

#[cfg(test)]
mod tests {
    use super::FrameVoice::ActiveOnly;
    use super::FrameVoice::ActiveOrPassive;
    use super::FrameVoice::PassiveOnly;

    /// An unlike frame has the voices every frame it unites has.
    #[test]
    fn frame_voices_meet_by_intersection() {
        assert_eq!(ActiveOrPassive.meet(ActiveOrPassive), Some(ActiveOrPassive));
        assert_eq!(ActiveOrPassive.meet(ActiveOnly), Some(ActiveOnly));
        assert_eq!(PassiveOnly.meet(ActiveOrPassive), Some(PassiveOnly));
        assert_eq!(PassiveOnly.meet(PassiveOnly), Some(PassiveOnly));
        assert_eq!(ActiveOnly.meet(PassiveOnly), None);
        assert_eq!(PassiveOnly.meet(ActiveOnly), None);
    }
}
