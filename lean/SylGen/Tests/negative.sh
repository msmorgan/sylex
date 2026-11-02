#!/bin/sh
set -eu
exe=$1
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir "$scratch/input"
check() {
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
check 'redundant constructor binding' 'primitive white: Color = .white'
check 'primitive must name a constructor' 'primitive missing: Color = .nonexistent'
check "expected '.constructor'" 'primitive colorWhite: Color = white'
check "dotted word '.Color.white'" 'primitive colorWhite: Color = .Color.white'
check 'primitive must bind every constructor field' 'primitive actionSearch(scope: Location): Action = .search'
check 'primitive parameter does not match' 'primitive actionSearch(scope: Location, wrong: NounPhrase): Action = .search'
check 'Duplicate bundle declaration' 'template item: Nat { body = 1 } template item: Nat { body = 2 }'
check "parameter '\$x' has sort" 'template mismatch(x: Color): Nat { body = $x }'
check 'unsupported string escape' 'template bad: String { body = "\n" }'
check 'unknown word' 'template missing: Color { body = nonexistent }'
check 'negative integer at Nat' 'template negative: Nat { body = -1 }'
check 'empty call' 'template empty: Color { body = blue() }'
check 'missing argument' 'template missing: Amount { body = lit }'
check 'duplicate field' 'template duplicate: Nat { body = 1, body = 2 }'
check "duplicate argument 'a'" 'template pair(a: Nat, b: Nat): Nat { body = $a } template twice: Nat { body = pair(a = 1, a = 2) }'
check "duplicate argument 'a'" 'template pair(a: Nat, b: Nat): Nat { body = $a } template filled: Nat { body = pair(1, a = 2) }'
check 'too many arguments' 'template pair(a: Nat, b: Nat): Nat { body = $a } template over: Nat { body = pair(b = 1, 2, 3) }'
check 'reserved parameter name' 'template bad(none: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(true: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(false: Int): Int { body = 1 }'
check 'unknown word' 'action bad { deed = foo(1), body = sequentially([]) }'
check 'none requires Option' 'action bad { deed = none(1), body = sequentially([]) }'
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
check 'ofAbility cannot be combined with deed or permanent' 'action bad { ofAbility = foo, deed = other, body = sequentially([]) }'
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
check 'firstMention is reserved for generated parameter identities' 'template noWrittenFirst(subject: NounPhrase): NounPhrase { body = firstMention("manual", $subject) }'
check 'firstMention is reserved for generated parameter identities' 'primitive noFirstAlias(id: String, body: NounPhrase): NounPhrase = .firstMention'
check 'raw DesignationLabel construction requires a registry declaration' 'template noDesignation: DesignationLabel { body = named("typo", heldByGame) }'
check 'raw RelationLabel construction requires a registry declaration' 'template noRelation: RelationLabel { body = named("typo") }'
check 'raw Subtype construction requires a registry declaration' 'template noSubtype: Subtype { body = of(creature, "Typo") }'
check 'raw Subtype construction requires a registry declaration' 'template noSpellSubtype: Subtype { body = spell("Typo") }'
check 'raw CardType construction requires a registry declaration' 'template noCardType: CardType { body = mk("Typo", none, []) }'
check 'raw CardType construction requires a registry declaration' 'template noCardTypeRecord: CardType { body = { name = "Typo" } }'
check 'raw RegistryDefinition construction requires a registry declaration' 'template noRegistry: RegistryDefinition { body = relation(named("typo")) }'
check 'Action.inCaller is reserved for generated caller scope' 'template noActionCaller(value: Action): Action { body = inCaller($value) }'
check 'Cost.inCaller is reserved for generated caller scope' 'template noCostCaller(value: Cost): Cost { body = inCaller($value) }'
check 'StaticSpec.inCaller is reserved for generated caller scope' 'template noStaticSpecCaller(value: StaticSpec): StaticSpec { body = inCaller($value) }'
check 'Replacement.inCaller is reserved for generated caller scope' 'template noReplacementCaller(value: Replacement): Replacement { body = inCaller($value) }'
check 'Amount.inCaller is reserved for generated caller scope' 'template noAmountCaller(value: Amount): Amount { body = inCaller($value) }'
check 'Predicate.inCaller is reserved for generated caller scope' 'template noPredicateCaller(value: Predicate): Predicate { body = inCaller($value) }'
check 'Quantity.inCaller is reserved for generated caller scope' 'template noQuantityCaller(value: Quantity): Quantity { body = inCaller($value) }'
check 'Location.inCaller is reserved for generated caller scope' 'template noLocationCaller(value: Location): Location { body = inCaller($value) }'
check 'Condition.inCaller is reserved for generated caller scope' 'template noConditionCaller(value: Condition): Condition { body = inCaller($value) }'
check 'GameEvent.inCaller is reserved for generated caller scope' 'template noGameEventCaller(value: GameEvent): GameEvent { body = inCaller($value) }'
check 'Ability.inCaller is reserved for generated caller scope' 'template noAbilityCaller(value: Ability): Ability { body = inCaller($value) }'
check 'TokenSpec.inCaller is reserved for generated caller scope' 'template noTokenSpecCaller(value: TokenSpec): TokenSpec { body = inCaller($value) }'
check 'MoveRider.inCaller is reserved for generated caller scope' 'template noMoveRiderCaller(value: MoveRider): MoveRider { body = inCaller($value) }'
check 'raw KeywordLabel construction requires a registry declaration' 'template noRawKeywordLabel: KeywordLabel { body = named("flyign") }'
check 'raw CounterKind construction requires a registry declaration' 'template noRawCounterKind: CounterKind { body = named("loyatlyCounter") }'
check 'raw KeywordLabel construction requires a registry declaration' 'primitive noKeywordLabelAlias(name: String): KeywordLabel = .named'
check 'raw CounterKind construction requires a registry declaration' 'primitive noCounterKindAlias(name: String, holder: Kind): CounterKind = .named'
check 'laterMention is reserved for generated parameter identities' 'template noWrittenLater(subject: NounPhrase): NounPhrase { body = laterMention("manual", $subject) }'
check 'Action.ownScope is reserved for generated scope and deeds' 'template noWrittenScope: Action { body = ownScope(none, sequentially([])) }'
check 'NounPhrase.inCaller is reserved for generated scope and deeds' 'template noNounCaller(subject: NounPhrase): NounPhrase { body = inCaller($subject) }'
check 'Deed.action is reserved for generated scope and deeds' 'template noRawDeed: Deed { body = action("discard") }'
check 'raw Ability.keyword construction requires an ability declaration' 'ability flying { body = [] } template noEmptyKeyword: Ability { body = keyword(flying, [], []) }'
check 'Action.ownScope is reserved for generated scope and deeds' 'template theRingTemptsYou: Action { body = ownScope(none, sequentially([])) }'
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
printf 'sylGen negative cases: 99 passed\n'

printf '%s\n' 'template shadowed(white: Color): Color { body = white }' > "$scratch/input/bad.syl"
"$exe" "$scratch/input" "$scratch/shadow-output" "" Shadow > "$scratch/log" 2>&1
grep -F ': Semantics.«Color» := («Semantics».«Color».«white» )' "$scratch/shadow-output/Shadow/"*.lean >/dev/null
printf 'sylGen bare word beside a same-named parameter: passed\n'

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
printf '%s\n' 'action theRingTemptsYou { body = ownScope(action("theRingTemptsYou"), sequentially([])) }' > "$scratch/corpus/builtin/definitions/keyword_action/theRingTemptsYou.syl"
(
  cd "$scratch/elsewhere"
  "$exe" "$scratch/corpus" "$scratch/ring-output" RingFixture Ring.Generated > "$scratch/log" 2>&1
)
test -f "$scratch/ring-output/Ring/Generated.lean"
printf 'sylGen corpus-relative Ring exception: passed\n'
