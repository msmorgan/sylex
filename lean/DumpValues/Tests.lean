import DumpValues.Sampling

open Lean Meta DumpValues

private def require (condition : Bool) (message : String) : MetaM Unit :=
  unless condition do throwError "{message}"

private structure FieldDefaults where
  flag : Bool := true
  number : Nat := 9

run_meta do
  let defaults ← base (mkConst ``FieldDefaults) 0
  require (← isDefEq defaults (mkAppN (mkConst ``FieldDefaults.mk)
    #[mkConst ``Bool.true, mkNatLit 9])) "structure defaults lost"
  let location ← construct (mkConst ``Semantics.Location) ``Semantics.Location.fromEnd 0 []
  require (← isDefEq location.getAppArgs[1]!
    (mkApp (mkConst ``Semantics.Ordinal.nth) (mkNatLit 1))) "constructor default lost"
  let (xs, omitted) ← ofExcept (combinations #[20, 20])
  require (xs.size == 256 && omitted == 144) "cap or omitted count"
  for i in [:20] do
    require (xs.contains #[i, 0] && xs.contains #[0, i]) "individual variant lost"
  require (match combinations #[258] with | .error _ => true | .ok _ => false)
    "overfull basis must fail explicitly"
  require ((← variants (mkConst ``Bool) 0).size == 2) "Bool basis"
  let optional ← mkAppM ``Option #[mkConst ``Bool]
  require ((← variants optional 0).size == 3) "Option Bool basis"
  let list ← mkAppM ``List #[mkConst ``Bool]
  require ((← variants list 0).size == 4) "List Bool basis"
  for type in [``Semantics.Action, ``Semantics.Amount] do
    let values ← variants (mkConst type) 0
    for ctor in (← getConstInfoInduct type).ctors do
      require (values.any (·.getAppFn.isConstOf ctor)) s!"missing constructor {ctor}"
    for value in values do discard <| render value
  let ns ← mkAppOptM ``Option.none #[some (mkConst ``Nat)]
  let ss ← mkAppOptM ``Option.none #[some (mkConst ``String)]
  require ((← render ns) != (← render ss)) "erased type parameter"
  try
    discard <| render (mkConst ``Nat.add)
    throwError "printer accepted a function"
  catch e =>
    require ((← e.toMessageData.toString).startsWith "residual constant") "wrong refusal"
