import SylFixtures
import SylGen.ReadOrder
import Semantics.Check.Phrase
import Lean

open Lean Meta Elab Command

private def swapMentions (identity : String) (value : Expr) : Expr :=
  value.replace fun e =>
    if e.isAppOfArity `Semantics.NounPhrase.firstMention 2 &&
        e.getAppArgs[0]! == mkStrLit identity then
      some (mkAppN (mkConst `Semantics.NounPhrase.laterMention) e.getAppArgs)
    else if e.isAppOfArity `Semantics.NounPhrase.laterMention 2 &&
        e.getAppArgs[0]! == mkStrLit identity then
      some (mkAppN (mkConst `Semantics.NounPhrase.firstMention) e.getAppArgs)
    else none

-- Check the generated constructor tree, then flip only the probe parameter's mention wrappers.
-- The mutation must fail for its binding identity, not merely for an unrelated shape error.
elab "check_read_order " declaration:ident " on " argument:term " via " checker:term : command => do
  let name ← resolveGlobalConstNoOverload declaration
  let identity := name.getString! ++ ".x"
  let goal ← liftTermElabM do
    let argument ← Term.elabTerm argument (some (mkConst `Semantics.NounPhrase))
    let checker ← Term.elabTerm checker none
    Term.synthesizeSyntheticMVarsNoPostponing
    let value ← withTransparency .all <| reduce (mkApp (mkConst name) argument) false
    let swapped := swapMentions identity value
    if swapped == value then throwError "probe contains no generated mentions for {identity}"
    let good := mkApp checker value
    let bad := mkApp checker swapped
    let empty ← mkAppOptM ``List.nil #[some (mkConst `Semantics.Refusal)]
    let refusal := mkApp (mkConst `Semantics.Refusal.bindingIdentity) (mkStrLit identity)
    let rejected ← mkAppM ``List.contains #[bad, refusal]
    mkAppM ``And #[← mkEq good empty, ← mkEq rejected (mkConst ``Bool.true)]
  let goal ← liftTermElabM <| PrettyPrinter.delab goal
  let theoremName := mkIdent ((name.getString! ++ "ChecksAndRejectsSwap").toName)
  elabCommand (← `(theorem $theoremName : $goal := by native_decide))

open Semantics Semantics.Templates

private def creatureProbe : NounPhrase := .described (.a .unmarked) isCreature
private def groupProbe : NounPhrase := .described .all isCreature
private def playerProbe : NounPhrase := .described (.a .unmarked) isPlayer
private def actionErrors (value : Action) := Action.check (DRS.ofList []) value
private def abilityErrors (value : Ability) := Ability.check (DRS.ofList []) value
private def staticErrors (value : StaticSpec) := (stepStaticSpec 64 (DRS.ofList []) value).2

check_read_order SylFixture.Semantics.Templates.parallelAlternatives on creatureProbe
  via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelJoinedHeaders on creatureProbe
  via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelHeaderModifiers on playerProbe
  via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelActivatedModifiers on you via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelTriggerLimit on groupProbe via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelDeonticException on thisCreature
  via staticErrors
check_read_order SylFixture.Semantics.Templates.parallelQualified on thisCreature via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelCardClasses on creatureProbe
  via abilityErrors
check_read_order SylFixture.Semantics.Templates.parallelDelayedAlternatives on creatureProbe
  via actionErrors

private def characteristicErrors (value : Characteristics) :=
  (stepCharacteristicsWith (stepOps 64) (DRS.ofList []) value).2
private def tokenErrors (value : TokenSpec) :=
  (stepTokenSpecMetadataWith (stepOps 64) (DRS.ofList []) value).2.2
check_read_order SylFixture.Semantics.Templates.parallelCharacteristics on groupProbe
  via characteristicErrors
check_read_order SylFixture.Semantics.Templates.parallelFaces on groupProbe via tokenErrors
check_read_order SylFixture.Semantics.Templates.parallelModes on playerProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.parallelGrantedAbilities on creatureProbe
  via staticErrors

check_read_order SylFixture.Semantics.Templates.orderPrevent on groupProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.orderRedirect on creatureProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.orderVote on playerProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.orderRemoveCounters on groupProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.orderMoveCounters on groupProbe via actionErrors
check_read_order SylFixture.Semantics.Templates.orderPay on playerProbe via actionErrors

/-- Every field-order exception has a checker-backed positive and swapped-mention probe above. -/
example : SylGen.ReadOrder.fieldOrders.map Prod.fst = #[
    `Semantics.Action.preventDamage, `Semantics.Action.redirectDamage, `Semantics.Action.vote,
    `Semantics.Action.removeCounters, `Semantics.Action.moveCounters, `Semantics.Action.pay] := by
  native_decide

private def eventErrors (value : GameEvent) := (stepGameEvent 64 (DRS.ofList []) value).2
check_read_order SylFixture.Semantics.Templates.orderActivator on playerProbe via abilityErrors
check_read_order SylFixture.Semantics.Templates.orderTriggerBodyTiming on creatureProbe
  via abilityErrors
check_read_order SylFixture.Semantics.Templates.orderModifiers on you via staticErrors
check_read_order SylFixture.Semantics.Templates.orderCause on creatureProbe via staticErrors
check_read_order SylFixture.Semantics.Templates.orderGatedCost on playerProbe via staticErrors
check_read_order SylFixture.Semantics.Templates.orderPassive on creatureProbe via eventErrors
check_read_order SylFixture.Semantics.Templates.orderConjunction on creatureProbe via staticErrors

private def predicateErrors (value : Predicate) := (stepPredicate 64 (DRS.ofList []) value).2
check_read_order SylFixture.Semantics.Templates.parallelComparison on groupProbe via predicateErrors
private def copyExceptionErrors (value : List CopyExcept) :=
  (stepCopyExceptListWith (stepOps 64) (DRS.ofList []) value).2
check_read_order SylFixture.Semantics.Templates.parallelCopyExceptions on groupProbe
  via copyExceptionErrors
check_read_order SylFixture.Semantics.Templates.parallelKeywordParameters on playerProbe
  via abilityErrors
