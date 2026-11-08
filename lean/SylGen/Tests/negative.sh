#!/bin/sh
set -eu
exe=$1
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir "$scratch/input"
passed=0
check() {
  passed=$((passed + 1))
  printf '%s\n' "$2" > "$scratch/input/bad.syl"
  if "$exe" "$scratch/input" "$scratch/out" Negative > "$scratch/log" 2>&1; then
    echo "unexpected success: $1" >&2
    exit 1
  fi
  if ! grep -F "$1" "$scratch/log" >/dev/null ||
     ! grep -E 'bad.syl:[0-9]+:[0-9]+:' "$scratch/log" >/dev/null; then
    cat "$scratch/log" >&2
    exit 1
  fi
}
check 'expected expression' 'template dotted: Color { body = .white }'
check 'expected expression' 'template dotted: Nat { lexical = { word = .white }, body = 1 }'
check 'expected expression' 'primitive dotted(value: Color = .white): Color = .white'
check 'only a primitive may name a constructor' 'template white: Color = .white'
check 'only a primitive may name a constructor' 'action search = .search'
check 'expected a capitalized constructor' 'primitive white: Color = .white'
check 'primitive must name a constructor' 'primitive missing: Color = Nonexistent'
check 'expected a capitalized constructor' 'primitive colorWhite: Color = white'
check 'primitive requires a constructor clause' 'primitive power: Stat'
check "dotted word '.Color.white'" 'primitive colorWhite: Color = .Color.white'
check 'primitive must bind every constructor field' 'primitive actionSearch(scope: Location): Action = Search'
check 'primitive parameter does not match' 'primitive actionSearch(scope: Location, wrong: NounPhrase): Action = Search'
check 'Duplicate bundle declaration' 'template item: Nat { body = 1 } template item: Nat { body = 2 }'
check "parameter '\$x' has sort" 'template mismatch(x: Color): Nat { body = $x }'
check 'unsupported string escape' 'template bad: String { body = "\n" }'
check 'unknown word' 'template missing: Color { body = nonexistent }'
check 'negative integer at Nat' 'template negative: Nat { body = -1 }'
check 'empty call' 'template empty: Color { body = blue() }'
check 'missing argument' 'primitive lit(value: Int): Amount = Literal template missing: Amount { body = lit }'
check 'duplicate field' 'template duplicate: Nat { body = 1, body = 2 }'
check "duplicate argument 'a'" 'template pair(a: Nat, b: Nat): Nat { body = $a } template twice: Nat { body = pair(a = 1, a = 2) }'
check "duplicate argument 'a'" 'template pair(a: Nat, b: Nat): Nat { body = $a } template filled: Nat { body = pair(1, a = 2) }'
check 'too many arguments' 'template pair(a: Nat, b: Nat): Nat { body = $a } template over: Nat { body = pair(b = 1, 2, 3) }'
check 'reserved parameter name' 'template bad(none: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(true: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(false: Int): Int { body = 1 }'
check "unknown word 'foo'" 'primitive sequentially(steps: List(Action) = []): Action = Sequentially action bad { deed = foo(1), body = sequentially([]) }'
check 'none requires Option' 'primitive sequentially(steps: List(Action) = []): Action = Sequentially action bad { deed = none(1), body = sequentially([]) }'
check "unknown word 'customWord' at String" 'primitive sequentially(steps: List(Action) = []): Action = Sequentially action bad { deed = customWord, body = sequentially([]) }'
check 'Dependency cycle in bundle' 'template a: Int { body = b } template b: Int { body = a }'
check "dotted word 'Int.b'" 'template a: Int { body = Int.b } template b: Int { body = 1 }'
check "non-canonical integer '007'" 'template leadingZero: Int { body = 007 }'
check "non-canonical integer '-01'" 'template negativeLeadingZero: Int { body = -01 }'
check "non-canonical integer '00'" 'template doubleZero: Int { body = 00 }'
check "non-canonical integer '-0'" 'template negativeZero: Int { body = -0 }'
check "invalid integer '1_000'" 'template separated: Int { body = 1_000 }'
check "integer out of range '9223372036854775808'" 'template tooLarge: Int { body = 9223372036854775808 }'
check "integer out of range '-9223372036854775809'" 'template tooSmall: Int { body = -9223372036854775809 }'
check 'Dependency cycle in bundle' 'ability mutualOne { body = [mutualTwo] } ability mutualTwo { body = [mutualOne] }'
check 'ofAbility cannot be combined with deed or permanent' 'primitive sequentially(steps: List(Action) = []): Action = Sequentially action bad { ofAbility = foo, deed = other, body = sequentially([]) }'
check "parameter '\$power' has sort" 'template wrongSortParameterIsRefused(power: Amount): Stat { body = $power }'
check "parameter '\$who' has sort" 'template noNounSubject(who: NounPhrase): Subject { body = $who }'
check "unknown word 'draw' at Cost" 'template noActionCost: Cost { body = draw(exactlyOf(1)) }'
check 'list at Action' 'template noListAction: Action { body = [] }'
check "unknown word 'nonexistent' at KeywordLabel" 'template noUnknownKeyword: KeywordLabel { body = nonexistent }'
check "unknown word 'if' at Int" 'template noEvaluation: Int { body = if(true, 1, 2) }'
check "parameter '\$value' has sort" 'template noSubtypePredicate(value: Subtype): Predicate { body = $value }'
check "parameter '\$value' has sort" 'template noAbilityTerm(value: Ability): KeywordTerm { body = $value }'
check "parameter '\$value' has sort" 'template noManaCost(value: List(ManaSymbol)): Cost { body = $value }'
check "parameter '\$value' has sort" 'template noActionCostParameter(value: Action): Cost { body = $value }'
check "parameter '\$value' has sort" 'template noWrittenToken(value: CharacteristicBundle): TokenSpec { body = $value }'
check "parameter '\$value' has sort" 'template noDurationSpan(value: Duration): LookbackSpan { body = $value }'
check "parameter '\$value' has sort" 'template noCounterSource(value: CounterKind): CounterKindSource { body = $value }'
check "parameter '\$value' has sort" 'template noNameSource(value: String): NameSource { body = $value }'
check "parameter '\$value' has sort" 'template noConditionAsThough(value: Condition): AsThough { body = $value }'
check "parameter '\$value' has sort" 'template noStaticAbility(value: StaticSpec): Ability { body = $value }'
check "parameter '\$value' has sort" 'template noCharacteristicBundle(value: Characteristics): CharacteristicBundle { body = $value }'
check "parameter '\$value' has sort" 'template noActionMode(value: Action): Mode { body = $value }'
check "parameter '\$value' has sort" 'template noReplacementAction(value: Replacement): Action { body = $value }'
check "parameter '\$value' has sort" 'template noActionReplacement(value: Action): Replacement { body = $value }'
check 'firstMention is reserved for generated parameter identities' 'primitive noFirstAlias(id: String, body: NounPhrase): NounPhrase = FirstMention'
check 'raw DesignationLabel construction requires a registry declaration' 'primitive noDesignation: DesignationLabel = Named'
check 'raw RelationLabel construction requires a registry declaration' 'primitive noRelation: RelationLabel = Named'
check 'raw Subtype construction requires a registry declaration' 'primitive noSubtype: Subtype = Of'
check 'raw Subtype construction requires a registry declaration' 'primitive noSpellSubtype: Subtype = Spell'
check 'raw CardType construction requires a registry declaration' 'primitive noCardType: CardType = Mk'
check 'raw CardType construction requires a registry declaration' 'template noCardTypeRecord: CardType { body = { name = "Typo" } }'
check 'raw RegistryDefinition construction requires a registry declaration' 'primitive noRegistry: RegistryDefinition = Relation'
check 'Action.inCaller is reserved for generated caller scope' 'primitive noActionCaller: Action = InCaller'
check 'Cost.inCaller is reserved for generated caller scope' 'primitive noCostCaller: Cost = InCaller'
check 'StaticSpec.inCaller is reserved for generated caller scope' 'primitive noStaticSpecCaller: StaticSpec = InCaller'
check 'Replacement.inCaller is reserved for generated caller scope' 'primitive noReplacementCaller: Replacement = InCaller'
check 'Amount.inCaller is reserved for generated caller scope' 'primitive noAmountCaller: Amount = InCaller'
check 'Predicate.inCaller is reserved for generated caller scope' 'primitive noPredicateCaller: Predicate = InCaller'
check 'Quantity.inCaller is reserved for generated caller scope' 'primitive noQuantityCaller: Quantity = InCaller'
check 'Location.inCaller is reserved for generated caller scope' 'primitive noLocationCaller: Location = InCaller'
check 'Condition.inCaller is reserved for generated caller scope' 'primitive noConditionCaller: Condition = InCaller'
check 'GameEvent.inCaller is reserved for generated caller scope' 'primitive noGameEventCaller: GameEvent = InCaller'
check 'Ability.inCaller is reserved for generated caller scope' 'primitive noAbilityCaller: Ability = InCaller'
check 'TokenSpec.inCaller is reserved for generated caller scope' 'primitive noTokenSpecCaller: TokenSpec = InCaller'
check 'MoveRider.inCaller is reserved for generated caller scope' 'primitive noMoveRiderCaller: MoveRider = InCaller'
check 'raw KeywordLabel construction requires a registry declaration' 'primitive noRawKeywordLabel: KeywordLabel = Named'
check 'raw CounterKind construction requires a registry declaration' 'primitive noRawCounterKind: CounterKind = Named'
check 'raw KeywordLabel construction requires a registry declaration' 'primitive noKeywordLabelAlias(name: String): KeywordLabel = Named'
check 'raw CounterKind construction requires a registry declaration' 'primitive noCounterKindAlias(name: String, holder: Kind): CounterKind = Named'
check 'laterMention is reserved for generated parameter identities' 'primitive noWrittenLater: NounPhrase = LaterMention'
check 'Action.ownScope is reserved for generated scope and deeds' 'primitive noWrittenScope: Action = OwnScope'
check 'NounPhrase.inCaller is reserved for generated scope and deeds' 'primitive noNounCaller: NounPhrase = InCaller'
check 'Deed.action is reserved for generated scope and deeds' 'primitive noRawDeed: Deed = Action'
check 'raw Ability.keyword construction requires an ability declaration' 'primitive noEmptyKeyword: Ability = Keyword'
check 'Action.ownScope is reserved for generated scope and deeds' 'primitive ownScope(deed: Option(Deed), body: Action): Action = OwnScope'
mv "$scratch/input/bad.syl" "$scratch/input/theRingTemptsYou.syl"
if "$exe" "$scratch/input" "$scratch/out" Negative > "$scratch/log" 2>&1; then
  echo 'unexpected success: copied Ring filename' >&2
  exit 1
fi
grep -F 'Action.ownScope is reserved for generated scope and deeds' "$scratch/log" >/dev/null
rm "$scratch/input/theRingTemptsYou.syl"
check 'identity requires a parameterless NounPhrase template and a nonempty string' 'template bad(value: NounPhrase): NounPhrase { identity = "actor", body = $value }'
check 'identity requires a parameterless NounPhrase template and a nonempty string' 'template bad: Color { identity = "actor", body = white }'
check 'identity requires a parameterless NounPhrase template and a nonempty string' 'template bad: NounPhrase { identity = "", body = actor }'
check 'identity must be a string' 'template bad: NounPhrase { identity = actor, body = actor }'
check "unknown parameter '\$missing'" 'template unknownParameter(amount: Int): Int { body = $missing }'
check "unknown parameter '\$later'" 'template laterParameter(early: Int = $later, later: Int): Int { body = $early }'
check "parameter '\$x' has sort Color, expected Option(Nat)" 'template optionMismatch(x: Color): Option(Nat) { body = $x }'
check 'a parameter reference takes no arguments' 'template parameterCall(x: Int): Int { body = $x(1) }'
check "'\$' must be directly followed by a parameter name" 'template loneSigil: Int { body = $ }'
check "'\$' must be directly followed by a parameter name" 'template spacedSigil(x: Int): Int { body = $ x }'
check "dotted word '\$x.y'" 'template dottedParameter(x: Int): Int { body = $x.y }'
check "unknown word 'amount' at Nat" 'template bareParameter(amount: Nat): Nat { body = amount }'
check "unknown word 'power' at Stat" 'template noConstructorWord: Stat { body = power }'
check 'integer at Amount' 'template noIntegerAmount: Amount { body = 3 }'
check 'integer at ManaSymbol' 'template noIntegerMana: List(ManaSymbol) { body = [2] }'
check "unknown word 'blue' at ManaSymbol" 'primitive blue: Color = Blue template noColorMana: List(ManaSymbol) { body = [blue] }'
check "unknown word 'blue' at ColorOrColorless" 'primitive blue: Color = Blue template noColorOrColorless: ColorOrColorless { body = blue }'
check "unknown word 'kicker' at KeywordTerm" 'ability kicker { body = [] } template noKeywordTerm: KeywordTerm { body = kicker }'
printf 'sylGen negative cases: %s passed\n' "$passed"

printf '%s\n' 'primitive white: Color = White template shadowed(white: Color): Color { body = white }' > "$scratch/input/bad.syl"
"$exe" "$scratch/input" "$scratch/shadow-output" "" Shadow > "$scratch/log" 2>&1
grep -F ': Semantics.«Color» := («Semantics».«Templates».«white» )' "$scratch/shadow-output/Shadow/"*.lean >/dev/null
printf 'sylGen bare word beside a same-named parameter: passed\n'

printf '%s\n' 'primitive lit(value: Int): Amount = Literal template keywordBare: Amount { body = lit(1) }' > "$scratch/input/bad.syl"
if "$exe" "$scratch/input" "$scratch/collision-output" "" Collision > "$scratch/log" 2>&1; then
  echo 'unexpected success: generated name colliding with a hand-written constant' >&2
  exit 1
fi
grep -F 'template keywordBare generates Semantics.Templates.keywordBare, which Semantics.Primitives already declares' "$scratch/log" >/dev/null
printf 'sylGen generated name colliding with a hand-written constant: refused\n'

printf '%s\n' 'template value: Int { body = 1 } template use: Int { body = value }' > "$scratch/input/bad.syl"
"$exe" "$scratch/input" "$scratch/custom-output" CustomRoot Custom.Generated > "$scratch/log" 2>&1
test -f "$scratch/custom-output/Custom/Generated.lean"
grep -F 'import Custom.Generated.' "$scratch/custom-output/Custom/Generated.lean" >/dev/null
grep -F '«CustomRoot».«Semantics».«Templates»' "$scratch/custom-output/Custom/Generated/"*.lean >/dev/null
printf 'sylGen configurable output: passed\n'

printf '%s\n' 'card { name = "Bad Creature", types = [creature] }' > "$scratch/input/bad.syl"
if ! "$exe" --fixture "$scratch/input" "$scratch/bad-output" BadFixture BadCards > "$scratch/log" 2>&1; then
  cat "$scratch/log" >&2
  exit 1
fi
export LEAN_PATH="$scratch/bad-output:$LEAN_PATH"
(
  cd "$scratch/bad-output"
  for data in BadCards/*.lean; do
    if grep -F 'check_card ' "$data" >/dev/null; then
      echo 'card check leaked into data module' >&2
      exit 1
    fi
    if ! lean -o "${data%.lean}.olean" "$data" > "$scratch/data.log" 2>&1; then
      cat "$scratch/data.log" >&2
      exit 1
    fi
  done
  for check in BadCards/Check/*.lean; do
    if lean "$check" > "$scratch/check.log" 2>&1; then
      echo 'invalid card check unexpectedly passed' >&2
      exit 1
    fi
    if ! grep -F 'Card semantic check failed:' "$scratch/check.log" >/dev/null; then
      cat "$scratch/check.log" >&2
      exit 1
    fi
  done
)
printf 'sylGen separate card refusal report: passed\n'

mkdir -p "$scratch/corpus/builtin/definitions/keyword_action" "$scratch/elsewhere"
ring='primitive ownScope(deed: Option(Deed), body: Action): Action = OwnScope
primitive sequentially(steps: List(Action) = []): Action = Sequentially
action theRingTemptsYou { body = ownScope(theRingTemptsYou, sequentially([])) }'
printf '%s\n' "$ring" > "$scratch/corpus/builtin/definitions/keyword_action/theRingTemptsYou.syl"
(
  cd "$scratch/elsewhere"
  "$exe" "$scratch/corpus" "$scratch/ring-output" RingFixture Ring.Generated > "$scratch/log" 2>&1
)
test -f "$scratch/ring-output/Ring/Generated.lean"
printf 'sylGen corpus-relative Ring exception: passed\n'

printf '%s\n' 'template elsewhere: Action { body = ownScope(none, sequentially([])) }' > "$scratch/corpus/builtin/elsewhere.syl"
if "$exe" "$scratch/corpus" "$scratch/ring-use-output" RingFixture Ring.Generated > "$scratch/log" 2>&1; then
  echo "unexpected success: the Ring's ownScope primitive used elsewhere" >&2
  exit 1
fi
grep -F 'elsewhere.syl:1:' "$scratch/log" | grep -F 'Action.ownScope is reserved for generated scope and deeds' >/dev/null
printf "sylGen Ring's ownScope primitive refused outside the Ring's file: passed\n"
