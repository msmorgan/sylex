import Semantics.Authoring

open Lean Meta

namespace DumpValues

private partial def renderType (type : Expr) : MetaM String := do
  let type ← withTransparency .all <| whnf type
  if let .sort level := type then return if level.isZero then "Prop" else "Type"
  let .const name _ := type.getAppFn | throwError "unsupported type parameter: {type}"
  let args ← type.getAppArgs.mapM renderType
  return name.toString ++ if args.isEmpty then ""
    else "(" ++ String.intercalate "," args.toList ++ ")"

/-- A constructor-only encoding. Rejecting residual code prevents a definition name from
standing in for its meaning. Type parameters are included without universe or pretty-printer
noise. -/
partial def render (value : Expr) : MetaM String := do
  let value ← withTransparency .all <| whnf value
  if value.hasMVar || value.hasFVar then throwError "open value"
  if let some n := value.rawNatLit? then return toString n
  if let .lit (.strVal s) := value then return (Json.str s).compress
  let .const name _ := value.getAppFn | throwError "residual expression: {value}"
  let .ctorInfo info ← getConstInfo name | throwError "residual constant: {name}"
  let args := value.getAppArgs
  unless args.size == info.numParams + info.numFields do
    throwError "unsaturated constructor: {name}"
  let params ← (args.extract 0 info.numParams).mapM renderType
  let params := if params.isEmpty then ""
    else "{" ++ String.intercalate "," params.toList ++ "}"
  let fields ← (args.extract info.numParams args.size).mapM render
  return name.toString ++ params ++ "(" ++ String.intercalate "," fields.toList ++ ")"

private def constructors (ty : Expr) : MetaM (Array Name) := do
  let ty ← whnf ty
  let info ← getConstInfoInduct ty.getAppFn.constName!
  let cs ← info.ctors.toArray.mapM fun n => return (n, (← getConstInfoCtor n).numFields)
  return (cs.toList.mergeSort (fun a b => a.2 <= b.2)).toArray.map (·.1)

mutual
  /-- Find a finite inhabitant; cycles try another constructor, never silently omit a variant. -/
  partial def base (ty : Expr) (ordinal : Nat) (seen : List Expr := []) : MetaM Expr := do
    let ty ← whnf ty
    if ty.isConstOf ``Nat then return mkNatLit (ordinal + 3)
    if ty.isConstOf ``Int then return mkApp (mkConst ``Int.ofNat) (mkNatLit (ordinal + 3))
    if ty.isConstOf ``String then return mkStrLit s!"Sample {ordinal}"
    if ty.isConstOf ``Semantics.Amount then
      return mkApp (mkConst ``Semantics.Amount.lit)
        (mkApp (mkConst ``Int.ofNat) (mkNatLit (ordinal + 3)))
    if ty.isConstOf ``Semantics.NounPhrase then return mkConst ``Semantics.NounPhrase.you
    if ty.isConstOf ``Semantics.Cost then
      return mkApp (mkConst ``Semantics.Cost.mana)
        (← mkAppM ``List.cons #[mkApp (mkConst ``Semantics.ManaSymbol.simple)
          (mkApp (mkConst ``Semantics.SimpleManaSymbol.generic) (mkNatLit 2)),
          ← mkAppOptM ``List.nil #[some (mkConst ``Semantics.ManaSymbol)]])
    if seen.contains ty then throwError "no finite base for {ty}"
    for ctor in ← constructors ty do
      try return ← construct ty ctor ordinal (ty :: seen)
      catch _ => pure ()
    throwError "no finite base for {ty}"

  partial def construct (ty : Expr) (ctor : Name) (ordinal : Nat)
      (seen : List Expr) (replacement : Option (Nat × Expr) := none) : MetaM Expr := do
    let info ← getConstInfoCtor ctor
    let mut value := mkAppN (mkConst ctor ty.getAppFn.constLevels!)
      (ty.getAppArgs.extract 0 info.numParams)
    let mut fields : NameMap Expr := {}
    for i in [:info.numFields] do
      let .forallE name field _ _ ← whnf (← inferType value) | throwError "constructor field"
      let arg ← if let some (j, arg) := replacement then
          if i == j then pure arg else fieldBase ty name field fields ordinal seen
        else fieldBase ty name field fields ordinal seen
      fields := fields.insert name arg
      value := mkApp value arg
    return value

  partial def fieldBase (ty : Expr) (name : Name) (field : Expr) (fields : NameMap Expr)
      (ordinal : Nat) (seen : List Expr) : MetaM Expr := do
    if let some value := field.getOptParamDefault? then
      return ← withTransparency .all <| reduce value false
    if let some fn := getDefaultFnForField? (← getEnv) ty.getAppFn.constName! name then
      let some (_, value) ← instantiateStructDefaultValueFn? fn
        (some ty.getAppFn.constLevels!) ty.getAppArgs (fun n => pure (fields.find? n))
        | throwError "cannot instantiate field default {fn}"
      return ← withTransparency .all <| reduce value false
    return ← base field ordinal seen
end

/-- Constructor basis, including one recursive layer and populated structure fields. -/
partial def variants (ty : Expr) (ordinal : Nat) (seen : List Expr := []) : MetaM (Array Expr) := do
  let ty ← whnf ty
  let first ← base ty ordinal
  if seen.contains ty then return #[first]
  if [``Nat, ``Int, ``String].contains ty.getAppFn.constName! then return #[first]
  if ty.isAppOf ``Option then
    let inner := ty.getAppArgs[0]!
    let cs ← constructors inner
    let finite ← cs.allM fun c => return (← getConstInfoCtor c).numFields == 0
    let present ← if finite then variants inner ordinal (ty :: seen)
      else pure #[← base inner ordinal]
    return #[first] ++ (← present.mapM fun e => mkAppM ``Option.some #[e])
  if ty.isAppOf ``List then
    let inner := ty.getAppArgs[0]!
    let elements ← variants inner ordinal (ty :: seen)
    let mut out := #[first]
    for e in elements do out := out.push (← mkAppM ``List.cons #[e, first])
    if elements.size > 1 then
      out := out.push (← mkAppM ``List.cons #[elements[0]!,
        ← mkAppM ``List.cons #[elements[1]!, first]])
    return out
  let mut out := #[first]
  for ctor in ← constructors ty do
    let value ← construct ty ctor ordinal []
    if !out.contains value then out := out.push value
    if (getStructureInfo? (← getEnv) ty.getAppFn.constName!).isSome then
      let info ← getConstInfoCtor ctor
      let args := value.getAppArgs
      let mut ctorApp := mkAppN value.getAppFn (args.extract 0 info.numParams)
      for i in [:info.numFields] do
        let .forallE _ field _ _ ← whnf (← inferType ctorApp) | throwError "structure field"
        for arg in ← variants field ordinal (ty :: seen) do
          let changed ← construct ty ctor ordinal [] (some (i, arg))
          if !out.contains changed then out := out.push changed
        ctorApp := mkApp ctorApp args[info.numParams + i]!
  return out

/-- Preserve all single-parameter variants before filling the lexicographic product. -/
def combinations (sizes : Array Nat) : Except String (Array (Array Nat) × Nat) := do
  let total := sizes.foldl (· * ·) 1
  let zero := sizes.map fun _ => 0
  let mut out := #[zero]
  for i in [:sizes.size] do
    for j in [1:sizes[i]!] do out := out.push (zero.set! i j)
  if out.size > 256 then throw s!"{out.size} individual variants exceed cap 256"
  let mut indices := zero
  for _ in [:min total 256] do
    if out.size >= 256 then break
    if !out.contains indices then out := out.push indices
    for i in (List.range sizes.size).reverse do
      if indices[i]! + 1 < sizes[i]! then
        indices := indices.set! i (indices[i]! + 1)
        break
      indices := indices.set! i 0
  return (out.qsort (fun a b => a.toList < b.toList), total - out.size)

end DumpValues
