//! Constructor signatures and defaults from the five authoritative Lean syntax
//! modules.
#![expect(
    clippy::too_many_lines,
    clippy::match_same_arms,
    reason = "This signature table preserves the ordering and distinct type entries of the Lean syntax declarations."
)]
use super::Constructor;
use super::Field;

pub(super) fn constructors(ty: &str) -> Option<Vec<Constructor>> {
    Some(match ty {
        "Procedure" => vec![
            Constructor::new("activate", "Activate", vec![]),
            Constructor::new("cast", "Cast", vec![]),
            Constructor::new("play", "Play", vec![]),
            Constructor::new("resolve", "Resolve", vec![]),
        ],
        "Stat" => vec![
            Constructor::new("power", "Power", vec![]),
            Constructor::new("toughness", "Toughness", vec![]),
            Constructor::new("manaValue", "ManaValue", vec![]),
            Constructor::new("loyalty", "Loyalty", vec![]),
            Constructor::new("defense", "Defense", vec![]),
            Constructor::new("basePower", "BasePower", vec![]),
            Constructor::new("baseToughness", "BaseToughness", vec![]),
        ],
        "CardType" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("name", "name", "String", None),
                Field::new("cardClass", "card_class", "Option CardClass", Some("none")),
                Field::new("stats", "stats", "List Stat", Some("[]")),
            ],
        )],
        "CardClass" => vec![
            Constructor::new("permanentCard", "PermanentCard", vec![]),
            Constructor::new("instantOrSorceryCard", "InstantOrSorceryCard", vec![]),
        ],
        "PlayerStat" => vec![
            Constructor::new("lifeTotal", "LifeTotal", vec![]),
            Constructor::new("startingLifeTotal", "StartingLifeTotal", vec![]),
            Constructor::new("speed", "Speed", vec![]),
        ],
        "Comparator" => vec![
            Constructor::new("atLeast", "AtLeast", vec![]),
            Constructor::new("atMost", "AtMost", vec![]),
            Constructor::new("greater", "Greater", vec![]),
            Constructor::new("less", "Less", vec![]),
            Constructor::new("eq", "Eq", vec![]),
        ],
        "QualitySort" => vec![
            Constructor::new("color", "Color", vec![]),
            Constructor::new(
                "subtype",
                "Subtype",
                vec![Field::new("host", "host", "CardType", None)],
            ),
            Constructor::new("cardName", "CardName", vec![]),
            Constructor::new("number", "Number", vec![]),
            Constructor::new("cardType", "CardType", vec![]),
            Constructor::new("counterKind", "CounterKind", vec![]),
            Constructor::new("ability", "Ability", vec![]),
        ],
        "Letter" => vec![
            Constructor::new("x", "X", vec![]),
            Constructor::new("y", "Y", vec![]),
        ],
        "Kind" => vec![
            Constructor::new("object", "Object", vec![]),
            Constructor::new("player", "Player", vec![]),
            Constructor::new(
                "quality",
                "Quality",
                vec![Field::new("sort", "sort", "QualitySort", None)],
            ),
            Constructor::new("outcome", "Outcome", vec![]),
            Constructor::new(
                "letter",
                "Letter",
                vec![Field::new("letter", "letter", "Letter", None)],
            ),
            Constructor::new("turnRef", "TurnRef", vec![]),
            Constructor::new("pile", "Pile", vec![]),
            Constructor::new(
                "join",
                "Join",
                vec![
                    Field::new("left", "left", "Kind", None),
                    Field::new("right", "right", "Kind", None),
                ],
            ),
        ],
        "AggregateOp" => vec![
            Constructor::new("sum", "Sum", vec![]),
            Constructor::new("min", "Min", vec![]),
            Constructor::new("max", "Max", vec![]),
        ],
        "SubtypeScope" => vec![
            Constructor::new("any", "Any", vec![]),
            Constructor::new("basicOnly", "BasicOnly", vec![]),
            Constructor::new("nonbasicOnly", "NonbasicOnly", vec![]),
        ],
        "KindAxis" => vec![
            Constructor::new("cardType", "CardType", vec![]),
            Constructor::new("permanentType", "PermanentType", vec![]),
            Constructor::new("color", "Color", vec![]),
            Constructor::new(
                "subtype",
                "Subtype",
                vec![
                    Field::new("host", "host", "CardType", None),
                    Field::new("scope", "scope", "SubtypeScope", None),
                ],
            ),
            Constructor::new(
                "value",
                "Value",
                vec![Field::new("stat", "stat", "Stat", None)],
            ),
            Constructor::new("counterKind", "CounterKind", vec![]),
            Constructor::new("colorPair", "ColorPair", vec![]),
        ],
        "OutcomeSort" => vec![
            Constructor::new("damageDealt", "DamageDealt", vec![]),
            Constructor::new("damageMarked", "DamageMarked", vec![]),
            Constructor::new("lifeGained", "LifeGained", vec![]),
            Constructor::new("lifeLost", "LifeLost", vec![]),
            Constructor::new("countersPut", "CountersPut", vec![]),
            Constructor::new("damagePrevented", "DamagePrevented", vec![]),
            Constructor::new("rollResult", "RollResult", vec![]),
            Constructor::new("coinFlipped", "CoinFlipped", vec![]),
            Constructor::new("diceRolled", "DiceRolled", vec![]),
            Constructor::new("namedNumber", "NamedNumber", vec![]),
            Constructor::new("repeatCount", "RepeatCount", vec![]),
            Constructor::new("countersRemoved", "CountersRemoved", vec![]),
            Constructor::new("manaAdded", "ManaAdded", vec![]),
            Constructor::new("manaProduced", "ManaProduced", vec![]),
            Constructor::new("ceilingShortfall", "CeilingShortfall", vec![]),
            Constructor::new("voteHeld", "VoteHeld", vec![]),
        ],
        "FlipCall" => vec![
            Constructor::new("wins", "Wins", vec![]),
            Constructor::new("loses", "Loses", vec![]),
        ],
        "CoinFace" => vec![
            Constructor::new("heads", "Heads", vec![]),
            Constructor::new("tails", "Tails", vec![]),
        ],
        "RollExtreme" => vec![
            Constructor::new("lowest", "Lowest", vec![]),
            Constructor::new("highest", "Highest", vec![]),
        ],
        "Plurality" => vec![
            Constructor::new("one", "One", vec![]),
            Constructor::new("many", "Many", vec![]),
        ],
        "Agreement" => vec![
            Constructor::new("share", "Share", vec![]),
            Constructor::new("differ", "Differ", vec![]),
        ],
        "PossessorAxis" => vec![
            Constructor::new("owner", "Owner", vec![]),
            Constructor::new("controller", "Controller", vec![]),
        ],
        "CombatRelation" => vec![
            Constructor::new("attackerOf", "AttackerOf", vec![]),
            Constructor::new("declaredAttacker", "DeclaredAttacker", vec![]),
            Constructor::new("blockerOf", "BlockerOf", vec![]),
            Constructor::new("blockedBy", "BlockedBy", vec![]),
            Constructor::new("unblocked", "Unblocked", vec![]),
            Constructor::new("attackedBy", "AttackedBy", vec![]),
            Constructor::new("couldBlock", "CouldBlock", vec![]),
            Constructor::new("couldBeBlockedBy", "CouldBeBlockedBy", vec![]),
        ],
        "CombatRole" => vec![
            Constructor::new("attacking", "Attacking", vec![]),
            Constructor::new("defending", "Defending", vec![]),
        ],
        "AttachMove" => vec![
            Constructor::new("attached", "Attached", vec![]),
            Constructor::new("unattached", "Unattached", vec![]),
        ],
        "ArithOp" => vec![
            Constructor::new("plus", "Plus", vec![]),
            Constructor::new("minus", "Minus", vec![]),
            Constructor::new("times", "Times", vec![]),
            Constructor::new("differenceBetween", "DifferenceBetween", vec![]),
        ],
        "Zone" => vec![
            Constructor::new("battlefield", "Battlefield", vec![]),
            Constructor::new("graveyard", "Graveyard", vec![]),
            Constructor::new("exile", "Exile", vec![]),
            Constructor::new("hand", "Hand", vec![]),
            Constructor::new("library", "Library", vec![]),
            Constructor::new("stack", "Stack", vec![]),
            Constructor::new("command", "Command", vec![]),
        ],
        "PileEnd" => vec![
            Constructor::new("top", "Top", vec![]),
            Constructor::new("bottom", "Bottom", vec![]),
        ],
        "Arrangement" => vec![
            Constructor::new("anyOrder", "AnyOrder", vec![]),
            Constructor::new("randomOrder", "RandomOrder", vec![]),
        ],
        "Ordinal" => vec![Constructor::new(
            "nth",
            "Nth",
            vec![Field::new("n", "n", "Nat", None)],
        )],
        "DesignationScope" => vec![
            Constructor::new(
                "heldBy",
                "HeldBy",
                vec![Field::new("holder", "holder", "Kind", None)],
            ),
            Constructor::new("heldByCard", "HeldByCard", vec![]),
            Constructor::new("heldByGame", "HeldByGame", vec![]),
        ],
        "DesignationLabel" => vec![Constructor::new(
            "named",
            "Named",
            vec![
                Field::new("name", "name", "String", None),
                Field::new("scope", "scope", "DesignationScope", None),
                Field::new("sole", "sole", "Bool", Some("false")),
            ],
        )],
        "RelationLabel" => vec![Constructor::new(
            "named",
            "Named",
            vec![Field::new("name", "name", "String", None)],
        )],
        "MergePosition" => vec![
            Constructor::new("top", "Top", vec![]),
            Constructor::new("bottom", "Bottom", vec![]),
        ],
        "KeywordLabel" => vec![Constructor::new(
            "named",
            "Named",
            vec![Field::new("name", "name", "String", None)],
        )],
        "AbilityWordLabel" => vec![Constructor::new(
            "named",
            "Named",
            vec![Field::new("name", "name", "String", None)],
        )],
        "CoreDeed" => vec![
            Constructor::new("target", "Target", vec![]),
            Constructor::new("draw", "Draw", vec![]),
            Constructor::new("trigger", "Trigger", vec![]),
            Constructor::new("gainControl", "GainControl", vec![]),
            Constructor::new("fullyUnlock", "FullyUnlock", vec![]),
        ],
        "StateBasedCause" => vec![Constructor::new(
            "nonpositiveLife",
            "NonpositiveLife",
            vec![],
        )],
        "DeedPermanent" => vec![
            Constructor::new("none", "None", vec![]),
            Constructor::new("performs", "Performs", vec![]),
            Constructor::new("isAbout", "IsAbout", vec![]),
        ],
        "Deed" => vec![
            Constructor::new(
                "core",
                "Core",
                vec![Field::new("deed", "deed", "CoreDeed", None)],
            ),
            Constructor::new(
                "action",
                "Action",
                vec![
                    Field::new("label", "label", "KeywordActionLabel", None),
                    Field::new("permanent", "permanent", "DeedPermanent", Some(".none")),
                ],
            ),
            Constructor::new(
                "ofAbility",
                "OfAbility",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
        ],
        "PileFace" => vec![
            Constructor::new("faceDown", "FaceDown", vec![]),
            Constructor::new("faceUp", "FaceUp", vec![]),
        ],
        "ChoiceSort" => vec![
            Constructor::new(
                "quality",
                "Quality",
                vec![Field::new("sort", "sort", "QualitySort", None)],
            ),
            Constructor::new("player", "Player", vec![]),
        ],
        "DieSides" => vec![
            Constructor::new(
                "sides",
                "Sides",
                vec![Field::new("sides", "sides", "Nat", None)],
            ),
            Constructor::new("thoseDice", "ThoseDice", vec![]),
            Constructor::new("anyDie", "AnyDie", vec![]),
        ],
        "ChoiceRef" => vec![
            Constructor::new("theChoice", "TheChoice", vec![]),
            Constructor::new("theLatestChoice", "TheLatestChoice", vec![]),
        ],
        "Disclosure" => vec![
            Constructor::new("openly", "Openly", vec![]),
            Constructor::new("secretly", "Secretly", vec![]),
        ],
        "HiddenSort" => vec![
            Constructor::new("numbers", "Numbers", vec![]),
            Constructor::new("choices", "Choices", vec![]),
        ],
        "ExposeVerb" => vec![
            Constructor::new("lookAt", "LookAt", vec![]),
            Constructor::new("reveal", "Reveal", vec![]),
        ],
        "Subtype" => vec![
            Constructor::new(
                "of",
                "Of",
                vec![
                    Field::new("host", "host", "CardType", None),
                    Field::new("label", "label", "String", None),
                ],
            ),
            Constructor::new(
                "spell",
                "Spell",
                vec![Field::new("label", "label", "String", None)],
            ),
        ],
        "NounWord" => vec![
            Constructor::new(
                "type",
                "Type",
                vec![Field::new("type", "type_", "CardType", None)],
            ),
            Constructor::new(
                "subtype",
                "Subtype",
                vec![Field::new("subtype", "subtype", "Subtype", None)],
            ),
            Constructor::new("card", "Card", vec![]),
            Constructor::new("spell", "Spell", vec![]),
            Constructor::new("player", "Player", vec![]),
            Constructor::new("permanent", "Permanent", vec![]),
            Constructor::new("token", "Token", vec![]),
            Constructor::new("emblem", "Emblem", vec![]),
            Constructor::new("copy", "Copy", vec![]),
            Constructor::new("join", "Join", vec![]),
            Constructor::new("stack", "Stack", vec![]),
            Constructor::new("ability", "Ability", vec![]),
            Constructor::new(
                "ofType",
                "OfType",
                vec![
                    Field::new("word", "word", "NounWord", None),
                    Field::new("type", "type_", "CardType", None),
                ],
            ),
            Constructor::new(
                "copied",
                "Copied",
                vec![Field::new("word", "word", "NounWord", None)],
            ),
            Constructor::new("pile", "Pile", vec![]),
            Constructor::new("source", "Source", vec![]),
        ],
        "SlotCarrier" => vec![
            Constructor::new("permanent", "Permanent", vec![]),
            Constructor::new("card", "Card", vec![]),
            Constructor::new("spell", "Spell", vec![]),
            Constructor::new("offStack", "OffStack", vec![]),
        ],
        "Reach" => vec![
            Constructor::new("bare", "Bare", vec![]),
            Constructor::new(
                "atSlot",
                "AtSlot",
                vec![Field::new("slot", "slot", "SlotCarrier", None)],
            ),
            Constructor::new(
                "word",
                "Word",
                vec![Field::new("word", "word", "NounWord", None)],
            ),
            Constructor::new(
                "unionHalf",
                "UnionHalf",
                vec![Field::new("word", "word", "NounWord", None)],
            ),
        ],
        "EntryCounterMark" => vec![
            Constructor::new("fresh", "Fresh", vec![]),
            Constructor::new("additional", "Additional", vec![]),
            Constructor::new("fewer", "Fewer", vec![]),
        ],
        "RoundMode" => vec![
            Constructor::new("up", "Up", vec![]),
            Constructor::new("down", "Down", vec![]),
        ],
        "TargetExtent" => vec![
            Constructor::new("someTarget", "SomeTarget", vec![]),
            Constructor::new("soleTarget", "SoleTarget", vec![]),
            Constructor::new("only", "Only", vec![]),
        ],
        "CopySort" => vec![
            Constructor::new("fromStack", "FromStack", vec![]),
            Constructor::new("fromCardZone", "FromCardZone", vec![]),
            Constructor::new(
                "inZone",
                "InZone",
                vec![Field::new("zone", "zone", "Zone", None)],
            ),
        ],
        "KeywordFamily" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("word", "word", "KeywordLabel", None),
                Field::new("sort", "sort", "Option QualitySort", None),
            ],
        )],
        "KeywordTerm" => vec![
            Constructor::new(
                "the",
                "The",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
            Constructor::new(
                "anyIn",
                "AnyIn",
                vec![Field::new("family", "family", "KeywordFamily", None)],
            ),
            Constructor::new(
                "theWith",
                "TheWith",
                vec![
                    Field::new("keyword", "keyword", "KeywordLabel", None),
                    Field::new("number", "number", "Nat", None),
                ],
            ),
        ],
        "LinkSource" => vec![
            Constructor::new(
                "byKeyword",
                "ByKeyword",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
            Constructor::new(
                "byNthKeyword",
                "ByNthKeyword",
                vec![
                    Field::new("ordinal", "ordinal", "Ordinal", None),
                    Field::new("keyword", "keyword", "KeywordLabel", None),
                ],
            ),
            Constructor::new("theAlternative", "TheAlternative", vec![]),
            Constructor::new("theAdditional", "TheAdditional", vec![]),
        ],
        "AbilityClass" => vec![
            Constructor::new("anyOnStack", "AnyOnStack", vec![]),
            Constructor::new("anyActivated", "AnyActivated", vec![]),
            Constructor::new("anyTriggered", "AnyTriggered", vec![]),
            Constructor::new("loyalty", "Loyalty", vec![]),
            Constructor::new("mana", "Mana", vec![]),
            Constructor::new(
                "keyword",
                "Keyword",
                vec![Field::new("k", "k", "KeywordLabel", None)],
            ),
        ],
        "ItalicWord" => vec![
            Constructor::new(
                "abilityWord",
                "AbilityWord",
                vec![Field::new("label", "label", "AbilityWordLabel", None)],
            ),
            Constructor::new(
                "flavorWord",
                "FlavorWord",
                vec![Field::new("label", "label", "FlavorWordLabel", None)],
            ),
        ],
        "Color" => vec![
            Constructor::new("white", "White", vec![]),
            Constructor::new("blue", "Blue", vec![]),
            Constructor::new("black", "Black", vec![]),
            Constructor::new("red", "Red", vec![]),
            Constructor::new("green", "Green", vec![]),
        ],
        "ColorOrColorless" => vec![
            Constructor::new("colorless", "Colorless", vec![]),
            Constructor::new(
                "of",
                "Of",
                vec![Field::new("color", "color", "Color", None)],
            ),
        ],
        "SimpleManaSymbol" => vec![
            Constructor::new(
                "generic",
                "Generic",
                vec![Field::new("amount", "amount", "Nat", None)],
            ),
            Constructor::new(
                "specific",
                "Specific",
                vec![Field::new("color", "color", "ColorOrColorless", None)],
            ),
        ],
        "ManaSymbol" => vec![
            Constructor::new(
                "simple",
                "Simple",
                vec![Field::new("symbol", "symbol", "SimpleManaSymbol", None)],
            ),
            Constructor::new(
                "hybrid",
                "Hybrid",
                vec![
                    Field::new("left", "left", "SimpleManaSymbol", None),
                    Field::new("right", "right", "Color", None),
                ],
            ),
            Constructor::new(
                "phyrexian",
                "Phyrexian",
                vec![
                    Field::new("color", "color", "Color", None),
                    Field::new("second", "second", "Option Color", None),
                ],
            ),
            Constructor::new("variable", "Variable", vec![]),
            Constructor::new("snow", "Snow", vec![]),
        ],
        "SubtypeSpace" => vec![
            Constructor::new("basicLand", "BasicLand", vec![]),
            Constructor::new("land", "Land", vec![]),
            Constructor::new("creature", "Creature", vec![]),
        ],
        "PayTimes" => vec![
            Constructor::new("once", "Once", vec![]),
            Constructor::new("anyNumberOfTimes", "AnyNumberOfTimes", vec![]),
            Constructor::new(
                "upTo",
                "UpTo",
                vec![Field::new("times", "times", "Nat", None)],
            ),
        ],
        "CostNamed" => vec![
            Constructor::new(
                "containing",
                "Containing",
                vec![Field::new("symbol", "symbol", "ManaSymbol", None)],
            ),
            Constructor::new(
                "ofKeyword",
                "OfKeyword",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
        ],
        "ColorFreedom" => vec![
            Constructor::new("sameColor", "SameColor", vec![]),
            Constructor::new("eachColor", "EachColor", vec![]),
            Constructor::new("distinctColors", "DistinctColors", vec![]),
        ],
        "ManaMatch" => vec![
            Constructor::new("anyColor", "AnyColor", vec![]),
            Constructor::new("anyType", "AnyType", vec![]),
            Constructor::new(
                "of",
                "Of",
                vec![Field::new("color", "color", "ColorOrColorless", None)],
            ),
        ],
        "Delta" => vec![
            Constructor::new("up", "Up", vec![Field::new("amount", "amount", "α", None)]),
            Constructor::new(
                "down",
                "Down",
                vec![Field::new("amount", "amount", "α", None)],
            ),
            Constructor::new(
                "set",
                "Set",
                vec![Field::new("amount", "amount", "α", None)],
            ),
        ],
        "Parity" => vec![
            Constructor::new("even", "Even", vec![]),
            Constructor::new("odd", "Odd", vec![]),
        ],
        "Supertype" => vec![
            Constructor::new("legendary", "Legendary", vec![]),
            Constructor::new("basic", "Basic", vec![]),
            Constructor::new("snow", "Snow", vec![]),
            Constructor::new("world", "World", vec![]),
        ],
        "LockState" => vec![
            Constructor::new("locked", "Locked", vec![]),
            Constructor::new("unlocked", "Unlocked", vec![]),
        ],
        "AttachmentSide" => vec![
            Constructor::new("host", "Host", vec![]),
            Constructor::new("attachment", "Attachment", vec![]),
        ],
        "CardFaceSide" => vec![
            Constructor::new("front", "Front", vec![]),
            Constructor::new("back", "Back", vec![]),
        ],
        "AttachWord" => vec![
            Constructor::new("enchanted", "Enchanted", vec![]),
            Constructor::new("equipped", "Equipped", vec![]),
            Constructor::new("fortified", "Fortified", vec![]),
        ],
        "OutcomeVerb" => vec![
            Constructor::new("winGame", "WinGame", vec![]),
            Constructor::new("loseGame", "LoseGame", vec![]),
            Constructor::new("drawGame", "DrawGame", vec![]),
        ],
        "DefinedSlots" => vec![
            Constructor::new("powerAlone", "PowerAlone", vec![]),
            Constructor::new("toughnessAlone", "ToughnessAlone", vec![]),
            Constructor::new("bothEach", "BothEach", vec![]),
        ],
        "CounterKind" => vec![Constructor::new(
            "named",
            "Named",
            vec![
                Field::new("name", "name", "String", None),
                Field::new("holder", "holder", "Kind", Some(".object")),
            ],
        )],
        "ProjAxis" => vec![
            Constructor::new(
                "stat",
                "Stat",
                vec![Field::new("stat", "stat", "Stat", None)],
            ),
            Constructor::new(
                "playerStat",
                "PlayerStat",
                vec![Field::new("stat", "stat", "PlayerStat", None)],
            ),
            Constructor::new(
                "counter",
                "Counter",
                vec![Field::new("kind", "kind", "CounterKind", None)],
            ),
            Constructor::new(
                "anyCounter",
                "AnyCounter",
                vec![Field::new("kind", "kind", "Kind", None)],
            ),
            Constructor::new("boundCounter", "BoundCounter", vec![]),
            Constructor::new("markedDamage", "MarkedDamage", vec![]),
            Constructor::new("chapter", "Chapter", vec![]),
            Constructor::new(
                "keywordNumber",
                "KeywordNumber",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
        ],
        "Status" => vec![
            Constructor::new("tapped", "Tapped", vec![]),
            Constructor::new("untapped", "Untapped", vec![]),
            Constructor::new("flipped", "Flipped", vec![]),
            Constructor::new("unflipped", "Unflipped", vec![]),
            Constructor::new("faceUp", "FaceUp", vec![]),
            Constructor::new("faceDown", "FaceDown", vec![]),
            Constructor::new("phasedIn", "PhasedIn", vec![]),
            Constructor::new("phasedOut", "PhasedOut", vec![]),
        ],
        "ColorSpec" => vec![
            Constructor::new(
                "some",
                "Some",
                vec![Field::new("colors", "colors", "List Color", None)],
            ),
            Constructor::new("every", "Every", vec![]),
        ],
        "TurnPart" => vec![
            Constructor::new("turn", "Turn", vec![]),
            Constructor::new("beginningPhase", "BeginningPhase", vec![]),
            Constructor::new("untapStep", "UntapStep", vec![]),
            Constructor::new("upkeep", "Upkeep", vec![]),
            Constructor::new("drawStep", "DrawStep", vec![]),
            Constructor::new("mainPhase", "MainPhase", vec![]),
            Constructor::new("firstMain", "FirstMain", vec![]),
            Constructor::new("combat", "Combat", vec![]),
            Constructor::new("beginningOfCombat", "BeginningOfCombat", vec![]),
            Constructor::new("declareAttackers", "DeclareAttackers", vec![]),
            Constructor::new("declareBlockers", "DeclareBlockers", vec![]),
            Constructor::new("firstStrikeCombatDamage", "FirstStrikeCombatDamage", vec![]),
            Constructor::new("combatDamage", "CombatDamage", vec![]),
            Constructor::new("endOfCombat", "EndOfCombat", vec![]),
            Constructor::new("postcombatMain", "PostcombatMain", vec![]),
            Constructor::new("endingPhase", "EndingPhase", vec![]),
            Constructor::new("endStep", "EndStep", vec![]),
            Constructor::new("cleanup", "Cleanup", vec![]),
        ],
        "Period" => vec![
            Constructor::new("turn", "Turn", vec![]),
            Constructor::new("yourTurn", "YourTurn", vec![]),
            Constructor::new("game", "Game", vec![]),
            Constructor::new(
                "part",
                "Part",
                vec![Field::new("part", "part", "TurnPart", None)],
            ),
            Constructor::new("thisObject", "ThisObject", vec![]),
        ],
        "EventSide" => vec![
            Constructor::new("before", "Before", vec![]),
            Constructor::new("after", "After", vec![]),
        ],
        "PaymentOutcome" => vec![
            Constructor::new("paid", "Paid", vec![]),
            Constructor::new("unpaid", "Unpaid", vec![]),
        ],
        "TallyMode" => vec![
            Constructor::new("events", "Events", vec![]),
            Constructor::new("quantity", "Quantity", vec![]),
        ],
        "ReplUse" => vec![
            Constructor::new("repeatedly", "Repeatedly", vec![]),
            Constructor::new("nextTimeOnly", "NextTimeOnly", vec![]),
        ],
        "DamageKind" => vec![
            Constructor::new("combat", "Combat", vec![]),
            Constructor::new("noncombat", "Noncombat", vec![]),
        ],
        "CondMarking" => vec![
            Constructor::new("asLongAs", "AsLongAs", vec![]),
            Constructor::new("ifSo", "IfSo", vec![]),
        ],
        "ChoiceMode" => vec![
            Constructor::new("unmarked", "Unmarked", vec![]),
            Constructor::new("theirChoice", "TheirChoice", vec![]),
            Constructor::new("atRandom", "AtRandom", vec![]),
            Constructor::new("yourChoice", "YourChoice", vec![]),
        ],
        "ObservationPoint" => vec![
            Constructor::new("arrival", "Arrival", vec![]),
            Constructor::new("departure", "Departure", vec![]),
        ],
        "ManaHeld" => vec![
            Constructor::new("thisMana", "ThisMana", vec![]),
            Constructor::new(
                "unspent",
                "Unspent",
                vec![Field::new("type", "type_", "Option ColorOrColorless", None)],
            ),
        ],
        "TextWord" => vec![
            Constructor::new(
                "color",
                "Color",
                vec![Field::new("color", "color", "Color", None)],
            ),
            Constructor::new(
                "subtype",
                "Subtype",
                vec![Field::new("subtype", "subtype", "Subtype", None)],
            ),
        ],
        "TextWordSort" => vec![
            Constructor::new("color", "Color", vec![]),
            Constructor::new("landType", "LandType", vec![]),
            Constructor::new("creatureType", "CreatureType", vec![]),
        ],
        "QualityOp" => vec![
            Constructor::new("adds", "Adds", vec![]),
            Constructor::new("sets", "Sets", vec![]),
            Constructor::new("loses", "Loses", vec![]),
        ],
        "TypeLineChanges" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new(
                    "supertypes",
                    "supertypes",
                    "Option (List Supertype)",
                    Some("none"),
                ),
                Field::new("types", "types", "Option (List CardType)", Some("none")),
                Field::new(
                    "subtypes",
                    "subtypes",
                    "Option (List Subtype)",
                    Some("none"),
                ),
            ],
        )],
        "CharacteristicStat" => vec![
            Constructor::new("power", "Power", vec![]),
            Constructor::new("toughness", "Toughness", vec![]),
            Constructor::new("loyalty", "Loyalty", vec![]),
            Constructor::new("defense", "Defense", vec![]),
        ],
        "SpentMode" => vec![
            Constructor::new("affectsIt", "AffectsIt", vec![]),
            Constructor::new("triggersThen", "TriggersThen", vec![]),
        ],
        "ContinuationPolicy" => vec![
            Constructor::new("optional", "Optional", vec![]),
            Constructor::new("required", "Required", vec![]),
        ],
        "NameSource" => vec![
            Constructor::new(
                "printed",
                "Printed",
                vec![Field::new("name", "name", "String", None)],
            ),
            Constructor::new("chosen", "Chosen", vec![]),
        ],
        "Coordinator" => vec![
            Constructor::new("and", "And", vec![]),
            Constructor::new("or", "Or", vec![]),
            Constructor::new("andOr", "AndOr", vec![]),
        ],
        "Location" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Location", None)],
            ),
            Constructor::new(
                "zone",
                "Zone",
                vec![
                    Field::new("zone", "zone", "Zone", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "fromEnd",
                "FromEnd",
                vec![
                    Field::new("end_", "end_", "PileEnd", None),
                    Field::new("ordinal", "ordinal", "Ordinal", Some(".nth 1")),
                    Field::new("loc", "loc", "Location", None),
                ],
            ),
            Constructor::new(
                "exiledWith",
                "ExiledWith",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new(
                "pile",
                "Pile",
                vec![Field::new("pile", "pile", "NounPhrase", None)],
            ),
            Constructor::new("anyLocation", "AnyLocation", vec![]),
            Constructor::new(
                "not",
                "Not",
                vec![Field::new("loc", "loc", "Location", None)],
            ),
            Constructor::new(
                "coord",
                "Coord",
                vec![
                    Field::new("coordinator", "coordinator", "Coordinator", None),
                    Field::new("locs", "locs", "List Location", None),
                ],
            ),
        ],
        "ChoiceDomain" => vec![
            Constructor::new(
                "nameOfCard",
                "NameOfCard",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "colorOtherThan",
                "ColorOtherThan",
                vec![Field::new("color", "color", "Color", None)],
            ),
            Constructor::new(
                "typeOtherThan",
                "TypeOtherThan",
                vec![Field::new("subtype", "subtype", "Subtype", None)],
            ),
            Constructor::new("basicTypesOnly", "BasicTypesOnly", vec![]),
            Constructor::new("nonbasicTypesOnly", "NonbasicTypesOnly", vec![]),
            Constructor::new(
                "number",
                "Number",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new(
                "players",
                "Players",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "abilitiesAmong",
                "AbilitiesAmong",
                vec![Field::new("keywords", "keywords", "List KeywordTerm", None)],
            ),
        ],
        "Subject" => vec![
            Constructor::new(
                "actor",
                "Actor",
                vec![Field::new("np", "np", "NounPhrase", None)],
            ),
            Constructor::new("any", "Any", vec![]),
            Constructor::new("by", "By", vec![Field::new("np", "np", "NounPhrase", None)]),
        ],
        "LookbackSpan" => vec![
            Constructor::new(
                "duration",
                "Duration",
                vec![
                    Field::new("d", "d", "Duration", None),
                    Field::new("timing", "timing", "Option Timing", None),
                ],
            ),
            Constructor::new("thisWay", "ThisWay", vec![]),
            Constructor::new("triggering", "Triggering", vec![]),
            Constructor::new("thisObject", "ThisObject", vec![]),
        ],
        "DidClause" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("subj", "subj", "Subject", None),
                Field::new("action", "action", "Action", None),
                Field::new("within", "within", "Option LookbackSpan", None),
                Field::new(
                    "ordinal",
                    "ordinal",
                    "Option (Ordinal × Option Period)",
                    Some("none"),
                ),
            ],
        )],
        "TurnRef" => vec![
            Constructor::new("current", "Current", vec![]),
            Constructor::new(
                "previous",
                "Previous",
                vec![Field::new("player", "player", "Option NounPhrase", None)],
            ),
            Constructor::new(
                "next",
                "Next",
                vec![Field::new("player", "player", "Option NounPhrase", None)],
            ),
            Constructor::new(
                "each",
                "Each",
                vec![Field::new("player", "player", "Option NounPhrase", None)],
            ),
            Constructor::new("later", "Later", vec![]),
            Constructor::new("that", "That", vec![]),
            Constructor::new(
                "mostRecent",
                "MostRecent",
                vec![Field::new("player", "player", "NounPhrase", None)],
            ),
        ],
        "Timing" => vec![
            Constructor::new(
                "inPart",
                "InPart",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                    Field::new("which", "which", "TurnRef", None),
                ],
            ),
            Constructor::new(
                "relativeToEvent",
                "RelativeToEvent",
                vec![
                    Field::new("side", "side", "EventSide", None),
                    Field::new("event", "event", "GameEvent", None),
                ],
            ),
            Constructor::new(
                "whileDoing",
                "WhileDoing",
                vec![Field::new("event", "event", "GameEvent", None)],
            ),
            Constructor::new(
                "onlyIf",
                "OnlyIf",
                vec![Field::new("condition", "condition", "Condition", None)],
            ),
            Constructor::new(
                "and",
                "And",
                vec![Field::new("conjuncts", "conjuncts", "List Timing", None)],
            ),
            Constructor::new(
                "or",
                "Or",
                vec![Field::new("disjuncts", "disjuncts", "List Timing", None)],
            ),
            Constructor::new(
                "not",
                "Not",
                vec![Field::new("timing", "timing", "Timing", None)],
            ),
            Constructor::new("anyTiming", "AnyTiming", vec![]),
        ],
        "Duration" => vec![
            Constructor::new(
                "between",
                "Between",
                vec![
                    Field::new("start", "start", "Option GameEvent", None),
                    Field::new("end_", "end_", "Option GameEvent", None),
                ],
            ),
            Constructor::new(
                "forAsLongAs",
                "ForAsLongAs",
                vec![Field::new("condition", "condition", "Condition", None)],
            ),
            Constructor::new("modifiesPrecedingAction", "ModifiesPrecedingAction", vec![]),
        ],
        "LinkedFacet" => vec![
            Constructor::new("colorsSpent", "ColorsSpent", vec![]),
            Constructor::new("manaValueSpent", "ManaValueSpent", vec![]),
            Constructor::new("phyrexianPaidWithLife", "PhyrexianPaidWithLife", vec![]),
            Constructor::new(
                "timesPaid",
                "TimesPaid",
                vec![Field::new("which", "which", "LinkSource", None)],
            ),
            Constructor::new(
                "readback",
                "Readback",
                vec![
                    Field::new("which", "which", "LinkSource", None),
                    Field::new("within", "within", "Option Duration", None),
                ],
            ),
            Constructor::new(
                "taken",
                "Taken",
                vec![Field::new("which", "which", "LinkSource", None)],
            ),
        ],
        "Predicate" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Predicate", None)],
            ),
            Constructor::new(
                "hasType",
                "HasType",
                vec![Field::new("type", "type_", "CardType", None)],
            ),
            Constructor::new(
                "hasSubtype",
                "HasSubtype",
                vec![Field::new("subtype", "subtype", "Subtype", None)],
            ),
            Constructor::new(
                "hasSupertype",
                "HasSupertype",
                vec![Field::new("supertype", "supertype", "Supertype", None)],
            ),
            Constructor::new("isPlayer", "IsPlayer", vec![]),
            Constructor::new(
                "isOpponentOf",
                "IsOpponentOf",
                vec![Field::new("who", "who", "NounPhrase", None)],
            ),
            Constructor::new(
                "isTeammateOf",
                "IsTeammateOf",
                vec![Field::new("who", "who", "NounPhrase", None)],
            ),
            Constructor::new(
                "chosen",
                "Chosen",
                vec![Field::new("ref", "ref_", "ChoiceRef", None)],
            ),
            Constructor::new(
                "qualityNoun",
                "QualityNoun",
                vec![
                    Field::new("sort", "sort", "QualitySort", None),
                    Field::new("domain", "domain", "Option ChoiceDomain", None),
                ],
            ),
            Constructor::new(
                "counterKindOn",
                "CounterKindOn",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "ofChosen",
                "OfChosen",
                vec![
                    Field::new("ref", "ref_", "ChoiceRef", None),
                    Field::new("sort", "sort", "QualitySort", None),
                ],
            ),
            Constructor::new(
                "ofYourChoice",
                "OfYourChoice",
                vec![
                    Field::new("sort", "sort", "QualitySort", None),
                    Field::new("domain", "domain", "Option ChoiceDomain", None),
                ],
            ),
            Constructor::new(
                "eachKind",
                "EachKind",
                vec![
                    Field::new("axis", "axis", "KindAxis", None),
                    Field::new("domain", "domain", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "hasKeyword",
                "HasKeyword",
                vec![Field::new("keyword", "keyword", "KeywordTerm", None)],
            ),
            Constructor::new(
                "hasPossessor",
                "HasPossessor",
                vec![
                    Field::new("axis", "axis", "PossessorAxis", None),
                    Field::new("possessor", "possessor", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "heldBeforeCast",
                "HeldBeforeCast",
                vec![Field::new(
                    "designation",
                    "designation",
                    "DesignationLabel",
                    None,
                )],
            ),
            Constructor::new(
                "inCombat",
                "InCombat",
                vec![
                    Field::new("relation", "relation", "CombatRelation", None),
                    Field::new("counterpart", "counterpart", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "did",
                "Did",
                vec![Field::new("clause", "clause", "DidClause", None)],
            ),
            Constructor::new(
                "colorIs",
                "ColorIs",
                vec![Field::new("color", "color", "Color", None)],
            ),
            Constructor::new(
                "colorCount",
                "ColorCount",
                vec![
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("count", "count", "Nat", None),
                ],
            ),
            Constructor::new(
                "named",
                "Named",
                vec![Field::new("source", "source", "NameSource", None)],
            ),
            Constructor::new(
                "shares",
                "Shares",
                vec![
                    Field::new("axis", "axis", "QualitySort", None),
                    Field::new("agreement", "agreement", "Agreement", None),
                    Field::new("with_", "with_", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "hasDesignation",
                "HasDesignation",
                vec![
                    Field::new("designation", "designation", "DesignationLabel", None),
                    Field::new("holder", "holder", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "related",
                "Related",
                vec![
                    Field::new("label", "label", "RelationLabel", None),
                    Field::new("counterpart", "counterpart", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "attachment",
                "Attachment",
                vec![
                    Field::new("side", "side", "AttachmentSide", None),
                    Field::new("word", "word", "Option AttachWord", None),
                    Field::new("counterpart", "counterpart", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "legalAttachment",
                "LegalAttachment",
                vec![
                    Field::new("side", "side", "AttachmentSide", None),
                    Field::new("counterpart", "counterpart", "NounPhrase", None),
                ],
            ),
            Constructor::new("isCard", "IsCard", vec![]),
            Constructor::new("isToken", "IsToken", vec![]),
            Constructor::new("isEmblem", "IsEmblem", vec![]),
            Constructor::new("isCopyOfACard", "IsCopyOfACard", vec![]),
            Constructor::new(
                "hasTypeIgnoringTypeChanges",
                "HasTypeIgnoringTypeChanges",
                vec![Field::new("type", "type_", "CardType", None)],
            ),
            Constructor::new(
                "currentFace",
                "CurrentFace",
                vec![Field::new("side", "side", "CardFaceSide", None)],
            ),
            Constructor::new(
                "hasStatus",
                "HasStatus",
                vec![Field::new("status", "status", "Status", None)],
            ),
            Constructor::new(
                "hasCounters",
                "HasCounters",
                vec![Field::new("kind", "kind", "Option CounterKind", None)],
            ),
            Constructor::new(
                "compare",
                "Compare",
                vec![
                    Field::new("axes", "axes", "List ProjAxis", None),
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("bound", "bound", "Amount", None),
                ],
            ),
            Constructor::new(
                "superlative",
                "Superlative",
                vec![
                    Field::new("op", "op", "AggregateOp", None),
                    Field::new("axis", "axis", "ProjAxis", None),
                    Field::new("domain", "domain", "Predicate", None),
                ],
            ),
            Constructor::new("withMostVotes", "WithMostVotes", vec![]),
            Constructor::new(
                "choseExtreme",
                "ChoseExtreme",
                vec![Field::new("op", "op", "AggregateOp", None)],
            ),
            Constructor::new(
                "compareOver",
                "CompareOver",
                vec![
                    Field::new("domain", "domain", "Predicate", None),
                    Field::new("measure", "measure", "Amount", None),
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("bound", "bound", "Amount", None),
                ],
            ),
            Constructor::new(
                "in_",
                "In",
                vec![Field::new("loc", "loc", "Location", None)],
            ),
            Constructor::new(
                "coord",
                "Coord",
                vec![
                    Field::new("coordinator", "coordinator", "Coordinator", None),
                    Field::new("items", "items", "List Predicate", None),
                ],
            ),
            Constructor::new(
                "not",
                "Not",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new("other", "Other", vec![]),
            Constructor::new("notChosen", "NotChosen", vec![]),
            Constructor::new(
                "otherThan",
                "OtherThan",
                vec![Field::new("anchor", "anchor", "NounPhrase", None)],
            ),
            Constructor::new(
                "coinCameUp",
                "CoinCameUp",
                vec![Field::new("face", "face", "CoinFace", None)],
            ),
            Constructor::new("isSource", "IsSource", vec![]),
            Constructor::new(
                "manaCostHas",
                "ManaCostHas",
                vec![Field::new("symbol", "symbol", "ManaSymbol", None)],
            ),
            Constructor::new(
                "abilityHead",
                "AbilityHead",
                vec![Field::new("class_", "class_", "AbilityClass", None)],
            ),
            Constructor::new(
                "abilityOf",
                "AbilityOf",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new(
                "activationCostHas",
                "ActivationCostHas",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new("triggeredNotLeftStack", "TriggeredNotLeftStack", vec![]),
            Constructor::new(
                "targets",
                "Targets",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("extent", "extent", "TargetExtent", None),
                ],
            ),
        ],
        "DetPhrase" => vec![
            Constructor::new(
                "target",
                "Target",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new(
                "a",
                "A",
                vec![Field::new("mode", "mode", "ChoiceMode", None)],
            ),
            Constructor::new("each", "Each", vec![]),
            Constructor::new("all", "All", vec![]),
            Constructor::new(
                "the",
                "The",
                vec![Field::new("plur", "plur", "Plurality", Some(".one"))],
            ),
            Constructor::new(
                "count",
                "Count",
                vec![
                    Field::new("quantity", "quantity", "Quantity", None),
                    Field::new("mode", "mode", "Option ChoiceMode", None),
                ],
            ),
        ],
        "ColorTerm" => vec![
            Constructor::new(
                "lit",
                "Lit",
                vec![Field::new("color", "color", "Color", None)],
            ),
            Constructor::new(
                "chosen",
                "Chosen",
                vec![Field::new("ref", "ref_", "ChoiceRef", None)],
            ),
            Constructor::new(
                "colorOf",
                "ColorOf",
                vec![Field::new("object", "object", "NounPhrase", None)],
            ),
        ],
        "ManaTypeTerm" => vec![
            Constructor::new("colorless", "Colorless", vec![]),
            Constructor::new(
                "ofColor",
                "OfColor",
                vec![Field::new("color", "color", "ColorTerm", None)],
            ),
        ],
        "NounPhrase" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "NounPhrase", None)],
            ),
            Constructor::new("which", "Which", vec![]),
            Constructor::new("this", "This", vec![]),
            Constructor::new(
                "headed",
                "Headed",
                vec![
                    Field::new("head", "head", "NounWord", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "becomes",
                "Becomes",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "theGrantor",
                "TheGrantor",
                vec![Field::new("head", "head", "NounWord", None)],
            ),
            Constructor::new("you", "You", vec![]),
            Constructor::new("actor", "Actor", vec![]),
            Constructor::new(
                "combatPlayer",
                "CombatPlayer",
                vec![Field::new("role", "role", "CombatRole", None)],
            ),
            Constructor::new(
                "described",
                "Described",
                vec![
                    Field::new("determiner", "determiner", "DetPhrase", None),
                    Field::new("predicate", "predicate", "Predicate", None),
                ],
            ),
            Constructor::new(
                "eachOf",
                "EachOf",
                vec![Field::new("group", "group", "NounPhrase", None)],
            ),
            Constructor::new(
                "coord",
                "Coord",
                vec![
                    Field::new("coordinator", "coordinator", "Coordinator", None),
                    Field::new("phrases", "phrases", "List NounPhrase", None),
                ],
            ),
            Constructor::new(
                "slice",
                "Slice",
                vec![
                    Field::new("end_", "end_", "PileEnd", None),
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("pile", "pile", "Location", None),
                ],
            ),
            Constructor::new(
                "run",
                "Run",
                vec![
                    Field::new("end_", "end_", "PileEnd", None),
                    Field::new("stop", "stop", "Predicate", None),
                    Field::new("pile", "pile", "Location", None),
                ],
            ),
            Constructor::new(
                "someOf",
                "SomeOf",
                vec![
                    Field::new("count", "count", "SliceCount", None),
                    Field::new("description", "description", "Option Predicate", None),
                    Field::new("group", "group", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "agree",
                "Agree",
                vec![
                    Field::new("axis", "axis", "QualitySort", None),
                    Field::new("agreement", "agreement", "Agreement", None),
                    Field::new("group", "group", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "theRest",
                "TheRest",
                vec![Field::new("plurality", "plurality", "Plurality", None)],
            ),
            Constructor::new(
                "pileOf",
                "PileOf",
                vec![
                    Field::new("count", "count", "SliceCount", None),
                    Field::new("by_", "by_", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "pro",
                "Pro",
                vec![
                    Field::new("reach", "reach", "Reach", None),
                    Field::new("plurality", "plurality", "Plurality", None),
                ],
            ),
            Constructor::new(
                "refl",
                "Refl",
                vec![Field::new("plurality", "plurality", "Plurality", None)],
            ),
            Constructor::new(
                "firstMention",
                "FirstMention",
                vec![
                    Field::new("id", "id", "BindingId", None),
                    Field::new("phrase", "phrase", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "laterMention",
                "LaterMention",
                vec![
                    Field::new("id", "id", "BindingId", None),
                    Field::new("phrase", "phrase", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "attachHost",
                "AttachHost",
                vec![
                    Field::new("word", "word", "AttachWord", None),
                    Field::new("head", "head", "NounWord", None),
                ],
            ),
            Constructor::new(
                "possessorOf",
                "PossessorOf",
                vec![
                    Field::new("axis", "axis", "PossessorAxis", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "designated",
                "Designated",
                vec![
                    Field::new("designation", "designation", "DesignationLabel", None),
                    Field::new("whose", "whose", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "counterpart",
                "Counterpart",
                vec![
                    Field::new("label", "label", "RelationLabel", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "oneEachOf",
                "OneEachOf",
                vec![
                    Field::new("roles", "roles", "List Predicate", None),
                    Field::new("pool", "pool", "NounPhrase", None),
                ],
            ),
            Constructor::new("any", "Any", vec![]),
        ],
        "Amount" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Amount", None)],
            ),
            Constructor::new(
                "lit",
                "Lit",
                vec![Field::new("value", "value", "Int", None)],
            ),
            Constructor::new(
                "statOf",
                "StatOf",
                vec![
                    Field::new("axis", "axis", "ProjAxis", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "countOf",
                "CountOf",
                vec![Field::new("group", "group", "NounPhrase", None)],
            ),
            Constructor::new(
                "aggregate",
                "Aggregate",
                vec![
                    Field::new("op", "op", "AggregateOp", None),
                    Field::new("axis", "axis", "ProjAxis", None),
                    Field::new("group", "group", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "paid",
                "Paid",
                vec![
                    Field::new("facet", "facet", "LinkedFacet", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "tally",
                "Tally",
                vec![
                    Field::new("clause", "clause", "DidClause", None),
                    Field::new("mode", "mode", "TallyMode", None),
                ],
            ),
            Constructor::new("thatMuch", "ThatMuch", vec![]),
            Constructor::new(
                "chosenNumber",
                "ChosenNumber",
                vec![Field::new("ref", "ref_", "ChoiceRef", None)],
            ),
            Constructor::new(
                "votesFor",
                "VotesFor",
                vec![Field::new("label", "label", "VoteLabel", None)],
            ),
            Constructor::new(
                "theOutcome",
                "TheOutcome",
                vec![Field::new("sort", "sort", "OutcomeSort", None)],
            ),
            Constructor::new(
                "coinsShowing",
                "CoinsShowing",
                vec![Field::new("face", "face", "CoinFace", None)],
            ),
            Constructor::new(
                "greatestStoredMatch",
                "GreatestStoredMatch",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new("groupSize", "GroupSize", vec![]),
            Constructor::new("theDifference", "TheDifference", vec![]),
            Constructor::new(
                "letter",
                "Letter",
                vec![Field::new("letter", "letter", "Letter", None)],
            ),
            Constructor::new(
                "arith",
                "Arith",
                vec![
                    Field::new("op", "op", "ArithOp", None),
                    Field::new("left", "left", "Amount", None),
                    Field::new("right", "right", "Amount", None),
                ],
            ),
            Constructor::new(
                "devotion",
                "Devotion",
                vec![
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("color", "color", "ColorTerm", None),
                    Field::new("second", "second", "Option ColorTerm", None),
                ],
            ),
            Constructor::new(
                "half",
                "Half",
                vec![
                    Field::new("rounding", "rounding", "RoundMode", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
            Constructor::new(
                "aggregateOver",
                "AggregateOver",
                vec![
                    Field::new("op", "op", "AggregateOp", None),
                    Field::new("domain", "domain", "Predicate", None),
                    Field::new("body", "body", "Amount", None),
                ],
            ),
            Constructor::new(
                "aggregateOf",
                "AggregateOf",
                vec![
                    Field::new("op", "op", "AggregateOp", None),
                    Field::new("amounts", "amounts", "List Amount", None),
                ],
            ),
            Constructor::new(
                "distinctCount",
                "DistinctCount",
                vec![
                    Field::new("axis", "axis", "KindAxis", None),
                    Field::new("domain", "domain", "NounPhrase", None),
                ],
            ),
            Constructor::new("anyAmount", "AnyAmount", vec![]),
            Constructor::new(
                "theNth",
                "TheNth",
                vec![Field::new("ordinal", "ordinal", "Ordinal", None)],
            ),
        ],
        "Quantity" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Quantity", None)],
            ),
            Constructor::new(
                "range",
                "Range",
                vec![
                    Field::new("low", "low", "Option Nat", None),
                    Field::new("high", "high", "Option Nat", None),
                ],
            ),
            Constructor::new(
                "upToOf",
                "UpToOf",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "exactlyOf",
                "ExactlyOf",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new("worth", "Worth", vec![Field::new("n", "n", "Nat", None)]),
            Constructor::new(
                "moreThanOf",
                "MoreThanOf",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "additional",
                "Additional",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new("theLast", "TheLast", vec![]),
        ],
        "SliceCount" => vec![
            Constructor::new(
                "counted",
                "Counted",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new("whole", "Whole", vec![]),
            Constructor::new(
                "totalling",
                "Totalling",
                vec![
                    Field::new("axis", "axis", "ProjAxis", None),
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("bound", "bound", "Amount", None),
                ],
            ),
        ],
        "Door" => vec![
            Constructor::new("thisDoor", "ThisDoor", vec![]),
            Constructor::new(
                "doorOf",
                "DoorOf",
                vec![
                    Field::new("state", "state", "Option LockState", None),
                    Field::new("room", "room", "NounPhrase", None),
                ],
            ),
        ],
        "RollWatch" => vec![
            Constructor::new("anyResult", "AnyResult", vec![]),
            Constructor::new(
                "resultIn",
                "ResultIn",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new("highestNatural", "HighestNatural", vec![]),
        ],
        "Condition" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Condition", None)],
            ),
            Constructor::new(
                "duringPart",
                "DuringPart",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "exists_",
                "Exists",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "did",
                "Did",
                vec![Field::new("clause", "clause", "DidClause", None)],
            ),
            Constructor::new(
                "matches",
                "Matches",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("predicate", "predicate", "Predicate", None),
                ],
            ),
            Constructor::new(
                "gameHas",
                "GameHas",
                vec![Field::new(
                    "designation",
                    "designation",
                    "DesignationLabel",
                    None,
                )],
            ),
            Constructor::new(
                "compareAmt",
                "CompareAmt",
                vec![
                    Field::new("subject", "subject", "Amount", None),
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("bound", "bound", "Amount", None),
                ],
            ),
            Constructor::new(
                "voteLead",
                "VoteLead",
                vec![
                    Field::new("label", "label", "VoteLabel", None),
                    Field::new("orTied", "or_tied", "Bool", None),
                ],
            ),
            Constructor::new(
                "anyResultIs",
                "AnyResultIs",
                vec![
                    Field::new("comparator", "comparator", "Comparator", None),
                    Field::new("bound", "bound", "Amount", None),
                ],
            ),
            Constructor::new("rolledDoubles", "RolledDoubles", vec![]),
            Constructor::new(
                "hasPriority",
                "HasPriority",
                vec![Field::new("player", "player", "NounPhrase", None)],
            ),
            Constructor::new(
                "soChooses",
                "SoChooses",
                vec![Field::new("chooser", "chooser", "NounPhrase", None)],
            ),
            Constructor::new(
                "heldSince",
                "HeldSince",
                vec![
                    Field::new("condition", "condition", "Condition", None),
                    Field::new("since", "since", "GameEvent", None),
                ],
            ),
            Constructor::new(
                "couldBeginToCast",
                "CouldBeginToCast",
                vec![
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("from_", "from_", "Location", None),
                ],
            ),
            Constructor::new(
                "not",
                "Not",
                vec![Field::new("condition", "condition", "Condition", None)],
            ),
            Constructor::new(
                "and",
                "And",
                vec![Field::new("conjuncts", "conjuncts", "List Condition", None)],
            ),
            Constructor::new(
                "or",
                "Or",
                vec![Field::new("disjuncts", "disjuncts", "List Condition", None)],
            ),
        ],
        "GameEvent" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "GameEvent", None)],
            ),
            Constructor::new(
                "does",
                "Does",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                ],
            ),
            Constructor::new(
                "beginningOf",
                "BeginningOf",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                    Field::new("which", "which", "TurnRef", None),
                ],
            ),
            Constructor::new(
                "endOf",
                "EndOf",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                    Field::new("which", "which", "TurnRef", None),
                ],
            ),
            Constructor::new("now", "Now", vec![]),
            Constructor::new(
                "stateHolds",
                "StateHolds",
                vec![Field::new("condition", "condition", "Condition", None)],
            ),
            Constructor::new(
                "chapterMark",
                "ChapterMark",
                vec![Field::new(
                    "chapters",
                    "chapters",
                    "List ChapterNumber",
                    None,
                )],
            ),
            Constructor::new(
                "nthOccurrence",
                "NthOccurrence",
                vec![
                    Field::new("ordinal", "ordinal", "Ordinal", None),
                    Field::new("per", "per", "Option Period", None),
                    Field::new("event", "event", "GameEvent", None),
                ],
            ),
            Constructor::new(
                "causes",
                "Causes",
                vec![
                    Field::new("cause", "cause", "Causing", None),
                    Field::new("event", "event", "GameEvent", None),
                ],
            ),
            Constructor::new(
                "stateBasedActionsChecked",
                "StateBasedActionsChecked",
                vec![],
            ),
        ],
        "Causing" => vec![
            Constructor::new(
                "source",
                "Source",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new(
                "event",
                "Event",
                vec![Field::new("event", "event", "GameEvent", None)],
            ),
            Constructor::new("anEffect", "AnEffect", vec![]),
        ],
        "FlipScope" => vec![
            Constructor::new(
                "count",
                "Count",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "per",
                "Per",
                vec![Field::new("each", "each", "NounPhrase", None)],
            ),
        ],
        "IgnoredOutcomes" => vec![
            Constructor::new(
                "extreme",
                "Extreme",
                vec![Field::new("extreme", "extreme", "RollExtreme", None)],
            ),
            Constructor::new(
                "allBut",
                "AllBut",
                vec![Field::new("extreme", "extreme", "RollExtreme", None)],
            ),
            Constructor::new(
                "chosen",
                "Chosen",
                vec![
                    Field::new("chooser", "chooser", "Option NounPhrase", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
        ],
        "Ballot" => vec![
            Constructor::new("anyBallot", "AnyBallot", vec![]),
            Constructor::new(
                "byLabel",
                "ByLabel",
                vec![Field::new("options", "options", "List VoteLabel", None)],
            ),
            Constructor::new(
                "byCandidate",
                "ByCandidate",
                vec![Field::new("candidates", "candidates", "NounPhrase", None)],
            ),
        ],
        "Exposed" => vec![
            Constructor::new(
                "cards",
                "Cards",
                vec![Field::new("cards", "cards", "NounPhrase", None)],
            ),
            Constructor::new(
                "zone",
                "Zone",
                vec![Field::new("zone", "zone", "Location", None)],
            ),
            Constructor::new(
                "choice",
                "Choice",
                vec![Field::new("sort", "sort", "ChoiceSort", None)],
            ),
        ],
        "VisibleThing" => vec![
            Constructor::new("topOfLibrary", "TopOfLibrary", vec![]),
            Constructor::new("wholeHand", "WholeHand", vec![]),
            Constructor::new(
                "objects",
                "Objects",
                vec![Field::new("objects", "objects", "NounPhrase", None)],
            ),
        ],
        "UsageLimit" => vec![Constructor::new(
            "mk",
            "Mk",
            vec![
                Field::new("times", "times", "Amount", None),
                Field::new("per", "per", "Period", None),
            ],
        )],
        "JoinedHeader" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("event", "event", "GameEvent", None),
                Field::new("alternatives", "alternatives", "List GameEvent", None),
                Field::new("while_", "while_", "Option Timing", None),
                Field::new("timing", "timing", "Option Timing", None),
            ],
        )],
        "SpendPurpose" => vec![
            Constructor::new(
                "toCast",
                "ToCast",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "toActivate",
                "ToActivate",
                vec![Field::new("source", "source", "Option Predicate", None)],
            ),
            Constructor::new(
                "toPay",
                "ToPay",
                vec![Field::new("cost", "cost", "CostNamed", None)],
            ),
            Constructor::new(
                "toTurnFaceUp",
                "ToTurnFaceUp",
                vec![Field::new("subject", "subject", "Predicate", None)],
            ),
        ],
        "AsThough" => vec![
            Constructor::new(
                "condition",
                "Condition",
                vec![Field::new("cond", "cond", "Condition", None)],
            ),
            Constructor::new(
                "mana",
                "Mana",
                vec![
                    Field::new("what", "what", "Option ColorOrColorless", None),
                    Field::new("as_", "as_", "ManaMatch", None),
                    Field::new("purpose", "purpose", "Option SpendPurpose", None),
                ],
            ),
            Constructor::new(
                "greater",
                "Greater",
                vec![
                    Field::new("stat", "stat", "Stat", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
        ],
        "Exchanged" => vec![
            Constructor::new(
                "lifeTotals",
                "LifeTotals",
                vec![Field::new("parties", "parties", "NounPhrase", None)],
            ),
            Constructor::new(
                "controlOf",
                "ControlOf",
                vec![
                    Field::new("left", "left", "NounPhrase", None),
                    Field::new("right", "right", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "cardsAcross",
                "CardsAcross",
                vec![
                    Field::new("left", "left", "NounPhrase", None),
                    Field::new("right", "right", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "zones",
                "Zones",
                vec![
                    Field::new("left", "left", "Location", None),
                    Field::new("right", "right", "Location", None),
                ],
            ),
            Constructor::new(
                "values",
                "Values",
                vec![
                    Field::new("left", "left", "Amount", None),
                    Field::new("right", "right", "Amount", None),
                ],
            ),
        ],
        "TextChange" => vec![
            Constructor::new("removeBracketed", "RemoveBracketed", vec![]),
            Constructor::new("targetToEach", "TargetToEach", vec![]),
            Constructor::new(
                "appendTextOf",
                "AppendTextOf",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new(
                "replaceWord",
                "ReplaceWord",
                vec![
                    Field::new("sorts", "sorts", "List TextWordSort", None),
                    Field::new("from_", "from_", "Option TextWord", None),
                    Field::new("to", "to", "Option TextWord", None),
                ],
            ),
            Constructor::new(
                "exchangeWith",
                "ExchangeWith",
                vec![Field::new("other", "other", "NounPhrase", None)],
            ),
        ],
        "WrittenQuality" => vec![
            Constructor::new(
                "everyTypeOf",
                "EveryTypeOf",
                vec![Field::new("space", "space", "SubtypeSpace", None)],
            ),
            Constructor::new(
                "chosen",
                "Chosen",
                vec![Field::new("quality", "quality", "Predicate", None)],
            ),
        ],
        "CounterKindSource" => vec![
            Constructor::new(
                "printed",
                "Printed",
                vec![Field::new("kind", "kind", "CounterKind", None)],
            ),
            Constructor::new(
                "chosen",
                "Chosen",
                vec![Field::new("menu", "menu", "List CounterKind", None)],
            ),
            Constructor::new(
                "distinctChosen",
                "DistinctChosen",
                vec![Field::new("menu", "menu", "List CounterKind", None)],
            ),
            Constructor::new("bound", "Bound", vec![]),
            Constructor::new("those", "Those", vec![]),
            Constructor::new(
                "sameAs",
                "SameAs",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new("anyKind", "AnyKind", vec![]),
        ],
        "ManaCostTerm" => vec![
            Constructor::new(
                "lit",
                "Lit",
                vec![Field::new("cost", "cost", "ManaCost", None)],
            ),
            Constructor::new(
                "manaCostOf",
                "ManaCostOf",
                vec![Field::new("object", "object", "NounPhrase", None)],
            ),
        ],
        "CostShift" => vec![
            Constructor::new(
                "generic",
                "Generic",
                vec![
                    Field::new("change", "change", "Delta Amount", None),
                    Field::new("floor", "floor", "Option Amount", None),
                ],
            ),
            Constructor::new(
                "bySymbols",
                "BySymbols",
                vec![
                    Field::new("change", "change", "Delta ManaCostTerm", None),
                    Field::new("coloredOnly", "colored_only", "Bool", None),
                ],
            ),
        ],
        "PipPaid" => vec![
            Constructor::new("generic", "Generic", vec![]),
            Constructor::new(
                "ofColor",
                "OfColor",
                vec![Field::new("color", "color", "ColorTerm", None)],
            ),
        ],
        "DeonticExcept" => vec![
            Constructor::new(
                "byKeyword",
                "ByKeyword",
                vec![Field::new("label", "label", "KeywordLabel", None)],
            ),
            Constructor::new(
                "byObjects",
                "ByObjects",
                vec![Field::new("objects", "objects", "NounPhrase", None)],
            ),
        ],
        "PreventCut" => vec![
            Constructor::new("all", "All", vec![]),
            Constructor::new(
                "some",
                "Some",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "shield",
                "Shield",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "allBut",
                "AllBut",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "half",
                "Half",
                vec![Field::new("rounding", "rounding", "RoundMode", None)],
            ),
        ],
        "DividedVerb" => vec![
            Constructor::new(
                "damage",
                "Damage",
                vec![Field::new("source", "source", "NounPhrase", None)],
            ),
            Constructor::new(
                "counters",
                "Counters",
                vec![Field::new("kind", "kind", "CounterKind", None)],
            ),
        ],
        "ProducedMana" => vec![
            Constructor::new(
                "runs",
                "Runs",
                vec![Field::new("runs", "runs", "List ProducedRun", None)],
            ),
            Constructor::new(
                "anyColor",
                "AnyColor",
                vec![Field::new("freedom", "freedom", "ColorFreedom", None)],
            ),
            Constructor::new(
                "ofChosenColor",
                "OfChosenColor",
                vec![Field::new(
                    "alternative",
                    "alternative",
                    "Option ProducedRun",
                    None,
                )],
            ),
            Constructor::new(
                "asPrintedCost",
                "AsPrintedCost",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "producedByEvent",
                "ProducedByEvent",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "couldProduce",
                "CouldProduce",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "amongColorsOf",
                "AmongColorsOf",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "amongWritten",
                "AmongWritten",
                vec![Field::new("colors", "colors", "List Color", None)],
            ),
            Constructor::new(
                "lastNoted",
                "LastNoted",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
        ],
        "DeckTrait" => vec![
            Constructor::new(
                "aCharacteristic",
                "ACharacteristic",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "manaValueParity",
                "ManaValueParity",
                vec![Field::new("parity", "parity", "Parity", None)],
            ),
            Constructor::new("repeatedManaSymbol", "RepeatedManaSymbol", vec![]),
            Constructor::new(
                "hasAbilityOf",
                "HasAbilityOf",
                vec![Field::new("class_", "class_", "AbilityClass", None)],
            ),
            Constructor::new(
                "anyOf",
                "AnyOf",
                vec![Field::new("traits", "traits", "List DeckTrait", None)],
            ),
        ],
        "DeckCondition" => vec![
            Constructor::new(
                "everyCardIs",
                "EveryCardIs",
                vec![
                    Field::new("scope", "scope", "Predicate", None),
                    Field::new("trait", "trait_", "DeckTrait", None),
                ],
            ),
            Constructor::new(
                "noCardIs",
                "NoCardIs",
                vec![
                    Field::new("scope", "scope", "Predicate", None),
                    Field::new("trait", "trait_", "DeckTrait", None),
                ],
            ),
            Constructor::new(
                "groupAgrees",
                "GroupAgrees",
                vec![Field::new("group", "group", "NounPhrase", None)],
            ),
            Constructor::new(
                "deckSizeOverMinimum",
                "DeckSizeOverMinimum",
                vec![Field::new("extra", "extra", "Nat", None)],
            ),
        ],
        "CombatParticipation" => vec![
            Constructor::new(
                "attacking",
                "Attacking",
                vec![Field::new(
                    "defender",
                    "defender",
                    "Option NounPhrase",
                    None,
                )],
            ),
            Constructor::new(
                "blocked",
                "Blocked",
                vec![Field::new("value", "value", "Bool", None)],
            ),
            Constructor::new("outsideCombat", "OutsideCombat", vec![]),
        ],
        "CombatUpdate" => vec![
            Constructor::new(
                "participation",
                "Participation",
                vec![Field::new("state", "state", "CombatParticipation", None)],
            ),
            Constructor::new(
                "blocking",
                "Blocking",
                vec![
                    Field::new("move", "move_", "AttachMove", None),
                    Field::new("attacker", "attacker", "NounPhrase", None),
                ],
            ),
        ],
        "Repetition" => vec![
            Constructor::new("again", "Again", vec![]),
            Constructor::new(
                "moreTimes",
                "MoreTimes",
                vec![Field::new("times", "times", "Amount", None)],
            ),
            Constructor::new("anyNumber", "AnyNumber", vec![]),
            Constructor::new(
                "untilCond",
                "UntilCond",
                vec![Field::new("condition", "condition", "Condition", None)],
            ),
            Constructor::new("againExcludingChosen", "AgainExcludingChosen", vec![]),
            Constructor::new(
                "fixed",
                "Fixed",
                vec![
                    Field::new("times", "times", "Amount", None),
                    Field::new("body", "body", "Action", None),
                ],
            ),
        ],
        "Characteristics" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("name", "name", "Option String", Some("none")),
                Field::new("cost", "cost", "Option ManaCost", Some("none")),
                Field::new("colors", "colors", "List Color", Some("[]")),
                Field::new("supertypes", "supertypes", "List Supertype", Some("[]")),
                Field::new("types", "types", "List CardType", Some("[]")),
                Field::new("subtypes", "subtypes", "List Subtype", Some("[]")),
                Field::new("text", "text", "List Ability", Some("[]")),
                Field::new("power", "power", "Option Amount", Some("none")),
                Field::new("toughness", "toughness", "Option Amount", Some("none")),
                Field::new("loyalty", "loyalty", "Option Amount", Some("none")),
                Field::new("defense", "defense", "Option Amount", Some("none")),
            ],
        )],
        "CharacteristicBundle" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new(
                    "characteristics",
                    "characteristics",
                    "Characteristics",
                    None,
                ),
                Field::new("qualities", "qualities", "List WrittenQuality", Some("[]")),
            ],
        )],
        "FaceDown" => vec![Constructor::new(
            "mk",
            "",
            vec![Field::new(
                "characteristics",
                "characteristics",
                "Option Characteristics",
                None,
            )],
        )],
        "CharacteristicEdit" => vec![
            Constructor::new(
                "typeLine",
                "TypeLine",
                vec![
                    Field::new("op", "op", "QualityOp", None),
                    Field::new("changes", "changes", "TypeLineChanges", None),
                ],
            ),
            Constructor::new(
                "name",
                "Name",
                vec![
                    Field::new("op", "op", "QualityOp", None),
                    Field::new("name", "name", "String", None),
                ],
            ),
            Constructor::new(
                "manaCost",
                "ManaCost",
                vec![Field::new("cost", "cost", "Option ManaCost", None)],
            ),
            Constructor::new(
                "colors",
                "Colors",
                vec![
                    Field::new("op", "op", "QualityOp", None),
                    Field::new("colors", "colors", "ColorSpec", None),
                ],
            ),
            Constructor::new(
                "stat",
                "Stat",
                vec![
                    Field::new("op", "op", "QualityOp", None),
                    Field::new("axis", "axis", "CharacteristicStat", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
            Constructor::new("ptSwitch", "PtSwitch", vec![]),
            Constructor::new(
                "quality",
                "Quality",
                vec![
                    Field::new("op", "op", "QualityOp", None),
                    Field::new("quality", "quality", "WrittenQuality", None),
                ],
            ),
            Constructor::new(
                "addedAbilities",
                "AddedAbilities",
                vec![Field::new("abilities", "abilities", "List Ability", None)],
            ),
            Constructor::new(
                "addedAbilitiesOf",
                "AddedAbilitiesOf",
                vec![
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("classes", "classes", "List AbilityClass", None),
                    Field::new("except", "except", "Option Predicate", None),
                ],
            ),
            Constructor::new(
                "addedSelectionOf",
                "AddedSelectionOf",
                vec![
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("selection", "selection", "AbilitySelection", None),
                ],
            ),
            Constructor::new(
                "removedAbilities",
                "RemovedAbilities",
                vec![Field::new(
                    "selection",
                    "selection",
                    "AbilitySelection",
                    None,
                )],
            ),
        ],
        "AbilitySelection" => vec![
            Constructor::new(
                "specified",
                "Specified",
                vec![Field::new(
                    "abilities",
                    "abilities",
                    "List AbilityLost",
                    None,
                )],
            ),
            Constructor::new(
                "family",
                "Family",
                vec![Field::new("class_", "class_", "AbilityClass", None)],
            ),
            Constructor::new(
                "allExcept",
                "AllExcept",
                vec![Field::new("except", "except", "Option Predicate", None)],
            ),
            Constructor::new(
                "printedAfter",
                "PrintedAfter",
                vec![Field::new("keyword", "keyword", "KeywordLabel", None)],
            ),
        ],
        "TokenSpec" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "TokenSpec", None)],
            ),
            Constructor::new(
                "written",
                "Written",
                vec![Field::new(
                    "characteristics",
                    "characteristics",
                    "CharacteristicBundle",
                    None,
                )],
            ),
            Constructor::new("asThose", "AsThose", vec![]),
            Constructor::new(
                "copyOf",
                "CopyOf",
                vec![
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("exceptions", "exceptions", "List CopyExcept", None),
                ],
            ),
            Constructor::new(
                "doubleFaced",
                "DoubleFaced",
                vec![
                    Field::new("front", "front", "CharacteristicBundle", None),
                    Field::new("back", "back", "CharacteristicBundle", None),
                ],
            ),
        ],
        "WouldClause" => vec![
            Constructor::new(
                "would",
                "Would",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                ],
            ),
            Constructor::new(
                "wouldAny",
                "WouldAny",
                vec![Field::new("clauses", "clauses", "List WouldClause", None)],
            ),
            Constructor::new(
                "causedBy",
                "CausedBy",
                vec![
                    Field::new("cause", "cause", "Causing", None),
                    Field::new("clause", "clause", "WouldClause", None),
                ],
            ),
            Constructor::new(
                "nthOccurrence",
                "NthOccurrence",
                vec![
                    Field::new("ordinal", "ordinal", "Ordinal", None),
                    Field::new("per", "per", "Option Period", None),
                    Field::new("clause", "clause", "WouldClause", None),
                ],
            ),
        ],
        "Replacement" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Replacement", None)],
            ),
            Constructor::new(
                "instead",
                "Instead",
                vec![Field::new("action", "action", "Action", None)],
            ),
            Constructor::new(
                "before",
                "Before",
                vec![
                    Field::new("action", "action", "Action", None),
                    Field::new("body", "body", "Option Replacement", None),
                ],
            ),
            Constructor::new(
                "alongside",
                "Alongside",
                vec![Field::new("action", "action", "Action", None)],
            ),
            Constructor::new(
                "withRiders",
                "WithRiders",
                vec![Field::new("riders", "riders", "List MoveRider", None)],
            ),
            Constructor::new(
                "onlyIf",
                "OnlyIf",
                vec![
                    Field::new("condition", "condition", "Condition", None),
                    Field::new("body", "body", "Replacement", None),
                ],
            ),
        ],
        "DeonticMods" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("timing", "timing", "Option Timing", Some("none")),
                Field::new("asThough", "as_though", "Option AsThough", Some("none")),
                Field::new("except", "except", "Option DeonticExcept", Some("none")),
                Field::new("cause", "cause", "Option Causing", Some("none")),
                Field::new("rider", "rider", "Option Action", Some("none")),
            ],
        )],
        "StaticSpec" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "StaticSpec", None)],
            ),
            Constructor::new(
                "costShift",
                "CostShift",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("shift", "shift", "CostShift", None),
                ],
            ),
            Constructor::new(
                "altCost",
                "AltCost",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("cost", "cost", "Option Cost", None),
                ],
            ),
            Constructor::new(
                "addedCost",
                "AddedCost",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("offered", "offered", "Bool", None),
                    Field::new("times", "times", "PayTimes", None),
                ],
            ),
            Constructor::new(
                "letterDefinition",
                "LetterDefinition",
                vec![
                    Field::new("letter", "letter", "Letter", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
            Constructor::new(
                "cant",
                "Cant",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("mods", "mods", "DeonticMods", None),
                ],
            ),
            Constructor::new(
                "must",
                "Must",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("mods", "mods", "DeonticMods", None),
                ],
            ),
            Constructor::new(
                "can",
                "Can",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("mods", "mods", "DeonticMods", None),
                ],
            ),
            Constructor::new(
                "asThough",
                "AsThough",
                vec![
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("asThough", "as_though", "AsThough", None),
                ],
            ),
            Constructor::new(
                "gatedBy",
                "GatedBy",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("subj", "subj", "Subject", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("mods", "mods", "DeonticMods", None),
                ],
            ),
            Constructor::new(
                "manaRetention",
                "ManaRetention",
                vec![
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("mana", "mana", "ManaHeld", None),
                ],
            ),
            Constructor::new(
                "partSkip",
                "PartSkip",
                vec![
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("part", "part", "TurnPart", None),
                ],
            ),
            Constructor::new(
                "characteristicChange",
                "CharacteristicChange",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("edits", "edits", "List CharacteristicEdit", None),
                ],
            ),
            Constructor::new(
                "characteristicDefining",
                "CharacteristicDefining",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("edits", "edits", "List CharacteristicEdit", None),
                ],
            ),
            Constructor::new(
                "retention",
                "Retention",
                vec![
                    Field::new("spec", "spec", "StaticSpec", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "copyChange",
                "CopyChange",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("exceptions", "exceptions", "List CopyExcept", None),
                ],
            ),
            Constructor::new(
                "controlGrant",
                "ControlGrant",
                vec![
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "replace",
                "Replace",
                vec![
                    Field::new("event", "event", "WouldClause", None),
                    Field::new("replacement", "replacement", "Replacement", None),
                    Field::new("timing", "timing", "Option Timing", None),
                    Field::new("use", "use_", "ReplUse", None),
                    Field::new("limit", "limit", "Option UsageLimit", None),
                ],
            ),
            Constructor::new(
                "conditional",
                "Conditional",
                vec![
                    Field::new("spec", "spec", "StaticSpec", None),
                    Field::new("condition", "condition", "Condition", None),
                    Field::new("marking", "marking", "CondMarking", None),
                ],
            ),
            Constructor::new(
                "visibility",
                "Visibility",
                vec![
                    Field::new("verb", "verb", "ExposeVerb", None),
                    Field::new("player", "player", "NounPhrase", None),
                    Field::new("what", "what", "VisibleThing", None),
                ],
            ),
            Constructor::new(
                "additionalTriggers",
                "AdditionalTriggers",
                vec![
                    Field::new("event", "event", "GameEvent", None),
                    Field::new("quantity", "quantity", "Quantity", None),
                ],
            ),
            Constructor::new(
                "conjunction",
                "Conjunction",
                vec![
                    Field::new("subject", "subject", "Option NounPhrase", None),
                    Field::new("parts", "parts", "List StaticSpec", None),
                ],
            ),
            Constructor::new(
                "paySubstitute",
                "PaySubstitute",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("substitute", "substitute", "ManaSubstitute", None),
                ],
            ),
            Constructor::new(
                "specialAction",
                "SpecialAction",
                vec![
                    Field::new("player", "player", "Option NounPhrase", None),
                    Field::new("when", "when", "Option Timing", None),
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("does", "does", "Action", None),
                ],
            ),
            Constructor::new(
                "asEvent",
                "AsEvent",
                vec![
                    Field::new("event", "event", "GameEvent", None),
                    Field::new("does", "does", "Action", None),
                ],
            ),
            Constructor::new(
                "resolutionDelayedTrigger",
                "ResolutionDelayedTrigger",
                vec![Field::new("does", "does", "Action", None)],
            ),
            Constructor::new(
                "whileHolds",
                "WhileHolds",
                vec![
                    Field::new("condition", "condition", "Condition", None),
                    Field::new("does", "does", "Action", None),
                ],
            ),
            Constructor::new(
                "rollModifier",
                "RollModifier",
                vec![
                    Field::new("roll", "roll", "GameEvent", None),
                    Field::new("modify", "modify", "Action", None),
                    Field::new("limit", "limit", "Option UsageLimit", None),
                ],
            ),
            Constructor::new(
                "combatant",
                "Combatant",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "modeChoice",
                "ModeChoice",
                vec![Field::new("change", "change", "ModeChange", None)],
            ),
            Constructor::new(
                "addsTarget",
                "AddsTarget",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("description", "description", "Predicate", None),
                ],
            ),
            Constructor::new(
                "textChange",
                "TextChange",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("change", "change", "TextChange", None),
                ],
            ),
            Constructor::new(
                "revealedAsCast",
                "RevealedAsCast",
                vec![
                    Field::new("spell", "spell", "NounPhrase", None),
                    Field::new("grants", "grants", "StaticSpec", None),
                ],
            ),
        ],
        "ModeChange" => vec![
            Constructor::new(
                "allInstead",
                "AllInstead",
                vec![
                    Field::new("cost", "cost", "Option Cost", None),
                    Field::new("condition", "condition", "Option Condition", None),
                ],
            ),
            Constructor::new(
                "extraModesCost",
                "ExtraModesCost",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new(
                "setQuantity",
                "SetQuantity",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new("repeatable", "Repeatable", vec![]),
            Constructor::new(
                "notChosenBefore",
                "NotChosenBefore",
                vec![Field::new("within", "within", "Duration", None)],
            ),
        ],
        "PlayPayment" => vec![
            Constructor::new("itsOwnCost", "ItsOwnCost", vec![]),
            Constructor::new("withoutPaying", "WithoutPaying", vec![]),
            Constructor::new(
                "payingInstead",
                "PayingInstead",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new(
                "plusAdditional",
                "PlusAdditional",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new(
                "faceDown",
                "FaceDown",
                vec![
                    Field::new("spec", "spec", "FaceDown", None),
                    Field::new("cost", "cost", "Cost", None),
                ],
            ),
            Constructor::new("prototyped", "Prototyped", vec![]),
            Constructor::new(
                "transformed",
                "Transformed",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new("fused", "Fused", vec![]),
            Constructor::new(
                "byKeyword",
                "ByKeyword",
                vec![Field::new("label", "label", "KeywordLabel", None)],
            ),
        ],
        "MoveRider" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "MoveRider", None)],
            ),
            Constructor::new(
                "entersAs",
                "EntersAs",
                vec![Field::new("status", "status", "Status", None)],
            ),
            Constructor::new(
                "entersAttacking",
                "EntersAttacking",
                vec![Field::new(
                    "defender",
                    "defender",
                    "Option NounPhrase",
                    None,
                )],
            ),
            Constructor::new("entersTransformed", "EntersTransformed", vec![]),
            Constructor::new(
                "entersMelded",
                "EntersMelded",
                vec![Field::new("into", "into", "String", None)],
            ),
            Constructor::new(
                "withCounters",
                "WithCounters",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("kind", "kind", "CounterKindSource", None),
                    Field::new("mark", "mark", "EntryCounterMark", None),
                ],
            ),
            Constructor::new(
                "under",
                "Under",
                vec![Field::new("controller", "controller", "NounPhrase", None)],
            ),
            Constructor::new(
                "asCopyOf",
                "AsCopyOf",
                vec![
                    Field::new("optional", "optional", "Bool", None),
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("exceptions", "exceptions", "List CopyExcept", None),
                ],
            ),
            Constructor::new(
                "faceDown",
                "FaceDown",
                vec![Field::new("spec", "spec", "FaceDown", None)],
            ),
            Constructor::new(
                "inOrder",
                "InOrder",
                vec![Field::new(
                    "arrangement",
                    "arrangement",
                    "Arrangement",
                    None,
                )],
            ),
            Constructor::new(
                "attachedTo",
                "AttachedTo",
                vec![Field::new("host", "host", "NounPhrase", None)],
            ),
            Constructor::new(
                "not",
                "Not",
                vec![Field::new("rider", "rider", "MoveRider", None)],
            ),
        ],
        "Cost" => vec![
            Constructor::new("anyCost", "AnyCost", vec![]),
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Cost", None)],
            ),
            Constructor::new(
                "mana",
                "Mana",
                vec![Field::new("cost", "cost", "ManaCost", None)],
            ),
            Constructor::new(
                "scaled",
                "Scaled",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
            Constructor::new("tapSymbol", "TapSymbol", vec![]),
            Constructor::new("untapSymbol", "UntapSymbol", vec![]),
            Constructor::new(
                "loyaltySymbol",
                "LoyaltySymbol",
                vec![Field::new("change", "change", "Delta Amount", None)],
            ),
            Constructor::new(
                "perform",
                "Perform",
                vec![Field::new("action", "action", "Action", None)],
            ),
            Constructor::new(
                "compound",
                "Compound",
                vec![Field::new("costs", "costs", "List Cost", None)],
            ),
            Constructor::new(
                "or",
                "Or",
                vec![Field::new("costs", "costs", "List Cost", None)],
            ),
            Constructor::new("itsManaCost", "ItsManaCost", vec![]),
            Constructor::new(
                "withSubstitute",
                "WithSubstitute",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("substitute", "substitute", "ManaSubstitute", None),
                ],
            ),
        ],
        "ManaSubstitute" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("action", "action", "Action", None),
                Field::new("pays", "pays", "List PipPaid", None),
            ],
        )],
        "ManaRider" => vec![
            Constructor::new(
                "spendOnly",
                "SpendOnly",
                vec![Field::new(
                    "purposes",
                    "purposes",
                    "List SpendPurpose",
                    None,
                )],
            ),
            Constructor::new(
                "spendNotOn",
                "SpendNotOn",
                vec![Field::new(
                    "purposes",
                    "purposes",
                    "List SpendPurpose",
                    None,
                )],
            ),
            Constructor::new(
                "onSpent",
                "OnSpent",
                vec![
                    Field::new("mode", "mode", "SpentMode", None),
                    Field::new("only", "only", "Bool", None),
                    Field::new("spell", "spell", "NounPhrase", None),
                    Field::new("says", "says", "Action", None),
                ],
            ),
        ],
        "CopyExcept" => vec![
            Constructor::new(
                "edits",
                "Edits",
                vec![Field::new(
                    "edits",
                    "edits",
                    "List CharacteristicEdit",
                    None,
                )],
            ),
            Constructor::new(
                "ability",
                "Ability",
                vec![Field::new("ability", "ability", "Ability", None)],
            ),
            Constructor::new("thisAbility", "ThisAbility", vec![]),
            Constructor::new(
                "entersWithCounters",
                "EntersWithCounters",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("kind", "kind", "CounterKind", None),
                    Field::new("mark", "mark", "EntryCounterMark", None),
                ],
            ),
        ],
        "RollRow" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("results", "results", "Quantity", None),
                Field::new("action", "action", "Action", None),
            ],
        )],
        "CreationSpec" => vec![
            Constructor::new(
                "token",
                "Token",
                vec![
                    Field::new("spec", "spec", "TokenSpec", None),
                    Field::new("riders", "riders", "List MoveRider", None),
                ],
            ),
            Constructor::new(
                "emblem",
                "Emblem",
                vec![
                    Field::new("name", "name", "Option String", None),
                    Field::new("abilities", "abilities", "List Ability", None),
                ],
            ),
        ],
        "Action" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Action", None)],
            ),
            Constructor::new(
                "ownScope",
                "OwnScope",
                vec![
                    Field::new("deed", "deed", "Option Deed", None),
                    Field::new("body", "body", "Action", None),
                ],
            ),
            Constructor::new(
                "dealDamage",
                "DealDamage",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("recipient", "recipient", "NounPhrase", None),
                    Field::new("kind", "kind", "Option DamageKind", Some("none")),
                ],
            ),
            Constructor::new(
                "markDamage",
                "MarkDamage",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("amount", "amount", "Amount", None),
                ],
            ),
            Constructor::new(
                "preventDamage",
                "PreventDamage",
                vec![
                    Field::new("cut", "cut", "PreventCut", None),
                    Field::new("damage", "damage", "Option WouldClause", None),
                    Field::new("also", "also", "Option Action", None),
                ],
            ),
            Constructor::new(
                "redirectDamage",
                "RedirectDamage",
                vec![
                    Field::new("cut", "cut", "PreventCut", None),
                    Field::new("to", "to", "NounPhrase", None),
                    Field::new("damage", "damage", "Option WouldClause", None),
                ],
            ),
            Constructor::new(
                "clearDamage",
                "ClearDamage",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "setStatus",
                "SetStatus",
                vec![
                    Field::new("status", "status", "Status", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "turnOver",
                "TurnOver",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("into", "into", "Option Predicate", None),
                ],
            ),
            Constructor::new(
                "combat",
                "Combat",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("update", "update", "CombatUpdate", None),
                ],
            ),
            Constructor::new(
                "attack",
                "Attack",
                vec![Field::new(
                    "defender",
                    "defender",
                    "Option NounPhrase",
                    None,
                )],
            ),
            Constructor::new(
                "block",
                "Block",
                vec![Field::new("attacker", "attacker", "NounPhrase", None)],
            ),
            Constructor::new(
                "attackUnblocked",
                "AttackUnblocked",
                vec![Field::new(
                    "defender",
                    "defender",
                    "Option NounPhrase",
                    None,
                )],
            ),
            Constructor::new(
                "attackWith",
                "AttackWith",
                vec![
                    Field::new("attackers", "attackers", "NounPhrase", None),
                    Field::new("defender", "defender", "Option NounPhrase", None),
                ],
            ),
            Constructor::new("trigger", "Trigger", vec![]),
            Constructor::new(
                "crew",
                "Crew",
                vec![Field::new("vehicle", "vehicle", "NounPhrase", None)],
            ),
            Constructor::new(
                "saddle",
                "Saddle",
                vec![Field::new("permanent", "permanent", "NounPhrase", None)],
            ),
            Constructor::new(
                "target",
                "Target",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "tapForMana",
                "TapForMana",
                vec![
                    Field::new("permanent", "permanent", "NounPhrase", None),
                    Field::new("type", "type_", "Option ManaTypeTerm", None),
                ],
            ),
            Constructor::new("commitCrime", "CommitCrime", vec![]),
            Constructor::new(
                "attachment",
                "Attachment",
                vec![
                    Field::new("move", "move_", "AttachMove", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("host", "host", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "changeControl",
                "ChangeControl",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("from_", "from_", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "gainDesignation",
                "GainDesignation",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("designation", "designation", "DesignationLabel", None),
                    Field::new("duration", "duration", "Option Duration", None),
                ],
            ),
            Constructor::new(
                "gameGains",
                "GameGains",
                vec![Field::new(
                    "designation",
                    "designation",
                    "DesignationLabel",
                    None,
                )],
            ),
            Constructor::new(
                "unlock",
                "Unlock",
                vec![Field::new("door", "door", "Door", None)],
            ),
            Constructor::new(
                "fullyUnlock",
                "FullyUnlock",
                vec![Field::new("room", "room", "NounPhrase", None)],
            ),
            Constructor::new(
                "conclude",
                "Conclude",
                vec![
                    Field::new("verb", "verb", "OutcomeVerb", None),
                    Field::new("cause", "cause", "Option StateBasedCause", Some("none")),
                ],
            ),
            Constructor::new("restartGame", "RestartGame", vec![]),
            Constructor::new(
                "separateIntoPiles",
                "SeparateIntoPiles",
                vec![
                    Field::new("group", "group", "NounPhrase", None),
                    Field::new("piles", "piles", "Nat", None),
                    Field::new("faces", "faces", "List PileFace", None),
                ],
            ),
            Constructor::new(
                "choose",
                "Choose",
                vec![
                    Field::new("first", "first", "Option NounPhrase", None),
                    Field::new("chosen", "chosen", "NounPhrase", None),
                    Field::new("disclosure", "disclosure", "Disclosure", None),
                    Field::new("when", "when", "Option Timing", None),
                ],
            ),
            Constructor::new(
                "revealChoices",
                "RevealChoices",
                vec![Field::new("sort", "sort", "HiddenSort", None)],
            ),
            Constructor::new(
                "vote",
                "Vote",
                vec![
                    Field::new("first", "first", "Option NounPhrase", None),
                    Field::new("disclosure", "disclosure", "Disclosure", None),
                    Field::new("ballot", "ballot", "Ballot", None),
                    Field::new("times", "times", "Option Quantity", None),
                ],
            ),
            Constructor::new(
                "move",
                "Move",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("from_", "from_", "Location", None),
                    Field::new("to", "to", "Option Location", None),
                    Field::new("at_", "at_", "ObservationPoint", None),
                    Field::new("riders", "riders", "List MoveRider", None),
                ],
            ),
            Constructor::new(
                "turnFaceDown",
                "TurnFaceDown",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("spec", "spec", "FaceDown", None),
                ],
            ),
            Constructor::new(
                "copy",
                "Copy",
                vec![
                    Field::new("sort", "sort", "CopySort", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("times", "times", "Amount", None),
                    Field::new("exceptions", "exceptions", "List CopyExcept", None),
                ],
            ),
            Constructor::new(
                "chooseNewTargets",
                "ChooseNewTargets",
                vec![Field::new("subject", "subject", "NounPhrase", None)],
            ),
            Constructor::new(
                "copyTargets",
                "CopyTargets",
                vec![
                    Field::new("copy", "copy", "NounPhrase", None),
                    Field::new("original", "original", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "changeLife",
                "ChangeLife",
                vec![Field::new("delta", "delta", "Delta Amount", None)],
            ),
            Constructor::new(
                "exchange",
                "Exchange",
                vec![Field::new("exchanged", "exchanged", "Exchanged", None)],
            ),
            Constructor::new(
                "addMana",
                "AddMana",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("produced", "produced", "ProducedMana", None),
                    Field::new("riders", "riders", "List ManaRider", None),
                ],
            ),
            Constructor::new(
                "draw",
                "Draw",
                vec![Field::new("quantity", "quantity", "Quantity", None)],
            ),
            Constructor::new(
                "expose",
                "Expose",
                vec![
                    Field::new("verb", "verb", "ExposeVerb", None),
                    Field::new("exposed", "exposed", "Exposed", None),
                ],
            ),
            Constructor::new(
                "search",
                "Search",
                vec![
                    Field::new("scope", "scope", "Location", None),
                    Field::new("for_", "for_", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "shuffle",
                "Shuffle",
                vec![Field::new("pile", "pile", "Location", None)],
            ),
            Constructor::new(
                "flipCoins",
                "FlipCoins",
                vec![
                    Field::new("count", "count", "FlipScope", None),
                    Field::new("call", "call", "Option FlipCall", None),
                    Field::new("outcome", "outcome", "Option CoinFace", None),
                ],
            ),
            Constructor::new(
                "rollDice",
                "RollDice",
                vec![
                    Field::new("count", "count", "Amount", None),
                    Field::new("sides", "sides", "DieSides", None),
                    Field::new("watch", "watch", "RollWatch", None),
                ],
            ),
            Constructor::new(
                "applyResultsTable",
                "ApplyResultsTable",
                vec![Field::new("rows", "rows", "List RollRow", None)],
            ),
            Constructor::new(
                "ignoreOutcomes",
                "IgnoreOutcomes",
                vec![Field::new("which", "which", "IgnoredOutcomes", None)],
            ),
            Constructor::new(
                "shiftResult",
                "ShiftResult",
                vec![Field::new(
                    "changes",
                    "changes",
                    "List (Delta Amount)",
                    None,
                )],
            ),
            Constructor::new(
                "storeResults",
                "StoreResults",
                vec![Field::new("on", "on", "NounPhrase", None)],
            ),
            Constructor::new(
                "rerollStored",
                "RerollStored",
                vec![
                    Field::new("quantity", "quantity", "Quantity", None),
                    Field::new("whose", "whose", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "establish",
                "Establish",
                vec![
                    Field::new("spec", "spec", "StaticSpec", None),
                    Field::new("duration", "duration", "Option Duration", None),
                ],
            ),
            Constructor::new(
                "createObject",
                "CreateObject",
                vec![
                    Field::new("count", "count", "Amount", None),
                    Field::new("spec", "spec", "CreationSpec", None),
                    Field::new("under", "under", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "putCounters",
                "PutCounters",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("kind", "kind", "CounterKindSource", None),
                    Field::new("on", "on", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "distribute",
                "Distribute",
                vec![
                    Field::new("verb", "verb", "DividedVerb", None),
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("among", "among", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "removeCounters",
                "RemoveCounters",
                vec![
                    Field::new("quantity", "quantity", "Option Quantity", None),
                    Field::new("kind", "kind", "Option CounterKindSource", None),
                    Field::new("from_", "from_", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "moveCounters",
                "MoveCounters",
                vec![
                    Field::new("amount", "amount", "Amount", None),
                    Field::new("kind", "kind", "Option CounterKindSource", None),
                    Field::new("source", "source", "NounPhrase", None),
                    Field::new("destination", "destination", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "pay",
                "Pay",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("times", "times", "PayTimes", None),
                    Field::new("outcome", "outcome", "PaymentOutcome", None),
                    Field::new("whose", "whose", "Option NounPhrase", None),
                    Field::new("keyword", "keyword", "Option KeywordLabel", None),
                ],
            ),
            Constructor::new(
                "instruct",
                "Instruct",
                vec![
                    Field::new("subj", "subj", "NounPhrase", None),
                    Field::new("action", "action", "Action", None),
                ],
            ),
            Constructor::new(
                "withContinuation",
                "WithContinuation",
                vec![
                    Field::new("policy", "policy", "ContinuationPolicy", None),
                    Field::new("body", "body", "Action", None),
                    Field::new("ifDid", "if_did", "Option Action", None),
                    Field::new("ifNot", "if_not", "Option Action", None),
                ],
            ),
            Constructor::new(
                "limited",
                "Limited",
                vec![
                    Field::new("action", "action", "Action", None),
                    Field::new("limit", "limit", "UsageLimit", None),
                ],
            ),
            Constructor::new(
                "doIf",
                "DoIf",
                vec![
                    Field::new("condition", "condition", "Condition", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("otherwise", "otherwise", "Option Action", None),
                    Field::new("postposed", "postposed", "Bool", Some("false")),
                ],
            ),
            Constructor::new(
                "doForEach",
                "DoForEach",
                vec![
                    Field::new("group", "group", "NounPhrase", None),
                    Field::new("body", "body", "Action", None),
                ],
            ),
            Constructor::new(
                "doForEachKind",
                "DoForEachKind",
                vec![
                    Field::new("axis", "axis", "KindAxis", None),
                    Field::new("domain", "domain", "Option NounPhrase", None),
                    Field::new("sort", "sort", "QualitySort", None),
                    Field::new("body", "body", "Action", None),
                ],
            ),
            Constructor::new(
                "repeat_",
                "Repeat",
                vec![Field::new("repetition", "repetition", "Repetition", None)],
            ),
            Constructor::new(
                "sequentially",
                "Sequentially",
                vec![Field::new("steps", "steps", "List Action", None)],
            ),
            Constructor::new(
                "simultaneously",
                "Simultaneously",
                vec![Field::new("steps", "steps", "List Action", None)],
            ),
            Constructor::new(
                "chooseModes",
                "ChooseModes",
                vec![
                    Field::new("quantity", "quantity", "Quantity", None),
                    Field::new(
                        "modes",
                        "modes",
                        "List (Option Cost × Option Nat × Action)",
                        None,
                    ),
                ],
            ),
            Constructor::new(
                "delay",
                "Delay",
                vec![
                    Field::new("event", "event", "GameEvent", None),
                    Field::new("alternatives", "alternatives", "List GameEvent", None),
                    Field::new("duration", "duration", "Option Duration", None),
                    Field::new("body", "body", "Action", None),
                ],
            ),
            Constructor::new(
                "replace",
                "Replace",
                vec![
                    Field::new("replaced", "replaced", "Action", None),
                    Field::new("replacement", "replacement", "Replacement", None),
                ],
            ),
            Constructor::new(
                "holdUntil",
                "HoldUntil",
                vec![
                    Field::new("action", "action", "Action", None),
                    Field::new("event", "event", "GameEvent", None),
                ],
            ),
            Constructor::new(
                "triggerReflexively",
                "TriggerReflexively",
                vec![
                    Field::new("body", "body", "Action", None),
                    Field::new("event", "event", "Option GameEvent", None),
                    Field::new("trigger", "trigger", "Action", None),
                ],
            ),
            Constructor::new(
                "skipUntap",
                "SkipUntap",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("steps", "steps", "Amount", None),
                ],
            ),
            Constructor::new(
                "skipPart",
                "SkipPart",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("count", "count", "Amount", None),
                ],
            ),
            Constructor::new(
                "addTurnPart",
                "AddTurnPart",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("anchor", "anchor", "Option TurnPart", None),
                    Field::new("count", "count", "Amount", None),
                    Field::new("followedBy", "followed_by", "Option TurnPart", None),
                ],
            ),
            Constructor::new(
                "procedure",
                "Procedure",
                vec![
                    Field::new("kind", "kind", "Procedure", None),
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("payment", "payment", "PlayPayment", None),
                    Field::new("resulting", "resulting", "Option Predicate", None),
                    Field::new("from_", "from_", "Option Location", None),
                    Field::new("of_", "of_", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "relate",
                "Relate",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("label", "label", "RelationLabel", None),
                    Field::new("counterpart", "counterpart", "NounPhrase", None),
                ],
            ),
            Constructor::new(
                "merge",
                "Merge",
                vec![
                    Field::new("subject", "subject", "NounPhrase", None),
                    Field::new("onto", "onto", "NounPhrase", None),
                    Field::new("position", "position", "MergePosition", None),
                ],
            ),
        ],
        "KeywordParam" => vec![
            Constructor::new(
                "cost",
                "Cost",
                vec![Field::new("cost", "cost", "Cost", None)],
            ),
            Constructor::new(
                "quality",
                "Quality",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "subject",
                "Subject",
                vec![Field::new("predicate", "predicate", "Predicate", None)],
            ),
            Constructor::new(
                "number",
                "Number",
                vec![Field::new("amount", "amount", "Amount", None)],
            ),
            Constructor::new(
                "deckCondition",
                "DeckCondition",
                vec![Field::new("condition", "condition", "DeckCondition", None)],
            ),
            Constructor::new(
                "ability",
                "Ability",
                vec![Field::new("ability", "ability", "Ability", None)],
            ),
        ],
        "AbilityLost" => vec![
            Constructor::new(
                "written",
                "Written",
                vec![Field::new("ability", "ability", "Ability", None)],
            ),
            Constructor::new(
                "term",
                "Term",
                vec![Field::new("keyword", "keyword", "KeywordTerm", None)],
            ),
        ],
        "Ability" => vec![
            Constructor::new(
                "inCaller",
                "InCaller",
                vec![Field::new("body", "body", "Ability", None)],
            ),
            Constructor::new(
                "keyword",
                "Keyword",
                vec![
                    Field::new("keyword", "keyword", "KeywordLabel", None),
                    Field::new("params", "params", "List KeywordParam", None),
                    Field::new("body", "body", "List Ability", None),
                ],
            ),
            Constructor::new(
                "activated",
                "Activated",
                vec![
                    Field::new("cost", "cost", "Cost", None),
                    Field::new("action", "action", "Action", None),
                    Field::new("timing", "timing", "Option Timing", None),
                    Field::new("limit", "limit", "Option UsageLimit", None),
                    Field::new("guard", "guard", "Option Condition", None),
                    Field::new("activator", "activator", "Option NounPhrase", None),
                ],
            ),
            Constructor::new(
                "triggered",
                "Triggered",
                vec![
                    Field::new("event", "event", "GameEvent", None),
                    Field::new("alternatives", "alternatives", "List GameEvent", None),
                    Field::new("while_", "while_", "Option Timing", None),
                    Field::new("joins", "joins", "List JoinedHeader", None),
                    Field::new("timing", "timing", "Option Timing", None),
                    Field::new("limit", "limit", "Option UsageLimit", None),
                    Field::new("intervening", "intervening", "Option Condition", None),
                    Field::new("action", "action", "Action", None),
                ],
            ),
            Constructor::new(
                "static",
                "Static",
                vec![Field::new("spec", "spec", "StaticSpec", None)],
            ),
            Constructor::new(
                "spell",
                "Spell",
                vec![
                    Field::new("timing", "timing", "Option Timing", None),
                    Field::new("action", "action", "Action", None),
                ],
            ),
            Constructor::new("mayBeginOnBattlefield", "MayBeginOnBattlefield", vec![]),
            Constructor::new(
                "italicHead",
                "ItalicHead",
                vec![
                    Field::new("word", "word", "ItalicWord", None),
                    Field::new("ability", "ability", "Ability", None),
                ],
            ),
            Constructor::new(
                "thatAbility",
                "ThatAbility",
                vec![Field::new("ref", "ref_", "ChoiceRef", None)],
            ),
            Constructor::new(
                "qualified",
                "Qualified",
                vec![
                    Field::new("ability", "ability", "Ability", None),
                    Field::new("timing", "timing", "Option Timing", None),
                    Field::new("limit", "limit", "Option UsageLimit", None),
                    Field::new("guard", "guard", "Option Condition", None),
                    Field::new("from_", "from_", "Option Zone", None),
                ],
            ),
            Constructor::new(
                "byCardClass",
                "ByCardClass",
                vec![
                    Field::new("onPermanent", "on_permanent", "Ability", None),
                    Field::new(
                        "onInstantOrSorcery",
                        "on_instant_or_sorcery",
                        "Ability",
                        None,
                    ),
                ],
            ),
        ],
        "CardFace" => vec![Constructor::new(
            "mk",
            "",
            vec![Field::new(
                "characteristics",
                "characteristics",
                "Characteristics",
                None,
            )],
        )],
        "SharedLineHalf" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("name", "name", "String", None),
                Field::new("cost", "cost", "Option ManaCost", Some("none")),
                Field::new("text", "text", "List Ability", Some("[]")),
            ],
        )],
        "LevelRange" => vec![
            Constructor::new(
                "between",
                "Between",
                vec![
                    Field::new("from_", "from_", "Nat", None),
                    Field::new("to", "to", "Nat", None),
                ],
            ),
            Constructor::new(
                "plus",
                "Plus",
                vec![Field::new("from_", "from_", "Nat", None)],
            ),
        ],
        "LevelBand" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("range", "range", "LevelRange", None),
                Field::new("text", "text", "List Ability", Some("[]")),
                Field::new("power", "power", "Option Amount", Some("none")),
                Field::new("toughness", "toughness", "Option Amount", Some("none")),
            ],
        )],
        "PrototypeFrame" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("cost", "cost", "Option ManaCost", Some("none")),
                Field::new("power", "power", "Option Amount", Some("none")),
                Field::new("toughness", "toughness", "Option Amount", Some("none")),
            ],
        )],
        "Card" => vec![
            Constructor::new(
                "singleFaced",
                "SingleFaced",
                vec![Field::new("face", "face", "CardFace", None)],
            ),
            Constructor::new(
                "transforming",
                "Transforming",
                vec![
                    Field::new("front", "front", "CardFace", None),
                    Field::new("back", "back", "CardFace", None),
                ],
            ),
            Constructor::new(
                "modalDfc",
                "ModalDfc",
                vec![
                    Field::new("front", "front", "CardFace", None),
                    Field::new("back", "back", "CardFace", None),
                ],
            ),
            Constructor::new(
                "split",
                "Split",
                vec![
                    Field::new("left", "left", "CardFace", None),
                    Field::new("right", "right", "CardFace", None),
                ],
            ),
            Constructor::new(
                "sharedLineSplit",
                "SharedLineSplit",
                vec![
                    Field::new("shared", "shared", "CardFace", None),
                    Field::new("left", "left", "SharedLineHalf", None),
                    Field::new("right", "right", "SharedLineHalf", None),
                ],
            ),
            Constructor::new(
                "adventurer",
                "Adventurer",
                vec![
                    Field::new("normal", "normal", "CardFace", None),
                    Field::new("adventure", "adventure", "CardFace", None),
                ],
            ),
            Constructor::new(
                "flip",
                "Flip",
                vec![
                    Field::new("normal", "normal", "CardFace", None),
                    Field::new("alternative", "alternative", "CardFace", None),
                ],
            ),
            Constructor::new(
                "leveler",
                "Leveler",
                vec![
                    Field::new("inner", "inner", "CardFace", None),
                    Field::new("bands", "bands", "List LevelBand", None),
                ],
            ),
            Constructor::new(
                "prototype",
                "Prototype",
                vec![
                    Field::new("inner", "inner", "CardFace", None),
                    Field::new("alternative", "alternative", "PrototypeFrame", None),
                ],
            ),
        ],
        "SbaRule" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("scope", "scope", "Predicate", None),
                Field::new("when", "when", "Condition", None),
                Field::new("then_", "then_", "Action", None),
            ],
        )],
        "PredefinedToken" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("name", "name", "String", None),
                Field::new("token", "token", "CharacteristicBundle", None),
            ],
        )],
        "Conferral" => vec![
            Constructor::new(
                "ability",
                "Ability",
                vec![Field::new("confer", "confer", "Ability", None)],
            ),
            Constructor::new(
                "property",
                "Property",
                vec![Field::new("spec", "spec", "StaticSpec", None)],
            ),
            Constructor::new(
                "stateBased",
                "StateBased",
                vec![
                    Field::new("when", "when", "Condition", None),
                    Field::new("then_", "then_", "Action", None),
                ],
            ),
            Constructor::new(
                "turnBased",
                "TurnBased",
                vec![
                    Field::new("part", "part", "TurnPart", None),
                    Field::new("then_", "then_", "Action", None),
                ],
            ),
        ],
        "ConferralRule" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("scope", "scope", "Predicate", None),
                Field::new("confer", "confer", "Conferral", None),
            ],
        )],
        "RegistryDefinition" => vec![
            Constructor::new(
                "counter",
                "Counter",
                vec![
                    Field::new("kind", "kind", "CounterKind", None),
                    Field::new("confers", "confers", "List Conferral", None),
                ],
            ),
            Constructor::new(
                "subtype",
                "Subtype",
                vec![
                    Field::new("subtype", "subtype", "Subtype", None),
                    Field::new("rules", "rules", "List Conferral", None),
                ],
            ),
            Constructor::new(
                "designation",
                "Designation",
                vec![
                    Field::new("label", "label", "DesignationLabel", None),
                    Field::new("confers", "confers", "List Conferral", None),
                ],
            ),
            Constructor::new(
                "cardType",
                "CardType",
                vec![
                    Field::new("cardType", "card_type", "CardType", None),
                    Field::new("confers", "confers", "List Conferral", None),
                ],
            ),
            Constructor::new(
                "relation",
                "Relation",
                vec![Field::new("label", "label", "RelationLabel", None)],
            ),
        ],
        "RulesTables" => vec![Constructor::new(
            "mk",
            "",
            vec![
                Field::new("sba", "sba", "List SbaRule", Some("[]")),
                Field::new("conferral", "conferral", "List ConferralRule", Some("[]")),
            ],
        )],
        _ => return None,
    })
}

pub(super) fn alias(ty: &str) -> Option<&'static str> {
    match ty {
        "FlavorWordLabel" => Some("String"),
        "VoteLabel" => Some("String"),
        "KeywordActionLabel" => Some("String"),
        "BindingId" => Some("String"),
        "ManaCost" => Some("List ManaSymbol"),
        "ProducedRun" => Some("List ColorOrColorless"),
        "ChapterNumber" => Some("Nat"),
        _ => None,
    }
}
