//! Refusal vocabulary mirrored from `lean/Semantics/Check/Refusal.lean`.
use deckmaste_semantics_v3::ast::AbilityWordLabel;
use deckmaste_semantics_v3::ast::BindingId;
use deckmaste_semantics_v3::ast::ChoiceRef;
use deckmaste_semantics_v3::ast::ChoiceSort;
use deckmaste_semantics_v3::ast::DesignationLabel;
use deckmaste_semantics_v3::ast::KeywordLabel;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::Letter;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::Period;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::PossessorAxis;
use deckmaste_semantics_v3::ast::QualitySort;
use deckmaste_semantics_v3::ast::Reach;

use crate::reads::ParticipialAnchor;

/// Mirrors `Semantics/Check/Refusal.lean:16`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    RecursionLimit,
    ConcreteCost,
    ConcreteBallot,
    RiderNegationPattern,
    ControlChangePattern,
    KindMismatch {
        expected: Box<Kind>,
        found: Box<Kind>,
    },
    Phrasal {
        k: Box<Kind>,
    },
    Targetable {
        k: Box<Kind>,
    },
    Targeter {
        k: Box<Kind>,
    },
    KindLte {
        k: Box<Kind>,
        bound: Box<Kind>,
    },
    PossessorKind {
        axis: PossessorAxis,
        k: Box<Kind>,
    },
    ProjScope {
        k: Box<Kind>,
    },
    AxesAt {
        k: Box<Kind>,
    },
    DesignationHolder {
        designation: DesignationLabel,
        k: Box<Kind>,
    },
    DesignationScope {
        designation: DesignationLabel,
    },
    DesignationPossessorFits {
        designation: DesignationLabel,
    },
    NoSource,
    SoleHolder,
    Singular,
    Plural,
    GroupMention,
    PartitiveBase,
    CountableGroup,
    CountedMention,
    SlicePossessor,
    PerMember,
    ExistentialMention,
    ComplementAnchor,
    Choosable,
    ChoiceClause,
    ChoiceOrder,
    TestSubject,
    OrdinalNonZero,
    EventOrdinalPeriod {
        period: Period,
    },
    Anaphor {
        r: Reach,
        pl: Plurality,
        found: u64,
    },
    ReflexiveAntecedent {
        pl: Plurality,
        found: u64,
    },
    WhichUnbound,
    WhichUnused,
    ParticipialDescription {
        anchor: ParticipialAnchor,
        found: u64,
    },
    ParticipialAgent,
    UnanchoredLookback,
    TurnAntecedent {
        found: u64,
    },
    BindingIdentity {
        id: BindingId,
    },
    ChoiceRef {
        ref_: ChoiceRef,
        sort: ChoiceSort,
        found: u64,
    },
    ChoiceInScope {
        k: Box<Kind>,
    },
    ChosenObjectRef {
        ref_: ChoiceRef,
        found: u64,
    },
    OutcomeInScope {
        s: OutcomeSort,
        found: u64,
    },
    QuantOutcomeInScope {
        found: u64,
    },
    CoinFlipInScope,
    DamageDealtInScope,
    GroupSizeInScope {
        found: u64,
    },
    DifferenceInScope {
        found: u64,
    },
    TheRestFits {
        plurality: Plurality,
    },
    ContrastAnchor {
        k: Box<Kind>,
    },
    EventAgent,
    ChooserInScope {
        found: u64,
    },
    TokenSpecInScope {
        found: u64,
    },
    OpenLetter {
        l: Letter,
    },
    Bindingless,
    IgnorableFor,
    ReadAmount,
    NonZeroQ,
    WellFormedQ,
    NonEmpty,
    AtLeastTwo,
    FlatConjuncts,
    FlatDisjuncts,
    OtherAnchored,
    CoordinableDisjuncts,
    Negatable,
    Uniquifying,
    BallotLabelsOk,
    AtLeastTwoZones,
    IsExtremal,
    ChosenQualityRead {
        q: QualitySort,
    },
    Ascribable,
    LinkSource,
    PileMention,
    PredSays,
    AmtNonZero,
    TallyOk,
    TextWordSort {
        sorts: Vec<QualitySort>,
        found: QualitySort,
    },
    ShuffleZone,
    DurationPossessor,
    QuantLiteral,
    ModesFit,
    FacesFit,
    ChapterMarks,
    Distinct,
    KindDomainOk,
    KindAxisSort,
    EachKindPlacement,
    KnownKeywordTerm,
    LinkedFacetNamed,
    EventHead,
    DeonticCause,
    DeonticRider,
    CreatesDelayedTrigger,
    VerbedVoiceOk,
    HandoffPerformer,
    TokenPhrase,
    NumberChoiceInScope,
    NotCoord,
    ForEachAmount,
    NotCompound,
    ReflexEnclosure,
    ThisWayOutcome,
    ThisWayUngrounded,
    TriggeringUngrounded,
    IfDoneArmed,
    ProducedRuns,
    LineNonEmpty,
    CounterHolderKind,
    QualityRead,
    NotWordHeaded,
    TokenNamed,
    DefinitionHolder {
        k: Box<Kind>,
    },
    DefinitionNamed,
    CardLine,
    ModalFrame,
    DoorFrame,
    PrintedAfterKeyword {
        k: KeywordLabel,
    },
    CardName,
    CardBox,
    CardCost,
    FlipHalf,
    LevelRange,
    LevelerFrame,
    BandsDisjoint,
    PrototypeFrame,
    RiddenEntering,
    HeadNoun,
    ShiftSets,
    EntryRidersOnly,
    KnownAbilityWord {
        word: AbilityWordLabel,
    },
    CdaEdit,
    CdaSubject,
    CdaGranted,
    CdaConditional,
    RetentionClause,
    RetentionSubject,
}
