//! Syntax mirrored from
//! `lean/Semantics/{Words,Events,Abilities,Card,Rules}.lean`.
use dpsi::Ident;

pub type FlavorWordLabel = Ident;
pub type VoteLabel = Ident;
pub type KeywordActionLabel = Ident;
pub type BindingId = Ident;
pub type ManaCost = Vec<ManaSymbol>;
pub type ProducedRun = Vec<ColorOrColorless>;
pub type ChapterNumber = u64;

/// Mirrors `Semantics/Words.lean:30`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Procedure {
    Activate,
    Cast,
    Play,
    Resolve,
}

/// Mirrors `Semantics/Words.lean:35`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Stat {
    Power,
    Toughness,
    ManaValue,
    Loyalty,
    Defense,
    BasePower,
    BaseToughness,
}

/// Mirrors `Semantics/Words.lean:54`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardType {
    pub name: Ident,
    pub card_class: Option<CardClass>,
    pub stats: Vec<Stat>,
}

/// Mirrors `Semantics/Words.lean:119`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardClass {
    PermanentCard,
    InstantOrSorceryCard,
}

/// Mirrors `Semantics/Words.lean:132`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PlayerStat {
    LifeTotal,
    StartingLifeTotal,
    Speed,
}

/// Mirrors `Semantics/Words.lean:139`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Comparator {
    AtLeast,
    AtMost,
    Greater,
    Less,
    Eq,
}

/// Mirrors `Semantics/Words.lean:144`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QualitySort {
    Color,
    Subtype { host: CardType },
    CardName,
    Number,
    CardType,
    CounterKind,
    Ability,
}

/// Mirrors `Semantics/Words.lean:155`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Letter {
    X,
    Y,
}

/// Mirrors `Semantics/Words.lean:160`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    Object,
    Player,
    Quality { sort: QualitySort },
    Outcome,
    Letter { letter: Letter },
    TurnRef,
    Pile,
    Join { left: Box<Kind>, right: Box<Kind> },
}

/// Mirrors `Semantics/Words.lean:173`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AggregateOp {
    Sum,
    Min,
    Max,
}

/// Mirrors `Semantics/Words.lean:177`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubtypeScope {
    Any,
    BasicOnly,
    NonbasicOnly,
}

/// Mirrors `Semantics/Words.lean:182`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KindAxis {
    CardType,
    PermanentType,
    Color,
    Subtype { host: CardType, scope: SubtypeScope },
    Value { stat: Stat },
    CounterKind,
    ColorPair,
}

/// Mirrors `Semantics/Words.lean:194`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OutcomeSort {
    DamageDealt,
    DamageMarked,
    LifeGained,
    LifeLost,
    CountersPut,
    DamagePrevented,
    RollResult,
    CoinFlipped,
    DiceRolled,
    NamedNumber,
    RepeatCount,
    CountersRemoved,
    ManaAdded,
    ManaProduced,
    CeilingShortfall,
    VoteHeld,
}

/// Mirrors `Semantics/Words.lean:200`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FlipCall {
    Wins,
    Loses,
}

/// Mirrors `Semantics/Words.lean:204`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CoinFace {
    Heads,
    Tails,
}

/// Mirrors `Semantics/Words.lean:208`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RollExtreme {
    Lowest,
    Highest,
}

/// Mirrors `Semantics/Words.lean:212`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Plurality {
    One,
    Many,
}

/// Mirrors `Semantics/Words.lean:219`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Agreement {
    Share,
    Differ,
}

/// Mirrors `Semantics/Words.lean:250`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PossessorAxis {
    Owner,
    Controller,
}

/// Mirrors `Semantics/Words.lean:265`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombatRelation {
    AttackerOf,
    DeclaredAttacker,
    BlockerOf,
    BlockedBy,
    Unblocked,
    AttackedBy,
    CouldBlock,
    CouldBeBlockedBy,
}

/// Mirrors `Semantics/Words.lean:271`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombatRole {
    Attacking,
    Defending,
}

/// Mirrors `Semantics/Words.lean:276`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttachMove {
    Attached,
    Unattached,
}

/// Mirrors `Semantics/Words.lean:280`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ArithOp {
    Plus,
    Minus,
    Times,
    DifferenceBetween,
}

/// Mirrors `Semantics/Words.lean:284`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Zone {
    Battlefield,
    Graveyard,
    Exile,
    Hand,
    Library,
    Stack,
    Command,
}

/// Mirrors `Semantics/Words.lean:288`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PileEnd {
    Top,
    Bottom,
}

/// Mirrors `Semantics/Words.lean:292`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Arrangement {
    AnyOrder,
    RandomOrder,
}

/// Mirrors `Semantics/Words.lean:297`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Ordinal {
    Nth { n: u64 },
}

/// Mirrors `Semantics/Words.lean:304`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DesignationScope {
    HeldBy { holder: Box<Kind> },
    HeldByCard,
    HeldByGame,
}

/// Mirrors `Semantics/Words.lean:321`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DesignationLabel {
    Named {
        name: Ident,
        scope: DesignationScope,
        sole: bool,
    },
}

/// Mirrors `Semantics/Words.lean:329`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelationLabel {
    Named { name: Ident },
}

/// Mirrors `Semantics/Words.lean:334`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MergePosition {
    Top,
    Bottom,
}

/// Mirrors `Semantics/Words.lean:343`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordLabel {
    Named { name: Ident },
}

/// Mirrors `Semantics/Words.lean:349`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbilityWordLabel {
    Named { name: Ident },
}

/// Mirrors `Semantics/Words.lean:373`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CoreDeed {
    Target,
    Draw,
    Trigger,
    GainControl,
    FullyUnlock,
}

/// Mirrors `Semantics/Words.lean:378`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateBasedCause {
    NonpositiveLife,
}

/// Mirrors `Semantics/Words.lean:384`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeedPermanent {
    None,
    Performs,
    IsAbout,
}

/// Mirrors `Semantics/Words.lean:399`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Deed {
    Core {
        deed: CoreDeed,
    },
    Action {
        label: KeywordActionLabel,
        permanent: DeedPermanent,
    },
    OfAbility {
        keyword: KeywordLabel,
    },
}

/// Mirrors `Semantics/Words.lean:407`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PileFace {
    FaceDown,
    FaceUp,
}

/// Mirrors `Semantics/Words.lean:411`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChoiceSort {
    Quality { sort: QualitySort },
    Player,
}

/// Mirrors `Semantics/Words.lean:417`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DieSides {
    Sides { sides: u64 },
    ThoseDice,
    AnyDie,
}

/// Mirrors `Semantics/Words.lean:426`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChoiceRef {
    TheChoice,
    TheLatestChoice,
}

/// Mirrors `Semantics/Words.lean:430`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Disclosure {
    Openly,
    Secretly,
}

/// Mirrors `Semantics/Words.lean:434`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HiddenSort {
    Numbers,
    Choices,
}

/// Mirrors `Semantics/Words.lean:438`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExposeVerb {
    LookAt,
    Reveal,
}

/// Mirrors `Semantics/Words.lean:443`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Subtype {
    Of { host: CardType, label: Ident },
    Spell { label: Ident },
}

/// Mirrors `Semantics/Words.lean:451`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NounWord {
    Type {
        type_: CardType,
    },
    Subtype {
        subtype: Subtype,
    },
    Card,
    Spell,
    Player,
    Permanent,
    Token,
    Emblem,
    Copy,
    Join,
    Stack,
    Ability,
    OfType {
        word: Box<NounWord>,
        type_: CardType,
    },
    Copied {
        word: Box<NounWord>,
    },
    Pile,
    Source,
}

/// Mirrors `Semantics/Words.lean:480`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SlotCarrier {
    Permanent,
    Card,
    Spell,
    OffStack,
}

/// Mirrors `Semantics/Words.lean:485`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Reach {
    Bare,
    AtSlot { slot: SlotCarrier },
    Word { word: Box<NounWord> },
    UnionHalf { word: Box<NounWord> },
}

/// Mirrors `Semantics/Words.lean:498`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntryCounterMark {
    Fresh,
    Additional,
    Fewer,
}

/// Mirrors `Semantics/Words.lean:502`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RoundMode {
    Up,
    Down,
}

/// Mirrors `Semantics/Words.lean:510`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TargetExtent {
    SomeTarget,
    SoleTarget,
    Only,
}

/// Mirrors `Semantics/Words.lean:514`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CopySort {
    FromStack,
    FromCardZone,
    InZone { zone: Zone },
}

/// Mirrors `Semantics/Words.lean:522`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeywordFamily {
    pub word: KeywordLabel,
    pub sort: Option<QualitySort>,
}

/// Mirrors `Semantics/Words.lean:527`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordTerm {
    The { keyword: KeywordLabel },
    AnyIn { family: KeywordFamily },
    TheWith { keyword: KeywordLabel, number: u64 },
}

/// Mirrors `Semantics/Words.lean:537`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LinkSource {
    ByKeyword {
        keyword: KeywordLabel,
    },
    ByNthKeyword {
        ordinal: Ordinal,
        keyword: KeywordLabel,
    },
    TheAlternative,
    TheAdditional,
}

/// Mirrors `Semantics/Words.lean:544`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbilityClass {
    AnyOnStack,
    AnyActivated,
    AnyTriggered,
    Loyalty,
    Mana,
    Keyword { k: KeywordLabel },
}

/// Mirrors `Semantics/Words.lean:558`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItalicWord {
    AbilityWord { label: AbilityWordLabel },
    FlavorWord { label: FlavorWordLabel },
}

/// Mirrors `Semantics/Words.lean:563`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

/// Mirrors `Semantics/Words.lean:567`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorOrColorless {
    Colorless,
    Of { color: Color },
}

/// Mirrors `Semantics/Words.lean:572`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimpleManaSymbol {
    Generic { amount: u64 },
    Specific { color: ColorOrColorless },
}

/// Mirrors `Semantics/Words.lean:577`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaSymbol {
    Simple {
        symbol: SimpleManaSymbol,
    },
    Hybrid {
        left: SimpleManaSymbol,
        right: Color,
    },
    Phyrexian {
        color: Color,
        second: Option<Color>,
    },
    Variable,
    Snow,
}

/// Mirrors `Semantics/Words.lean:591`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubtypeSpace {
    BasicLand,
    Land,
    Creature,
}

/// Mirrors `Semantics/Words.lean:595`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PayTimes {
    Once,
    AnyNumberOfTimes,
    UpTo { times: u64 },
}

/// Mirrors `Semantics/Words.lean:601`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CostNamed {
    Containing { symbol: ManaSymbol },
    OfKeyword { keyword: KeywordLabel },
}

/// Mirrors `Semantics/Words.lean:606`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorFreedom {
    SameColor,
    EachColor,
    DistinctColors,
}

/// Mirrors `Semantics/Words.lean:610`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaMatch {
    AnyColor,
    AnyType,
    Of { color: ColorOrColorless },
}

/// Mirrors `Semantics/Words.lean:617`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Delta<T> {
    Up { amount: T },
    Down { amount: T },
    Set { amount: T },
}

/// Mirrors `Semantics/Words.lean:623`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Parity {
    Even,
    Odd,
}

/// Mirrors `Semantics/Words.lean:627`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Supertype {
    Legendary,
    Basic,
    Snow,
    World,
}

/// Mirrors `Semantics/Words.lean:631`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LockState {
    Locked,
    Unlocked,
}

/// Mirrors `Semantics/Words.lean:636`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttachmentSide {
    Host,
    Attachment,
}

/// Mirrors `Semantics/Words.lean:641`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardFaceSide {
    Front,
    Back,
}

/// Mirrors `Semantics/Words.lean:645`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttachWord {
    Enchanted,
    Equipped,
    Fortified,
}

/// Mirrors `Semantics/Words.lean:651`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OutcomeVerb {
    WinGame,
    LoseGame,
    DrawGame,
}

/// Mirrors `Semantics/Words.lean:670`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CounterKind {
    Named { name: Ident, holder: Box<Kind> },
}

/// Mirrors `Semantics/Words.lean:674`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProjAxis {
    Stat { stat: Stat },
    PlayerStat { stat: PlayerStat },
    Counter { kind: CounterKind },
    AnyCounter { kind: Box<Kind> },
    BoundCounter,
    MarkedDamage,
    Chapter,
    KeywordNumber { keyword: KeywordLabel },
}

/// Mirrors `Semantics/Words.lean:700`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Tapped,
    Untapped,
    Flipped,
    Unflipped,
    FaceUp,
    FaceDown,
    PhasedIn,
    PhasedOut,
}

/// Mirrors `Semantics/Words.lean:704`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorSpec {
    Some { colors: Vec<Color> },
    Every,
}

/// Mirrors `Semantics/Words.lean:710`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TurnPart {
    Turn,
    BeginningPhase,
    UntapStep,
    Upkeep,
    DrawStep,
    MainPhase,
    FirstMain,
    Combat,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeCombatDamage,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    EndingPhase,
    EndStep,
    Cleanup,
}

/// Mirrors `Semantics/Words.lean:726`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Period {
    Turn,
    YourTurn,
    Game,
    Part { part: TurnPart },
    ThisObject,
}

/// Mirrors `Semantics/Words.lean:735`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EventSide {
    Before,
    After,
}

/// Mirrors `Semantics/Events.lean:12`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PaymentOutcome {
    Paid,
    Unpaid,
}

/// Mirrors `Semantics/Events.lean:18`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TallyMode {
    Events,
    Quantity,
}

/// Mirrors `Semantics/Events.lean:23`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReplUse {
    Repeatedly,
    NextTimeOnly,
}

/// Mirrors `Semantics/Events.lean:33`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DamageKind {
    Combat,
    Noncombat,
}

/// Mirrors `Semantics/Events.lean:40`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CondMarking {
    AsLongAs,
    IfSo,
}

/// Mirrors `Semantics/Events.lean:45`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChoiceMode {
    Unmarked,
    TheirChoice,
    AtRandom,
    YourChoice,
}

/// Mirrors `Semantics/Abilities.lean:21`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObservationPoint {
    Arrival,
    Departure,
}

/// Mirrors `Semantics/Abilities.lean:25`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaHeld {
    ThisMana,
    Unspent { type_: Option<ColorOrColorless> },
}

/// Mirrors `Semantics/Abilities.lean:32`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextWord {
    Color { color: Color },
    Subtype { subtype: Subtype },
}

/// Mirrors `Semantics/Abilities.lean:39`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextWordSort {
    Color,
    LandType,
    CreatureType,
}

/// Mirrors `Semantics/Abilities.lean:43`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QualityOp {
    Adds,
    Sets,
    Loses,
}

/// Mirrors `Semantics/Abilities.lean:50`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TypeLineChanges {
    pub supertypes: Option<Vec<Supertype>>,
    pub types: Option<Vec<CardType>>,
    pub subtypes: Option<Vec<Subtype>>,
}

/// Mirrors `Semantics/Abilities.lean:56`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CharacteristicStat {
    Power,
    Toughness,
    Loyalty,
    Defense,
}

/// Mirrors `Semantics/Abilities.lean:60`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpentMode {
    AffectsIt,
    TriggersThen,
}

/// Mirrors `Semantics/Abilities.lean:66`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ContinuationPolicy {
    Optional,
    Required,
}

/// Mirrors `Semantics/Abilities.lean:73`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NameSource {
    Printed { name: Ident },
    Chosen,
}

/// Mirrors `Semantics/Abilities.lean:81`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Coordinator {
    And,
    Or,
    AndOr,
}

/// Mirrors `Semantics/Abilities.lean:97`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Location {
    InCaller {
        body: Box<Location>,
    },
    Zone {
        zone: Zone,
        whose: Option<Box<NounPhrase>>,
    },
    FromEnd {
        end_: PileEnd,
        ordinal: Ordinal,
        loc: Box<Location>,
    },
    ExiledWith {
        source: Box<NounPhrase>,
    },
    Pile {
        pile: Box<NounPhrase>,
    },
    AnyLocation,
    Not {
        loc: Box<Location>,
    },
    Coord {
        coordinator: Coordinator,
        locs: Vec<Location>,
    },
}

/// Mirrors `Semantics/Abilities.lean:118`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChoiceDomain {
    NameOfCard { predicate: Box<Predicate> },
    ColorOtherThan { color: Color },
    TypeOtherThan { subtype: Subtype },
    BasicTypesOnly,
    NonbasicTypesOnly,
    Number { quantity: Box<Quantity> },
    Players { predicate: Box<Predicate> },
    AbilitiesAmong { keywords: Vec<KeywordTerm> },
}

/// Mirrors `Semantics/Abilities.lean:131`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Subject {
    Actor { np: Box<NounPhrase> },
    Any,
    By { np: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:137`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LookbackSpan {
    Duration {
        d: Box<Duration>,
        timing: Option<Box<Timing>>,
    },
    ThisWay,
    Triggering,
    ThisObject,
}

/// Mirrors `Semantics/Abilities.lean:159`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DidClause {
    pub subj: Box<Subject>,
    pub action: Box<Action>,
    pub within: Option<Box<LookbackSpan>>,
    pub ordinal: Option<(Ordinal, Option<Period>)>,
}

/// Mirrors `Semantics/Abilities.lean:167`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TurnRef {
    Current,
    Previous { player: Option<Box<NounPhrase>> },
    Next { player: Option<Box<NounPhrase>> },
    Each { player: Option<Box<NounPhrase>> },
    Later,
    That,
    MostRecent { player: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:186`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Timing {
    InPart {
        part: TurnPart,
        whose: Option<Box<NounPhrase>>,
        which: Box<TurnRef>,
    },
    RelativeToEvent {
        side: EventSide,
        event: Box<GameEvent>,
    },
    WhileDoing {
        event: Box<GameEvent>,
    },
    OnlyIf {
        condition: Box<Condition>,
    },
    And {
        conjuncts: Vec<Timing>,
    },
    Or {
        disjuncts: Vec<Timing>,
    },
    Not {
        timing: Box<Timing>,
    },
    AnyTiming,
}

/// Mirrors `Semantics/Abilities.lean:203`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Duration {
    Between {
        start: Option<Box<GameEvent>>,
        end_: Option<Box<GameEvent>>,
    },
    ForAsLongAs {
        condition: Box<Condition>,
    },
    ModifiesPrecedingAction,
}

/// Mirrors `Semantics/Abilities.lean:215`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LinkedFacet {
    ColorsSpent,
    ManaValueSpent,
    PhyrexianPaidWithLife,
    TimesPaid {
        which: LinkSource,
    },
    Readback {
        which: LinkSource,
        within: Option<Box<Duration>>,
    },
    Taken {
        which: LinkSource,
    },
}

/// Mirrors `Semantics/Abilities.lean:232`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Predicate {
    InCaller {
        body: Box<Predicate>,
    },
    HasType {
        type_: CardType,
    },
    HasSubtype {
        subtype: Subtype,
    },
    HasSupertype {
        supertype: Supertype,
    },
    IsPlayer,
    IsOpponentOf {
        who: Box<NounPhrase>,
    },
    IsTeammateOf {
        who: Box<NounPhrase>,
    },
    Chosen {
        ref_: ChoiceRef,
    },
    QualityNoun {
        sort: QualitySort,
        domain: Option<Box<ChoiceDomain>>,
    },
    CounterKindOn {
        subject: Box<NounPhrase>,
    },
    OfChosen {
        ref_: ChoiceRef,
        sort: QualitySort,
    },
    OfYourChoice {
        sort: QualitySort,
        domain: Option<Box<ChoiceDomain>>,
    },
    EachKind {
        axis: KindAxis,
        domain: Option<Box<NounPhrase>>,
    },
    HasKeyword {
        keyword: KeywordTerm,
    },
    HasPossessor {
        axis: PossessorAxis,
        possessor: Box<NounPhrase>,
    },
    HeldBeforeCast {
        designation: DesignationLabel,
    },
    InCombat {
        relation: CombatRelation,
        counterpart: Option<Box<NounPhrase>>,
    },
    Did {
        clause: Box<DidClause>,
    },
    ColorIs {
        color: Color,
    },
    ColorCount {
        comparator: Comparator,
        count: u64,
    },
    Named {
        source: NameSource,
    },
    Shares {
        axis: QualitySort,
        agreement: Agreement,
        with_: Box<NounPhrase>,
    },
    HasDesignation {
        designation: DesignationLabel,
        holder: Option<Box<NounPhrase>>,
    },
    Related {
        label: RelationLabel,
        counterpart: Option<Box<NounPhrase>>,
    },
    Attachment {
        side: AttachmentSide,
        word: Option<AttachWord>,
        counterpart: Option<Box<NounPhrase>>,
    },
    LegalAttachment {
        side: AttachmentSide,
        counterpart: Box<NounPhrase>,
    },
    IsCard,
    IsToken,
    IsEmblem,
    IsCopyOfACard,
    HasTypeIgnoringTypeChanges {
        type_: CardType,
    },
    CurrentFace {
        side: CardFaceSide,
    },
    HasStatus {
        status: Status,
    },
    HasCounters {
        kind: Option<CounterKind>,
    },
    Compare {
        axes: Vec<ProjAxis>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    Superlative {
        op: AggregateOp,
        axis: ProjAxis,
        domain: Box<Predicate>,
    },
    WithMostVotes,
    ChoseExtreme {
        op: AggregateOp,
    },
    CompareOver {
        domain: Box<Predicate>,
        measure: Box<Amount>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    In {
        loc: Box<Location>,
    },
    Coord {
        coordinator: Coordinator,
        items: Vec<Predicate>,
    },
    Not {
        predicate: Box<Predicate>,
    },
    Other,
    NotChosen,
    OtherThan {
        anchor: Box<NounPhrase>,
    },
    CoinCameUp {
        face: CoinFace,
    },
    IsSource,
    ManaCostHas {
        symbol: ManaSymbol,
    },
    AbilityHead {
        class_: AbilityClass,
    },
    AbilityOf {
        source: Box<NounPhrase>,
    },
    ActivationCostHas {
        cost: Box<Cost>,
    },
    TriggeredNotLeftStack,
    Targets {
        subject: Box<NounPhrase>,
        extent: TargetExtent,
    },
}

/// Mirrors `Semantics/Abilities.lean:338`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DetPhrase {
    Target {
        quantity: Box<Quantity>,
    },
    A {
        mode: ChoiceMode,
    },
    Each,
    All,
    The {
        plur: Plurality,
    },
    Count {
        quantity: Box<Quantity>,
        mode: Option<ChoiceMode>,
    },
}

/// Mirrors `Semantics/Abilities.lean:349`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorTerm {
    Lit { color: Color },
    Chosen { ref_: ChoiceRef },
    ColorOf { object: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:358`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaTypeTerm {
    Colorless,
    OfColor { color: Box<ColorTerm> },
}

/// Mirrors `Semantics/Abilities.lean:362`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NounPhrase {
    InCaller {
        body: Box<NounPhrase>,
    },
    Which,
    This,
    Headed {
        head: Box<NounWord>,
        subject: Box<NounPhrase>,
    },
    Becomes {
        subject: Box<NounPhrase>,
    },
    TheGrantor {
        head: Box<NounWord>,
    },
    You,
    Actor,
    CombatPlayer {
        role: CombatRole,
    },
    Described {
        determiner: Box<DetPhrase>,
        predicate: Box<Predicate>,
    },
    EachOf {
        group: Box<NounPhrase>,
    },
    Coord {
        coordinator: Coordinator,
        phrases: Vec<NounPhrase>,
    },
    Slice {
        end_: PileEnd,
        amount: Box<Amount>,
        pile: Box<Location>,
    },
    Run {
        end_: PileEnd,
        stop: Box<Predicate>,
        pile: Box<Location>,
    },
    SomeOf {
        count: Box<SliceCount>,
        description: Option<Box<Predicate>>,
        group: Box<NounPhrase>,
    },
    Agree {
        axis: QualitySort,
        agreement: Agreement,
        group: Box<NounPhrase>,
    },
    TheRest {
        plurality: Plurality,
    },
    PileOf {
        count: Box<SliceCount>,
        by_: Option<Box<NounPhrase>>,
    },
    Pro {
        reach: Reach,
        plurality: Plurality,
    },
    Refl {
        plurality: Plurality,
    },
    FirstMention {
        id: BindingId,
        phrase: Box<NounPhrase>,
    },
    LaterMention {
        id: BindingId,
        phrase: Box<NounPhrase>,
    },
    AttachHost {
        word: AttachWord,
        head: Box<NounWord>,
    },
    PossessorOf {
        axis: PossessorAxis,
        subject: Box<NounPhrase>,
    },
    Designated {
        designation: DesignationLabel,
        whose: Box<NounPhrase>,
    },
    Counterpart {
        label: RelationLabel,
        subject: Box<NounPhrase>,
    },
    OneEachOf {
        roles: Vec<Predicate>,
        pool: Box<NounPhrase>,
    },
    Any,
}

/// Mirrors `Semantics/Abilities.lean:426`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Amount {
    InCaller {
        body: Box<Amount>,
    },
    Lit {
        value: i64,
    },
    StatOf {
        axis: ProjAxis,
        subject: Box<NounPhrase>,
    },
    CountOf {
        group: Box<NounPhrase>,
    },
    Aggregate {
        op: AggregateOp,
        axis: ProjAxis,
        group: Box<NounPhrase>,
    },
    Paid {
        facet: Box<LinkedFacet>,
        subject: Box<NounPhrase>,
    },
    Tally {
        clause: Box<DidClause>,
        mode: TallyMode,
    },
    ThatMuch,
    ChosenNumber {
        ref_: ChoiceRef,
    },
    VotesFor {
        label: VoteLabel,
    },
    TheOutcome {
        sort: OutcomeSort,
    },
    CoinsShowing {
        face: CoinFace,
    },
    GreatestStoredMatch {
        subject: Box<NounPhrase>,
    },
    GroupSize,
    TheDifference,
    Letter {
        letter: Letter,
    },
    Arith {
        op: ArithOp,
        left: Box<Amount>,
        right: Box<Amount>,
    },
    Devotion {
        player: Box<NounPhrase>,
        color: Box<ColorTerm>,
        second: Option<Box<ColorTerm>>,
    },
    Half {
        rounding: RoundMode,
        amount: Box<Amount>,
    },
    AggregateOver {
        op: AggregateOp,
        domain: Box<Predicate>,
        body: Box<Amount>,
    },
    AggregateOf {
        op: AggregateOp,
        amounts: Vec<Amount>,
    },
    DistinctCount {
        axis: KindAxis,
        domain: Box<NounPhrase>,
    },
    AnyAmount,
    TheNth {
        ordinal: Ordinal,
    },
}

/// Mirrors `Semantics/Abilities.lean:460`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Quantity {
    InCaller { body: Box<Quantity> },
    Range { low: Option<u64>, high: Option<u64> },
    UpToOf { amount: Box<Amount> },
    ExactlyOf { amount: Box<Amount> },
    Worth { n: u64 },
    MoreThanOf { amount: Box<Amount> },
    Additional { quantity: Box<Quantity> },
    TheLast,
}

/// Mirrors `Semantics/Abilities.lean:479`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SliceCount {
    Counted {
        quantity: Box<Quantity>,
    },
    Whole,
    Totalling {
        axis: ProjAxis,
        comparator: Comparator,
        bound: Box<Amount>,
    },
}

/// Mirrors `Semantics/Abilities.lean:487`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Door {
    ThisDoor,
    DoorOf {
        state: Option<LockState>,
        room: Box<NounPhrase>,
    },
}

/// Mirrors `Semantics/Abilities.lean:492`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RollWatch {
    AnyResult,
    ResultIn { quantity: Box<Quantity> },
    HighestNatural,
}

/// Mirrors `Semantics/Abilities.lean:497`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Condition {
    InCaller {
        body: Box<Condition>,
    },
    DuringPart {
        part: TurnPart,
        whose: Option<Box<NounPhrase>>,
    },
    Exists {
        subject: Box<NounPhrase>,
    },
    Did {
        clause: Box<DidClause>,
    },
    Matches {
        subject: Box<NounPhrase>,
        predicate: Box<Predicate>,
    },
    GameHas {
        designation: DesignationLabel,
    },
    CompareAmt {
        subject: Box<Amount>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    VoteLead {
        label: VoteLabel,
        or_tied: bool,
    },
    AnyResultIs {
        comparator: Comparator,
        bound: Box<Amount>,
    },
    RolledDoubles,
    HasPriority {
        player: Box<NounPhrase>,
    },
    SoChooses {
        chooser: Box<NounPhrase>,
    },
    HeldSince {
        condition: Box<Condition>,
        since: Box<GameEvent>,
    },
    CouldBeginToCast {
        player: Box<NounPhrase>,
        subject: Box<NounPhrase>,
        from_: Box<Location>,
    },
    Not {
        condition: Box<Condition>,
    },
    And {
        conjuncts: Vec<Condition>,
    },
    Or {
        disjuncts: Vec<Condition>,
    },
}

/// Mirrors `Semantics/Abilities.lean:539`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GameEvent {
    InCaller {
        body: Box<GameEvent>,
    },
    Does {
        subj: Box<Subject>,
        action: Box<Action>,
    },
    BeginningOf {
        part: TurnPart,
        whose: Option<Box<NounPhrase>>,
        which: Box<TurnRef>,
    },
    EndOf {
        part: TurnPart,
        whose: Option<Box<NounPhrase>>,
        which: Box<TurnRef>,
    },
    Now,
    StateHolds {
        condition: Box<Condition>,
    },
    ChapterMark {
        chapters: Vec<ChapterNumber>,
    },
    NthOccurrence {
        ordinal: Ordinal,
        per: Option<Period>,
        event: Box<GameEvent>,
    },
    Causes {
        cause: Box<Causing>,
        event: Box<GameEvent>,
    },
    StateBasedActionsChecked,
}

/// Mirrors `Semantics/Abilities.lean:561`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Causing {
    Source { source: Box<NounPhrase> },
    Event { event: Box<GameEvent> },
    AnEffect,
}

/// Mirrors `Semantics/Abilities.lean:566`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FlipScope {
    Count { amount: Box<Amount> },
    Per { each: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:570`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IgnoredOutcomes {
    Extreme {
        extreme: RollExtreme,
    },
    AllBut {
        extreme: RollExtreme,
    },
    Chosen {
        chooser: Option<Box<NounPhrase>>,
        amount: Box<Amount>,
    },
}

/// Mirrors `Semantics/Abilities.lean:575`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Ballot {
    AnyBallot,
    ByLabel { options: Vec<VoteLabel> },
    ByCandidate { candidates: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:580`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Exposed {
    Cards { cards: Box<NounPhrase> },
    Zone { zone: Box<Location> },
    Choice { sort: ChoiceSort },
}

/// Mirrors `Semantics/Abilities.lean:585`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VisibleThing {
    TopOfLibrary,
    WholeHand,
    Objects { objects: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:594`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UsageLimit {
    Mk { times: Box<Amount>, per: Period },
}

/// Mirrors `Semantics/Abilities.lean:598`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JoinedHeader {
    pub event: Box<GameEvent>,
    pub alternatives: Vec<GameEvent>,
    pub while_: Option<Box<Timing>>,
    pub timing: Option<Box<Timing>>,
}

/// Mirrors `Semantics/Abilities.lean:607`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpendPurpose {
    ToCast { predicate: Box<Predicate> },
    ToActivate { source: Option<Box<Predicate>> },
    ToPay { cost: CostNamed },
    ToTurnFaceUp { subject: Box<Predicate> },
}

/// Mirrors `Semantics/Abilities.lean:616`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AsThough {
    Condition {
        cond: Box<Condition>,
    },
    Mana {
        what: Option<ColorOrColorless>,
        as_: ManaMatch,
        purpose: Option<Box<SpendPurpose>>,
    },
    Greater {
        stat: Stat,
        amount: Box<Amount>,
    },
}

/// Mirrors `Semantics/Abilities.lean:623`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Exchanged {
    LifeTotals {
        parties: Box<NounPhrase>,
    },
    ControlOf {
        left: Box<NounPhrase>,
        right: Box<NounPhrase>,
    },
    CardsAcross {
        left: Box<NounPhrase>,
        right: Box<NounPhrase>,
    },
    Zones {
        left: Box<Location>,
        right: Box<Location>,
    },
    Values {
        left: Box<Amount>,
        right: Box<Amount>,
    },
}

/// Mirrors `Semantics/Abilities.lean:633`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextChange {
    RemoveBracketed,
    TargetToEach,
    AppendTextOf {
        source: Box<NounPhrase>,
    },
    ReplaceWord {
        sorts: Vec<TextWordSort>,
        from_: Option<TextWord>,
        to: Option<TextWord>,
    },
    ExchangeWith {
        other: Box<NounPhrase>,
    },
}

/// Mirrors `Semantics/Abilities.lean:657`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WrittenQuality {
    EveryTypeOf { space: SubtypeSpace },
    Chosen { quality: Box<Predicate> },
}

/// Mirrors `Semantics/Abilities.lean:662`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CounterKindSource {
    Printed { kind: CounterKind },
    Chosen { menu: Vec<CounterKind> },
    DistinctChosen { menu: Vec<CounterKind> },
    Bound,
    Those,
    SameAs { source: Box<NounPhrase> },
    AnyKind,
}

/// Mirrors `Semantics/Abilities.lean:680`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaCostTerm {
    Lit { cost: ManaCost },
    ManaCostOf { object: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:686`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CostShift {
    Generic {
        change: Delta<Box<Amount>>,
        floor: Option<Box<Amount>>,
    },
    BySymbols {
        change: Delta<Box<ManaCostTerm>>,
        colored_only: bool,
    },
}

/// Mirrors `Semantics/Abilities.lean:702`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PipPaid {
    Generic,
    OfColor { color: Box<ColorTerm> },
}

/// Mirrors `Semantics/Abilities.lean:711`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeonticExcept {
    ByKeyword { label: KeywordLabel },
    ByObjects { objects: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:718`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PreventCut {
    All,
    Some { amount: Box<Amount> },
    Shield { amount: Box<Amount> },
    AllBut { amount: Box<Amount> },
    Half { rounding: RoundMode },
}

/// Mirrors `Semantics/Abilities.lean:725`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DividedVerb {
    Damage { source: Box<NounPhrase> },
    Counters { kind: CounterKind },
}

/// Mirrors `Semantics/Abilities.lean:729`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProducedMana {
    Runs { runs: Vec<ProducedRun> },
    AnyColor { freedom: ColorFreedom },
    OfChosenColor { alternative: Option<ProducedRun> },
    AsPrintedCost { subject: Box<NounPhrase> },
    ProducedByEvent { subject: Box<NounPhrase> },
    CouldProduce { subject: Box<NounPhrase> },
    AmongColorsOf { subject: Box<NounPhrase> },
    AmongWritten { colors: Vec<Color> },
    LastNoted { subject: Box<NounPhrase> },
}

/// Mirrors `Semantics/Abilities.lean:740`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeckTrait {
    ACharacteristic { predicate: Box<Predicate> },
    ManaValueParity { parity: Parity },
    RepeatedManaSymbol,
    HasAbilityOf { class_: AbilityClass },
    AnyOf { traits: Vec<DeckTrait> },
}

/// Mirrors `Semantics/Abilities.lean:751`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeckCondition {
    EveryCardIs {
        scope: Box<Predicate>,
        trait_: Box<DeckTrait>,
    },
    NoCardIs {
        scope: Box<Predicate>,
        trait_: Box<DeckTrait>,
    },
    GroupAgrees {
        group: Box<NounPhrase>,
    },
    DeckSizeOverMinimum {
        extra: u64,
    },
}

/// Mirrors `Semantics/Abilities.lean:766`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombatParticipation {
    Attacking { defender: Option<Box<NounPhrase>> },
    Blocked { value: bool },
    OutsideCombat,
}

/// Mirrors `Semantics/Abilities.lean:771`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombatUpdate {
    Participation {
        state: Box<CombatParticipation>,
    },
    Blocking {
        move_: AttachMove,
        attacker: Box<NounPhrase>,
    },
}

/// Mirrors `Semantics/Abilities.lean:777`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Repetition {
    Again,
    MoreTimes {
        times: Box<Amount>,
    },
    AnyNumber,
    UntilCond {
        condition: Box<Condition>,
    },
    AgainExcludingChosen,
    Fixed {
        times: Box<Amount>,
        body: Box<Action>,
    },
}

/// Mirrors `Semantics/Abilities.lean:788`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Characteristics {
    pub name: Option<Ident>,
    pub cost: Option<ManaCost>,
    pub colors: Vec<Color>,
    pub supertypes: Vec<Supertype>,
    pub types: Vec<CardType>,
    pub subtypes: Vec<Subtype>,
    pub text: Vec<Ability>,
    pub power: Option<Box<Amount>>,
    pub toughness: Option<Box<Amount>>,
    pub loyalty: Option<Box<Amount>>,
    pub defense: Option<Box<Amount>>,
}

/// Mirrors `Semantics/Abilities.lean:804`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CharacteristicBundle {
    pub characteristics: Box<Characteristics>,
    pub qualities: Vec<WrittenQuality>,
}

/// Mirrors `Semantics/Abilities.lean:812`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FaceDown {
    pub characteristics: Option<Box<Characteristics>>,
}

/// Mirrors `Semantics/Abilities.lean:816`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CharacteristicEdit {
    TypeLine {
        op: QualityOp,
        changes: TypeLineChanges,
    },
    Name {
        op: QualityOp,
        name: Ident,
    },
    ManaCost {
        cost: Option<ManaCost>,
    },
    Colors {
        op: QualityOp,
        colors: ColorSpec,
    },
    Stat {
        op: QualityOp,
        axis: CharacteristicStat,
        amount: Box<Amount>,
    },
    PtSwitch,
    Quality {
        op: QualityOp,
        quality: Box<WrittenQuality>,
    },
    AddedAbilities {
        abilities: Vec<Ability>,
    },
    AddedAbilitiesOf {
        source: Box<NounPhrase>,
        classes: Vec<AbilityClass>,
        except: Option<Box<Predicate>>,
    },
    AddedSelectionOf {
        source: Box<NounPhrase>,
        selection: Box<AbilitySelection>,
    },
    RemovedAbilities {
        selection: Box<AbilitySelection>,
    },
}

/// Mirrors `Semantics/Abilities.lean:840`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbilitySelection {
    Specified { abilities: Vec<AbilityLost> },
    Family { class_: AbilityClass },
    AllExcept { except: Option<Box<Predicate>> },
    PrintedAfter { keyword: KeywordLabel },
}

/// Mirrors `Semantics/Abilities.lean:849`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TokenSpec {
    InCaller {
        body: Box<TokenSpec>,
    },
    Written {
        characteristics: Box<CharacteristicBundle>,
    },
    AsThose,
    CopyOf {
        source: Box<NounPhrase>,
        exceptions: Vec<CopyExcept>,
    },
    DoubleFaced {
        front: Box<CharacteristicBundle>,
        back: Box<CharacteristicBundle>,
    },
}

/// Mirrors `Semantics/Abilities.lean:862`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WouldClause {
    Would {
        subj: Box<Subject>,
        action: Box<Action>,
    },
    WouldAny {
        clauses: Vec<WouldClause>,
    },
    CausedBy {
        cause: Box<Causing>,
        clause: Box<WouldClause>,
    },
    NthOccurrence {
        ordinal: Ordinal,
        per: Option<Period>,
        clause: Box<WouldClause>,
    },
}

/// Mirrors `Semantics/Abilities.lean:877`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Replacement {
    InCaller {
        body: Box<Replacement>,
    },
    Instead {
        action: Box<Action>,
    },
    Before {
        action: Box<Action>,
        body: Option<Box<Replacement>>,
    },
    Alongside {
        action: Box<Action>,
    },
    WithRiders {
        riders: Vec<MoveRider>,
    },
    OnlyIf {
        condition: Box<Condition>,
        body: Box<Replacement>,
    },
}

/// Mirrors `Semantics/Abilities.lean:914`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeonticMods {
    pub timing: Option<Box<Timing>>,
    pub as_though: Option<Box<AsThough>>,
    pub except: Option<Box<DeonticExcept>>,
    pub cause: Option<Box<Causing>>,
    pub rider: Option<Box<Action>>,
}

/// Mirrors `Semantics/Abilities.lean:939`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StaticSpec {
    InCaller {
        body: Box<StaticSpec>,
    },
    CostShift {
        subject: Box<NounPhrase>,
        shift: Box<CostShift>,
    },
    AltCost {
        subject: Box<NounPhrase>,
        cost: Option<Box<Cost>>,
    },
    AddedCost {
        cost: Box<Cost>,
        offered: bool,
        times: PayTimes,
    },
    LetterDefinition {
        letter: Letter,
        amount: Box<Amount>,
    },
    Cant {
        subj: Box<Subject>,
        action: Box<Action>,
        mods: Box<DeonticMods>,
    },
    Must {
        subj: Box<Subject>,
        action: Box<Action>,
        mods: Box<DeonticMods>,
    },
    Can {
        subj: Box<Subject>,
        action: Box<Action>,
        mods: Box<DeonticMods>,
    },
    AsThough {
        subj: Box<Subject>,
        action: Box<Action>,
        as_though: Box<AsThough>,
    },
    GatedBy {
        cost: Box<Cost>,
        subj: Box<Subject>,
        action: Box<Action>,
        mods: Box<DeonticMods>,
    },
    ManaRetention {
        player: Box<NounPhrase>,
        mana: ManaHeld,
    },
    PartSkip {
        player: Box<NounPhrase>,
        part: TurnPart,
    },
    CharacteristicChange {
        subject: Box<NounPhrase>,
        edits: Vec<CharacteristicEdit>,
    },
    CharacteristicDefining {
        subject: Box<NounPhrase>,
        edits: Vec<CharacteristicEdit>,
    },
    Retention {
        spec: Box<StaticSpec>,
        subject: Box<NounPhrase>,
    },
    CopyChange {
        subject: Box<NounPhrase>,
        source: Box<NounPhrase>,
        exceptions: Vec<CopyExcept>,
    },
    ControlGrant {
        player: Box<NounPhrase>,
        subject: Box<NounPhrase>,
    },
    Replace {
        event: Box<WouldClause>,
        replacement: Box<Replacement>,
        timing: Option<Box<Timing>>,
        use_: ReplUse,
        limit: Option<Box<UsageLimit>>,
    },
    Conditional {
        spec: Box<StaticSpec>,
        condition: Box<Condition>,
        marking: CondMarking,
    },
    Visibility {
        verb: ExposeVerb,
        player: Box<NounPhrase>,
        what: Box<VisibleThing>,
    },
    AdditionalTriggers {
        event: Box<GameEvent>,
        quantity: Box<Quantity>,
    },
    Conjunction {
        subject: Option<Box<NounPhrase>>,
        parts: Vec<StaticSpec>,
    },
    PaySubstitute {
        subject: Box<NounPhrase>,
        substitute: Box<ManaSubstitute>,
    },
    SpecialAction {
        player: Option<Box<NounPhrase>>,
        when: Option<Box<Timing>>,
        cost: Box<Cost>,
        does: Box<Action>,
    },
    AsEvent {
        event: Box<GameEvent>,
        does: Box<Action>,
    },
    ResolutionDelayedTrigger {
        does: Box<Action>,
    },
    WhileHolds {
        condition: Box<Condition>,
        does: Box<Action>,
    },
    RollModifier {
        roll: Box<GameEvent>,
        modify: Box<Action>,
        limit: Option<Box<UsageLimit>>,
    },
    Combatant {
        subject: Box<NounPhrase>,
    },
    ModeChoice {
        change: Box<ModeChange>,
    },
    AddsTarget {
        subject: Box<NounPhrase>,
        description: Box<Predicate>,
    },
    TextChange {
        subject: Box<NounPhrase>,
        change: Box<TextChange>,
    },
    RevealedAsCast {
        spell: Box<NounPhrase>,
        grants: Box<StaticSpec>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1099`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModeChange {
    AllInstead {
        cost: Option<Box<Cost>>,
        condition: Option<Box<Condition>>,
    },
    ExtraModesCost {
        cost: Box<Cost>,
    },
    SetQuantity {
        quantity: Box<Quantity>,
    },
    Repeatable,
    NotChosenBefore {
        within: Box<Duration>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1113`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PlayPayment {
    ItsOwnCost,
    WithoutPaying,
    PayingInstead {
        cost: Box<Cost>,
    },
    PlusAdditional {
        cost: Box<Cost>,
    },
    FaceDown {
        spec: Box<FaceDown>,
        cost: Box<Cost>,
    },
    Prototyped,
    Transformed {
        cost: Box<Cost>,
    },
    Fused,
    ByKeyword {
        label: KeywordLabel,
    },
}

/// Mirrors `Semantics/Abilities.lean:1144`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MoveRider {
    InCaller {
        body: Box<MoveRider>,
    },
    EntersAs {
        status: Status,
    },
    EntersAttacking {
        defender: Option<Box<NounPhrase>>,
    },
    EntersTransformed,
    EntersMelded {
        into: Ident,
    },
    WithCounters {
        amount: Box<Amount>,
        kind: Box<CounterKindSource>,
        mark: EntryCounterMark,
    },
    Under {
        controller: Box<NounPhrase>,
    },
    AsCopyOf {
        optional: bool,
        source: Box<NounPhrase>,
        exceptions: Vec<CopyExcept>,
    },
    FaceDown {
        spec: Box<FaceDown>,
    },
    InOrder {
        arrangement: Arrangement,
    },
    AttachedTo {
        host: Box<NounPhrase>,
    },
    Not {
        rider: Box<MoveRider>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1175`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Cost {
    AnyCost,
    InCaller {
        body: Box<Cost>,
    },
    Mana {
        cost: ManaCost,
    },
    Scaled {
        cost: Box<Cost>,
        amount: Box<Amount>,
    },
    TapSymbol,
    UntapSymbol,
    LoyaltySymbol {
        change: Delta<Box<Amount>>,
    },
    Perform {
        action: Box<Action>,
    },
    Compound {
        costs: Vec<Cost>,
    },
    Or {
        costs: Vec<Cost>,
    },
    ItsManaCost,
    WithSubstitute {
        cost: Box<Cost>,
        substitute: Box<ManaSubstitute>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1198`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ManaSubstitute {
    pub action: Box<Action>,
    pub pays: Vec<PipPaid>,
}

/// Mirrors `Semantics/Abilities.lean:1202`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ManaRider {
    SpendOnly {
        purposes: Vec<SpendPurpose>,
    },
    SpendNotOn {
        purposes: Vec<SpendPurpose>,
    },
    OnSpent {
        mode: SpentMode,
        only: bool,
        spell: Box<NounPhrase>,
        says: Box<Action>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1207`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CopyExcept {
    Edits {
        edits: Vec<CharacteristicEdit>,
    },
    Ability {
        ability: Box<Ability>,
    },
    ThisAbility,
    EntersWithCounters {
        amount: Box<Amount>,
        kind: CounterKind,
        mark: EntryCounterMark,
    },
}

/// Mirrors `Semantics/Abilities.lean:1213`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RollRow {
    pub results: Box<Quantity>,
    pub action: Box<Action>,
}

/// Mirrors `Semantics/Abilities.lean:1217`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreationSpec {
    Token {
        spec: Box<TokenSpec>,
        riders: Vec<MoveRider>,
    },
    Emblem {
        name: Option<Ident>,
        abilities: Vec<Ability>,
    },
}

/// Mirrors `Semantics/Abilities.lean:1222`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Action {
    InCaller {
        body: Box<Action>,
    },
    OwnScope {
        deed: Option<Deed>,
        body: Box<Action>,
    },
    DealDamage {
        amount: Box<Amount>,
        recipient: Box<NounPhrase>,
        kind: Option<DamageKind>,
    },
    MarkDamage {
        subject: Box<NounPhrase>,
        amount: Box<Amount>,
    },
    PreventDamage {
        cut: Box<PreventCut>,
        damage: Option<Box<WouldClause>>,
        also: Option<Box<Action>>,
    },
    RedirectDamage {
        cut: Box<PreventCut>,
        to: Box<NounPhrase>,
        damage: Option<Box<WouldClause>>,
    },
    ClearDamage {
        subject: Box<NounPhrase>,
    },
    SetStatus {
        status: Status,
        subject: Box<NounPhrase>,
    },
    TurnOver {
        subject: Box<NounPhrase>,
        into: Option<Box<Predicate>>,
    },
    Combat {
        subject: Box<NounPhrase>,
        update: Box<CombatUpdate>,
    },
    Attack {
        defender: Option<Box<NounPhrase>>,
    },
    Block {
        attacker: Box<NounPhrase>,
    },
    AttackUnblocked {
        defender: Option<Box<NounPhrase>>,
    },
    AttackWith {
        attackers: Box<NounPhrase>,
        defender: Option<Box<NounPhrase>>,
    },
    Trigger,
    Crew {
        vehicle: Box<NounPhrase>,
    },
    Saddle {
        permanent: Box<NounPhrase>,
    },
    Target {
        subject: Box<NounPhrase>,
    },
    TapForMana {
        permanent: Box<NounPhrase>,
        type_: Option<Box<ManaTypeTerm>>,
    },
    CommitCrime,
    Attachment {
        move_: AttachMove,
        subject: Box<NounPhrase>,
        host: Option<Box<NounPhrase>>,
    },
    ChangeControl {
        subject: Box<NounPhrase>,
        from_: Option<Box<NounPhrase>>,
    },
    GainDesignation {
        subject: Box<NounPhrase>,
        designation: DesignationLabel,
        duration: Option<Box<Duration>>,
    },
    GameGains {
        designation: DesignationLabel,
    },
    Unlock {
        door: Box<Door>,
    },
    FullyUnlock {
        room: Box<NounPhrase>,
    },
    Conclude {
        verb: OutcomeVerb,
        cause: Option<StateBasedCause>,
    },
    RestartGame,
    SeparateIntoPiles {
        group: Box<NounPhrase>,
        piles: u64,
        faces: Vec<PileFace>,
    },
    Choose {
        first: Option<Box<NounPhrase>>,
        chosen: Box<NounPhrase>,
        disclosure: Disclosure,
        when: Option<Box<Timing>>,
    },
    RevealChoices {
        sort: HiddenSort,
    },
    Vote {
        first: Option<Box<NounPhrase>>,
        disclosure: Disclosure,
        ballot: Box<Ballot>,
        times: Option<Box<Quantity>>,
    },
    Move {
        subject: Box<NounPhrase>,
        from_: Box<Location>,
        to: Option<Box<Location>>,
        at_: ObservationPoint,
        riders: Vec<MoveRider>,
    },
    TurnFaceDown {
        subject: Box<NounPhrase>,
        spec: Box<FaceDown>,
    },
    Copy {
        sort: CopySort,
        subject: Box<NounPhrase>,
        times: Box<Amount>,
        exceptions: Vec<CopyExcept>,
    },
    ChooseNewTargets {
        subject: Box<NounPhrase>,
    },
    CopyTargets {
        copy: Box<NounPhrase>,
        original: Box<NounPhrase>,
    },
    ChangeLife {
        delta: Delta<Box<Amount>>,
    },
    Exchange {
        exchanged: Box<Exchanged>,
    },
    AddMana {
        amount: Box<Amount>,
        produced: Box<ProducedMana>,
        riders: Vec<ManaRider>,
    },
    Draw {
        quantity: Box<Quantity>,
    },
    Expose {
        verb: ExposeVerb,
        exposed: Box<Exposed>,
    },
    Search {
        scope: Box<Location>,
        for_: Box<NounPhrase>,
    },
    Shuffle {
        pile: Box<Location>,
    },
    FlipCoins {
        count: Box<FlipScope>,
        call: Option<FlipCall>,
        outcome: Option<CoinFace>,
    },
    RollDice {
        count: Box<Amount>,
        sides: DieSides,
        watch: Box<RollWatch>,
    },
    ApplyResultsTable {
        rows: Vec<RollRow>,
    },
    IgnoreOutcomes {
        which: Box<IgnoredOutcomes>,
    },
    ShiftResult {
        changes: Vec<Delta<Box<Amount>>>,
    },
    StoreResults {
        on: Box<NounPhrase>,
    },
    RerollStored {
        quantity: Box<Quantity>,
        whose: Box<NounPhrase>,
    },
    Establish {
        spec: Box<StaticSpec>,
        duration: Option<Box<Duration>>,
    },
    CreateObject {
        count: Box<Amount>,
        spec: Box<CreationSpec>,
        under: Option<Box<NounPhrase>>,
    },
    PutCounters {
        amount: Box<Amount>,
        kind: Box<CounterKindSource>,
        on: Box<NounPhrase>,
    },
    Distribute {
        verb: Box<DividedVerb>,
        amount: Box<Amount>,
        among: Box<NounPhrase>,
    },
    RemoveCounters {
        quantity: Option<Box<Quantity>>,
        kind: Option<Box<CounterKindSource>>,
        from_: Box<NounPhrase>,
    },
    MoveCounters {
        amount: Box<Amount>,
        kind: Option<Box<CounterKindSource>>,
        source: Box<NounPhrase>,
        destination: Box<NounPhrase>,
    },
    Pay {
        cost: Box<Cost>,
        times: PayTimes,
        outcome: PaymentOutcome,
        whose: Option<Box<NounPhrase>>,
        keyword: Option<KeywordLabel>,
    },
    Instruct {
        subj: Box<NounPhrase>,
        action: Box<Action>,
    },
    WithContinuation {
        policy: ContinuationPolicy,
        body: Box<Action>,
        if_did: Option<Box<Action>>,
        if_not: Option<Box<Action>>,
    },
    Limited {
        action: Box<Action>,
        limit: Box<UsageLimit>,
    },
    DoIf {
        condition: Box<Condition>,
        action: Box<Action>,
        otherwise: Option<Box<Action>>,
        postposed: bool,
    },
    DoForEach {
        group: Box<NounPhrase>,
        body: Box<Action>,
    },
    DoForEachKind {
        axis: KindAxis,
        domain: Option<Box<NounPhrase>>,
        sort: QualitySort,
        body: Box<Action>,
    },
    Repeat {
        repetition: Box<Repetition>,
    },
    Sequentially {
        steps: Vec<Action>,
    },
    Simultaneously {
        steps: Vec<Action>,
    },
    ChooseModes {
        quantity: Box<Quantity>,
        modes: Vec<Mode>,
    },
    Delay {
        event: Box<GameEvent>,
        alternatives: Vec<GameEvent>,
        duration: Option<Box<Duration>>,
        body: Box<Action>,
    },
    Replace {
        replaced: Box<Action>,
        replacement: Box<Replacement>,
    },
    HoldUntil {
        action: Box<Action>,
        event: Box<GameEvent>,
    },
    TriggerReflexively {
        body: Box<Action>,
        event: Option<Box<GameEvent>>,
        trigger: Box<Action>,
    },
    SkipUntap {
        subject: Box<NounPhrase>,
        steps: Box<Amount>,
    },
    SkipPart {
        part: TurnPart,
        count: Box<Amount>,
    },
    AddTurnPart {
        part: TurnPart,
        anchor: Option<TurnPart>,
        count: Box<Amount>,
        followed_by: Option<TurnPart>,
    },
    Procedure {
        kind: Procedure,
        subject: Box<NounPhrase>,
        payment: Box<PlayPayment>,
        resulting: Option<Box<Predicate>>,
        from_: Option<Box<Location>>,
        of_: Option<Box<NounPhrase>>,
    },
    Relate {
        subject: Box<NounPhrase>,
        label: RelationLabel,
        counterpart: Box<NounPhrase>,
    },
    Merge {
        subject: Box<NounPhrase>,
        onto: Box<NounPhrase>,
        position: MergePosition,
    },
}

/// Mirrors `Semantics/Abilities.lean:1435`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordParam {
    Cost { cost: Box<Cost> },
    Quality { predicate: Box<Predicate> },
    Subject { predicate: Box<Predicate> },
    Number { amount: Box<Amount> },
    DeckCondition { condition: Box<DeckCondition> },
    Ability { ability: Box<Ability> },
}

/// Mirrors `Semantics/Abilities.lean:1450`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbilityLost {
    Written { ability: Box<Ability> },
    Term { keyword: KeywordTerm },
}

/// Mirrors `Semantics/Abilities.lean:1454`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Ability {
    InCaller {
        body: Box<Ability>,
    },
    Keyword {
        keyword: KeywordLabel,
        params: Vec<KeywordParam>,
        body: Vec<Ability>,
    },
    Activated {
        cost: Box<Cost>,
        action: Box<Action>,
        timing: Option<Box<Timing>>,
        limit: Option<Box<UsageLimit>>,
        guard: Option<Box<Condition>>,
        activator: Option<Box<NounPhrase>>,
    },
    Triggered {
        event: Box<GameEvent>,
        alternatives: Vec<GameEvent>,
        while_: Option<Box<Timing>>,
        joins: Vec<JoinedHeader>,
        timing: Option<Box<Timing>>,
        limit: Option<Box<UsageLimit>>,
        intervening: Option<Box<Condition>>,
        action: Box<Action>,
    },
    Static {
        spec: Box<StaticSpec>,
    },
    Spell {
        timing: Option<Box<Timing>>,
        action: Box<Action>,
    },
    MayBeginOnBattlefield,
    ItalicHead {
        word: ItalicWord,
        ability: Box<Ability>,
    },
    ThatAbility {
        ref_: ChoiceRef,
    },
    Qualified {
        ability: Box<Ability>,
        timing: Option<Box<Timing>>,
        limit: Option<Box<UsageLimit>>,
        guard: Option<Box<Condition>>,
        from_: Option<Zone>,
    },
    ByCardClass {
        on_permanent: Box<Ability>,
        on_instant_or_sorcery: Box<Ability>,
    },
}

/// Mirrors `Semantics/Card.lean:13`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardFace {
    pub characteristics: Box<Characteristics>,
}

/// Mirrors `Semantics/Card.lean:17`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SharedLineHalf {
    pub name: Ident,
    pub cost: Option<ManaCost>,
    pub text: Vec<Ability>,
}

/// Mirrors `Semantics/Card.lean:23`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LevelRange {
    Between { from_: u64, to: u64 },
    Plus { from_: u64 },
}

/// Mirrors `Semantics/Card.lean:30`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LevelBand {
    pub range: LevelRange,
    pub text: Vec<Ability>,
    pub power: Option<Box<Amount>>,
    pub toughness: Option<Box<Amount>>,
}

/// Mirrors `Semantics/Card.lean:37`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PrototypeFrame {
    pub cost: Option<ManaCost>,
    pub power: Option<Box<Amount>>,
    pub toughness: Option<Box<Amount>>,
}

/// Mirrors `Semantics/Card.lean:42`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Card {
    SingleFaced {
        face: CardFace,
    },
    Transforming {
        front: CardFace,
        back: CardFace,
    },
    ModalDfc {
        front: CardFace,
        back: CardFace,
    },
    Split {
        left: CardFace,
        right: CardFace,
    },
    SharedLineSplit {
        shared: CardFace,
        left: SharedLineHalf,
        right: SharedLineHalf,
    },
    Adventurer {
        normal: CardFace,
        adventure: CardFace,
    },
    Flip {
        normal: CardFace,
        alternative: CardFace,
    },
    Leveler {
        inner: CardFace,
        bands: Vec<LevelBand>,
    },
    Prototype {
        inner: CardFace,
        alternative: PrototypeFrame,
    },
}

/// Mirrors `Semantics/Rules.lean:25`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SbaRule {
    pub scope: Box<Predicate>,
    pub when: Box<Condition>,
    pub then_: Box<Action>,
}

/// Mirrors `Semantics/Rules.lean:36`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PredefinedToken {
    pub name: Ident,
    pub token: Box<CharacteristicBundle>,
}

/// Mirrors `Semantics/Rules.lean:51`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Conferral {
    Ability {
        confer: Box<Ability>,
    },
    Property {
        spec: Box<StaticSpec>,
    },
    StateBased {
        when: Box<Condition>,
        then_: Box<Action>,
    },
    TurnBased {
        part: TurnPart,
        then_: Box<Action>,
    },
}

/// Mirrors `Semantics/Rules.lean:75`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConferralRule {
    pub scope: Box<Predicate>,
    pub confer: Conferral,
}

/// Mirrors `Semantics/Rules.lean:87`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RegistryDefinition {
    Counter {
        kind: CounterKind,
        confers: Vec<Conferral>,
    },
    Subtype {
        subtype: Subtype,
        rules: Vec<Conferral>,
    },
    Designation {
        label: DesignationLabel,
        confers: Vec<Conferral>,
    },
    CardType {
        card_type: CardType,
        confers: Vec<Conferral>,
    },
    Relation {
        label: RelationLabel,
    },
}

/// Mirrors `Semantics/Rules.lean:155`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RulesTables {
    pub sba: Vec<SbaRule>,
    pub conferral: Vec<ConferralRule>,
}

/// The cost, weight, and action of a mode, matching `Action.chooseModes`.
pub type Mode = (Option<Box<Cost>>, (Option<u64>, Box<Action>));

/// The polymorphic wildcard supplied by the authoring language.
pub trait AnyValue {
    fn any() -> Self;
}

#[must_use]
pub fn any<T: AnyValue>() -> T {
    T::any()
}

macro_rules! any_value {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(impl AnyValue for $ty {
            fn any() -> Self { Self::$variant }
        })*
    };
}
any_value! {
    Subject => Any, NounPhrase => Any, Ballot => AnyBallot, Cost => AnyCost,
    Amount => AnyAmount, CounterKindSource => AnyKind, RollWatch => AnyResult,
    Location => AnyLocation, Timing => AnyTiming, DieSides => AnyDie,
}
