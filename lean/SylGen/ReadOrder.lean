import Semantics.Registry

open Lean Meta
namespace SylGen.ReadOrder

private def occurrenceKey : Name := `sylMentionOccurrence
private def originKey : Name := `sylMentionOrigin
private def isMention (e : Expr) : Bool :=
  e.isAppOfArity `Semantics.NounPhrase.firstMention 2 ||
    e.isAppOfArity `Semantics.NounPhrase.laterMention 2

private partial def firstIdentities (e : Expr) : List Expr :=
  let here := if e.isAppOfArity `Semantics.NounPhrase.firstMention 2 then
    [e.getAppArgs[0]!] else []
  here ++ match e with
  | .app f a => firstIdentities f ++ firstIdentities a
  | .lam _ _ b _ | .forallE _ _ b _ | .mdata _ b | .proj _ _ b => firstIdentities b
  | .letE _ _ v b _ => firstIdentities v ++ firstIdentities b
  | _ => []

private partial def tag (identities : List Expr) (group : String) (e : Expr) : StateM Nat Expr := do
  let (origin, inner) := match e with
    | .mdata m b => if m.contains originKey then (group ++ "/" ++ m.getString originKey, b)
        else (group, e)
    | _ => (group, e)
  if isMention inner && identities.contains inner.getAppArgs[0]! then
    let index ← get
    modify (· + 1)
    let body ← tag identities group inner.appArg!
    let metadata := (({} : MData).setNat occurrenceKey index).setString originKey origin
    return .mdata metadata (mkApp inner.appFn! body)
  match e with
  | .app f a => return .app (← tag identities group f) (← tag identities group a)
  | .lam n t b bi => return .lam n t (← tag identities group b) bi
  | .forallE n t b bi => return .forallE n t (← tag identities group b) bi
  | .letE n t v b nd => return .letE n t (← tag identities group v) (← tag identities group b) nd
  | .mdata m b => return .mdata m (← tag identities group b)
  | .proj n i b => return .proj n i (← tag identities group b)
  | _ => return e

private structure Expansion where
  nextCall : Nat := 0
  nextOccurrence : Nat := 0

private def tagCall (value : Expr) : StateT Expansion MetaM Expr := do
  let state ← get
  let (value, nextOccurrence) := (tag (firstIdentities value) (toString state.nextCall) value).run
    state.nextOccurrence
  set ({ nextCall := state.nextCall + 1, nextOccurrence } : Expansion)
  return value

private partial def expand (e : Expr) : StateT Expansion MetaM Expr := do
  match e with
  | .mdata m b => return .mdata m (← expand b)
  | .lam .. | .forallE .. => return e
  | _ => pure ()
  if let some name := e.getAppFn.constName? then
    if let .defnInfo info ← getConstInfo name then
      let body := info.value.instantiateLevelParams info.levelParams e.getAppFn.constLevels!
      let body ← tagCall body
      return ← expand (body.betaRev e.getAppArgs.reverse (preserveMData := true))
  let reduced ← whnf e
  if reduced != e then return ← expand reduced
  if e.isApp then return mkAppN e.getAppFn (← e.getAppArgs.mapM expand)
  return e

private partial def hasTag : Expr → Bool
  | .mdata m b => m.contains occurrenceKey || hasTag b
  | .app f a => hasTag f || hasTag a
  | .lam _ t b _ | .forallE _ t b _ => hasTag t || hasTag b
  | .letE _ t v b _ => hasTag t || hasTag v || hasTag b
  | .proj _ _ b => hasTag b
  | _ => false

private def fields (e : Expr) : MetaM (Array (String × Expr)) := do
  let some name := e.getAppFn.constName? | return #[]
  let info ← getConstInfo name
  forallTelescope info.type fun binders _ => do
    let mut out := #[]
    for binder in binders, arg in e.getAppArgs do
      out := out.push ((← binder.fvarId!.getDecl).userName.toString, arg)
    return out

private def reorder (fs : Array (String × Expr)) (names : List String) : Array Expr :=
  (names.toArray.filterMap fun n => (fs.find? (·.1 == n)).map (·.2)) ++
    (fs.filter (fun f => !names.contains f.1)).map (·.2)

/-- Checker read-order exceptions to constructor field order. Names here are AST fields,
not plugin declarations. `stepActionClauseWith` handles passive agents separately below. -/
def fieldOrders : Array (Name × List String) := #[
  (`Semantics.Action.preventDamage, ["damage", "cut", "also"]),
  (`Semantics.Action.redirectDamage, ["damage", "cut", "to"]),
  (`Semantics.Action.vote, ["first", "times", "ballot"]),
  (`Semantics.Action.removeCounters, ["from_", "quantity", "kind"]),
  (`Semantics.Action.moveCounters, ["source", "amount", "kind", "destination"]),
  (`Semantics.Action.pay, ["whose", "cost"])]

private def fieldOrder (name : Name) : List String :=
  ((fieldOrders.find? (·.1 == name)).map (·.2)).getD []

private def branchList (name : Name) : Bool := name ∈ [
  `Semantics.Predicate.or, `Semantics.NounPhrase.or, `Semantics.Condition.or,
  `Semantics.Timing.or, `Semantics.Cost.or, `Semantics.WouldClause.wouldAny]

-- A parallel read can discard introductions, publish one branch, or retain only common identities.
private inductive ScopeResult where
  | incoming | first | last | common

private inductive Plan where
  | visit (value : Expr)
  | sequential (steps : Array Plan)
  | parallel (result : ScopeResult) (branches : Array Plan)
  | alternatives (branches : Array Plan)

private partial def elements (list : Expr) : MetaM (Array Expr) := do
  let list ← whnf list
  if list.isAppOfArity ``List.cons 3 then
    return #[list.getAppArgs[1]!] ++ (← elements list.getAppArgs[2]!)
  return #[]

private def headerPlan (fs : Array (String × Expr)) : MetaM Plan := do
  let field := fun key => (fs.find? (·.1 == key)).map (·.2)
  let visit := fun key => .sequential ((field key).toArray.map Plan.visit)
  let alternatives ← match field "alternatives" with
    | some xs => elements xs
    | none => pure #[]
  return .sequential #[
    .parallel .common (#[visit "event"] ++ alternatives.map Plan.visit),
    .parallel .incoming #[visit "while_", visit "timing"]]

private def modifiersPlan (fs : Array (String × Expr)) : Plan :=
  let visit := fun key => Plan.sequential
    (((fs.find? (·.1 == key)).map (·.2)).toArray.map Plan.visit)
  .parallel .first #[.sequential #[visit "asThough", visit "timing"], visit "except"]

private def plan (e : Expr) : MetaM Plan := do
  let some name := e.getAppFn.constName? | return .sequential #[]
  let fs ← fields e
  let find := fun key => (fs.find? (·.1 == key)).map (·.2)
  let visit := fun key => Plan.sequential ((find key).toArray.map Plan.visit)
  let list := fun key => match find key with
    | some xs => elements xs
    | none => pure #[]
  let independent := fun key => do
    return Plan.parallel .incoming ((← list key).map Plan.visit)
  let passive ← match find "subj" with
    | some subject => pure ((← whnf subject).isAppOf `Semantics.Subject.by)
    | none => pure false
  if name == `Semantics.Predicate.compareOver then
    return .sequential #[visit "domain", .parallel .last #[visit "measure", visit "bound"]]
  if name == `Semantics.JoinedHeader.mk then return ← headerPlan fs
  if name == `Semantics.Ability.triggered then
    let primary ← headerPlan (fs.filter (·.1 != "timing"))
    let headers := Plan.parallel .common (#[primary] ++ (← list "joins").map Plan.visit)
    return .parallel .incoming #[
      .sequential #[headers, visit "intervening", visit "action", visit "timing"], visit "limit"]
  if name == `Semantics.Ability.activated then
    return .parallel .incoming #[.sequential #[visit "activator", visit "cost", visit "action",
      .parallel .incoming #[visit "timing", visit "guard"]]]
  if name == `Semantics.Ability.qualified then
    return .parallel .incoming #[visit "ability", visit "timing", visit "guard"]
  if name == `Semantics.Ability.byCardClass then
    return .parallel .incoming (fs.map (Plan.visit ∘ Prod.snd))
  if name == `Semantics.Ability.keyword then
    return .parallel .incoming #[← independent "params", visit "body"]
  if name == `Semantics.DeonticMods.mk then return modifiersPlan fs
  if name == `Semantics.Characteristics.mk then
    return .parallel .first #[
      .sequential ((["power", "toughness", "loyalty", "defense"].toArray).map visit),
      ← independent "text"]
  if name == `Semantics.TokenSpec.doubleFaced then
    return .parallel .first #[visit "front", visit "back"]
  if name == `Semantics.CharacteristicEdit.addedAbilities then
    return .parallel .incoming ((← list "abilities").map Plan.visit)
  if name == `Semantics.Action.chooseModes then
    return .sequential #[visit "quantity", ← independent "modes"]
  if name == `Semantics.Action.delay then
    return .parallel .incoming #[
      .sequential #[visit "event", visit "duration", visit "body"], visit "alternatives"]
  if name.getPrefix == `Semantics.CopyExcept then
    return .parallel .incoming #[.sequential (fs.map (Plan.visit ∘ Prod.snd))]
  if name ∈ [`Semantics.StaticSpec.cant, `Semantics.StaticSpec.can,
      `Semantics.StaticSpec.must, `Semantics.StaticSpec.gatedBy] then
    let mods ← match find "mods" with
      | some mods => fields (← whnf mods)
      | none => pure #[]
    let modifier := fun key => Plan.sequential
      (((mods.find? (·.1 == key)).map (·.2)).toArray.map Plan.visit)
    let mut steps := #[]
    if name == `Semantics.StaticSpec.cant then steps := steps.push (modifier "cause")
    unless passive do steps := steps.push (visit "subj")
    steps := steps ++ #[visit "action", modifiersPlan mods]
    if name == `Semantics.StaticSpec.gatedBy then steps := steps.push (visit "cost")
    if passive then steps := steps.push (visit "subj")
    if name == `Semantics.StaticSpec.can then steps := steps.push (modifier "rider")
    return .sequential steps
  if passive && (find "action").isSome then
    return .sequential (#[visit "action", visit "subj"] ++
      (fs.filter (fun f => !["subj", "action"].contains f.1)).map (Plan.visit ∘ Prod.snd))
  if branchList name || (name ∈ [`Semantics.Predicate.coord,
      `Semantics.NounPhrase.coord, `Semantics.Location.coord] &&
      (match find "coordinator" with
       | some coordinator => coordinator.isConstOf `Semantics.Coordinator.or ||
           (name == `Semantics.Predicate.coord &&
             coordinator.isConstOf `Semantics.Coordinator.andOr)
       | none => false)) then
    let mut branches := #[]
    for (_, value) in fs do branches := branches ++ (← elements value).map Plan.visit
    return .alternatives branches
  return .sequential ((reorder fs (fieldOrder name)).map Plan.visit)

private structure WalkState where
  seen : List (String × String) := []
  changed : Bool := false
  choices : Std.HashMap Nat Bool := {}

private partial def execute (visit : Expr → StateT WalkState MetaM Unit) :
    Plan → StateT WalkState MetaM Unit
  | .visit value => visit value
  | .sequential steps => steps.forM (execute visit)
  | .alternatives branches => execute visit (.parallel .incoming branches)
  | .parallel result branches => do
    let incoming := (← get).seen
    let mut outputs := #[]
    for branch in branches do
      modify fun s => { s with seen := incoming }
      execute visit branch
      outputs := outputs.push (← get).seen
    let seen := match result with
      | .incoming => incoming
      | .first => outputs[0]?.getD incoming
      | .last => outputs.back?.getD incoming
      | .common => (outputs[0]?.getD incoming).filter fun id => outputs.all (·.contains id)
    modify fun s => { s with seen }

private partial def walk (e : Expr) : StateT WalkState MetaM Unit := do
  unless hasTag e do return
  if let .mdata metadata inner := e then
    if metadata.contains occurrenceKey then
      let index := metadata.getNat occurrenceKey
      if (← get).choices.contains index then return
      let .lit (.strVal identity) := inner.getAppArgs[0]! | return
      let identity := (metadata.getString originKey, identity)
      let first := !(← get).seen.contains identity
      modify fun s => { s with
        seen := identity :: s.seen, choices := s.choices.insert index first,
        changed := s.changed || first != inner.isAppOf `Semantics.NounPhrase.firstMention }
      if first then walk inner.appArg!
      return
    walk inner
    return
  let e ← whnf e
  -- Reduction can expose an occurrence passed through a transparent declaration.
  if let .mdata _ _ := e then walk e; return
  unless hasTag e do return
  let some name := e.getAppFn.constName? | return
  if name == `Semantics.NounPhrase.laterMention then return
  execute walk (← plan e)

private partial def finish (choices : Std.HashMap Nat Bool) (e : Expr) : Expr :=
  match e with
  | .mdata metadata inner =>
    if metadata.contains occurrenceKey then
      let first := choices[metadata.getNat occurrenceKey]?
      let ctor := first.map fun yes => if yes then `Semantics.NounPhrase.firstMention
        else `Semantics.NounPhrase.laterMention
      let inner := finish choices inner
      .mdata (({} : MData).setString originKey (metadata.getString originKey))
        (ctor.elim inner fun ctor => mkAppN (mkConst ctor) inner.getAppArgs)
    else .mdata metadata (finish choices inner)
  | .app f a => .app (finish choices f) (finish choices a)
  | .lam n t b bi => .lam n (finish choices t) (finish choices b) bi
  | .forallE n t b bi => .forallE n (finish choices t) (finish choices b) bi
  | .letE n t v b nd => .letE n (finish choices t) (finish choices v) (finish choices b) nd
  | .proj n i b => .proj n i (finish choices b)
  | _ => e

-- Beta expansion can duplicate a tagged argument across branches. Each copy makes its own
-- first/later choice, while origin metadata preserves the shared lexical identity.
private partial def freshOccurrences (e : Expr) : StateM Nat Expr := do
  match e with
  | .mdata m b =>
    let m ← if m.contains occurrenceKey then do
        let index ← get
        modify (· + 1)
        pure (m.setNat occurrenceKey index)
      else pure m
    return .mdata m (← freshOccurrences b)
  | .app f a => return .app (← freshOccurrences f) (← freshOccurrences a)
  | .lam n t b bi => return .lam n t (← freshOccurrences b) bi
  | .letE n t v b nd => return .letE n t (← freshOccurrences v) (← freshOccurrences b) nd
  | .proj n i b => return .proj n i (← freshOccurrences b)
  | _ => return e

/-- Assign occurrences after expansion. Keep calls when their assignments already agree;
when a surrounding clause changes a helper's order, retain the corrected constructor tree.
Origin metadata keeps each helper invocation's local identities separate on further expansion. -/
def assign (value : Expr) : MetaM Expr := do
  lambdaTelescope value fun parameters body => do
    let (expanded, _) ← (do expand (← tagCall body)).run {}
    let expanded := (freshOccurrences expanded).run' 0
    let (_, state) ← (walk expanded).run {}
    if !state.changed then return value
    mkLambdaFVars parameters (finish state.choices expanded)

end SylGen.ReadOrder
