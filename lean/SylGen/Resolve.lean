import SylGen.Environment

open Lean Meta
namespace SylGen

inductive Term where
  | marked (identity : String) (body : Term)
  | atom (text : String)
  | parameter (name : String) (sort : SortRef)
  | call (name : String) (args : Array (Option String × Term))
  | list (values : Array Term)
  | record (fields : Array (String × Term))
  deriving Inhabited

structure Context where
  constructors : Registry
  declarations : Registry
  fallback : Registry := {}
  parameters : Array Parameter := #[]
  legacyScope : Bool := false
  deriving Inhabited

structure Resolution where
  dependencies : Array Name := #[]
  deriving Inhabited

abbrev ResolveM := ReaderT Context (StateT Resolution MetaM)

private def fail {α : Type} (p : Position) (message : String) : ResolveM α :=
  throwError p.error message

private def findSignature (head : String) (sort : SortRef) : ResolveM (Option Signature) := do
  let ctx ← read
  if let some sig := ctx.declarations[(head, sort.text)]? then
    modify fun s => { s with dependencies := s.dependencies.push sig.constant }
    return some sig
  if let some sig := ctx.fallback[(head, sort.text)]? then return some sig
  if sort.name == "KeywordLabel" then
    if let some ability := ctx.fallback[(head, "Ability")]? then
      if (← getEnv).contains (ability.constant ++ `label) then
        return some (← signature (ability.constant ++ `label))
  if sort.name == "Deed" then
    if let some action := ctx.fallback[(head, "Action")]? then
      if (← getEnv).contains (action.constant ++ `deed) then
        return some (← signature (action.constant ++ `deed))
  return ctx.constructors[(head, sort.text)]? <|> ctx.constructors[(leanWord head, sort.text)]?

private def underlying (s : SortRef) : SortRef :=
  if s.name == "Option" then s.arguments[0]! else s

mutual
  partial def resolve (v : Value) (expected : SortRef) : ResolveM Term := do
    let ctx ← read
    if let .record p fields := v then
      if expected.name == "CardType" then
        fail p "raw CardType construction requires a registry declaration"
      if expected.name == "Card" then
        return .call "Semantics.Card.singleFaced"
          #[(none, ← resolve v { name := "CardFace" })]
      if expected.name == "CardFace" && !(fields.any (·.1 == "characteristics")) then
        return .record #[("characteristics", ← resolve (.record p fields) { name := "Characteristics" })]
    if let .parameter p name := v then
      let some param := ctx.parameters.find? (·.name == name)
        | fail p s!"unknown parameter '${name}'"
      if param.sort == expected then return .parameter name param.sort
      if expected.name == "Option" && param.sort == underlying expected then
        return .call "some" #[(none, .parameter name param.sort)]
      fail p s!"parameter '${name}' has sort {param.sort.text}, expected {expected.text}"
    if let .word p head args := v then
      if head == "none" then
        unless expected.name == "Option" && args.isEmpty do fail p "none requires Option"
        return .atom "none"
      if head == "some" && expected.name == "Option" then
        fail p "some is implicit; write the value"
      if let some sig ← findSignature head expected then
        if let some message := sourceConstructorRefusal sig.constant ctx.legacyScope then fail p message
        let args := if expected.name == "Card" && sig.constant.getString! != "singleFaced" then
            match args with
            | #[(none, .record _ fields)] => fields.map (fun (k, v) => (some k, v))
            | _ => args
          else args
        return ← resolveCall p sig args
      if expected.name == "KeywordTerm" && args.isEmpty then
        if let some sig ← findSignature head { name := "KeywordLabel" } then
          return .call "Semantics.KeywordTerm.the" #[(none, ← resolveCall p sig #[])]
    if expected.name == "Option" then
      return .call "some" #[(none, ← resolve v (underlying expected))]
    match v with
    | .string p text =>
      unless expected.name == "String" do fail p s!"string at {expected.text}"
      return .atom (toString (Lean.Json.str text))
    | .integer p text =>
      let leaf := Term.atom ("(" ++ text ++ ")")
      match expected.name with
      | "Int" => return leaf
      | "Nat" =>
        if text.startsWith "-" then fail p "negative integer at Nat"
        return leaf
      | "Amount" => return .call "Semantics.Amount.lit" #[(none, leaf)]
      | "ManaSymbol" =>
        if text.startsWith "-" then fail p "negative generic mana"
        return .call "Semantics.ManaSymbol.simple" #[(none, .call "Semantics.SimpleManaSymbol.generic" #[(none, leaf)])]
      | _ => fail p s!"integer at {expected.text}"
    | .list p values =>
      unless expected.name == "List" do fail p s!"list at {expected.text}"
      return .list (← values.mapM (resolve · expected.arguments[0]!))
    | .record p fields =>
      let signature ← structureFields expected
      let mut result := #[]
      for (key, value) in fields do
        unless signature.any (fun f => f.name == key || f.name == leanWord key) do
          fail value.position s!"unknown field '{key}' at {expected.text}"
      for f in signature do
        if let some (_, value) := fields.find? (fun field => field.1 == f.name || leanWord field.1 == f.name) then
          result := result.push (f.name, ← resolve value f.sort)
        else unless f.hasDefault do
          fail p s!"missing field '{f.name}' at {expected.text}"
      return .record result
    | .word p head args =>
      if args.isEmpty && head ∈ ["white", "blue", "black", "red", "green"] then
        if expected.name ∈ ["ColorOrColorless", "ManaSymbol"] then
          let color := Term.call "Semantics.ColorOrColorless.of"
            #[(none, .atom ("Semantics.Color." ++ head))]
          if expected.name == "ColorOrColorless" then return color
          return .call "Semantics.ManaSymbol.simple"
            #[(none, .call "Semantics.SimpleManaSymbol.specific" #[(none, color)])]
      fail p s!"unknown word '{head}' at {expected.text}"
    | .parameter p name => fail p s!"unknown parameter '${name}'"

  partial def resolveCall (p : Position) (sig : Signature)
      (args : Array (Option String × Value)) : ResolveM Term := do
    let mut result := #[]
    let mut assigned : Array String := #[]
    -- Arguments bind in written order. A named argument binds its parameter; a positional one
    -- binds the first parameter not yet bound, so it skips parameters named before it. Naming a
    -- parameter that an earlier argument bound, by name or position, is a duplicate.
    for (key, value) in args do
      let field ← if let some key := key then do
          let some f := sig.fields.find? (fun f => f.name == key || f.name == leanWord key)
            | fail value.position s!"unknown argument '{key}'"
          pure f
        else do
          let some f := sig.fields.find? (fun f => !assigned.contains f.name)
            | fail value.position "too many arguments"
          pure f
      if assigned.contains field.name then fail value.position s!"duplicate argument '{field.name}'"
      assigned := assigned.push field.name
      result := result.push (some field.name, ← resolve value field.sort)
    for field in sig.fields do
      unless field.hasDefault || assigned.contains field.name do
        fail p s!"missing argument '{field.name}' for {sig.constant}"
    -- Lean elaborates applications in binder order, even when source named arguments differ.
    let ordered := sig.fields.filterMap fun f =>
      (result.find? (·.1 == some f.name)).map fun (_, value) =>
        (some (if f.emittedName.isEmpty then f.name else f.emittedName), value)
    let call := Term.call (leanName sig.constant) (if sig.constant.getPrefix == ``Int then
      ordered.map fun (_, value) => (none, value) else ordered)
    return sig.identity.elim call (fun identity => .marked identity call)
end

partial def Term.count (name : String) : Term → Nat
  | .marked _ body => body.count name
  | .parameter n _ => if n == name then 1 else 0
  | .atom _ => 0
  | .call _ args => (args.toList.map fun a => a.2.count name).foldl (· + ·) 0
  | .list args => (args.toList.map (Term.count name)).foldl (· + ·) 0
  | .record args => (args.toList.map fun a => a.2.count name).foldl (· + ·) 0

partial def Term.identityCount (identity : String) : Term → Nat
  | .marked mark body => (if mark == identity then 1 else 0) + body.identityCount identity
  | .call _ args => (args.toList.map fun a => a.2.identityCount identity).foldl (· + ·) 0
  | .list args => (args.toList.map (Term.identityCount identity)).foldl (· + ·) 0
  | .record args => (args.toList.map fun a => a.2.identityCount identity).foldl (· + ·) 0
  | _ => 0

partial def callerSort (s : SortRef) : Bool :=
  if s.name ∈ ["Option", "List"] then callerSort s.arguments[0]!
  else s.name ∈ ["Action", "Cost", "StaticSpec", "Replacement", "Amount", "Predicate",
    "Quantity", "Location", "Condition", "NounPhrase", "GameEvent", "Ability", "TokenSpec",
    "MoveRider", "PlayPayment", "Ballot"]

private def mention (identity inner : String) : StateM (Array String) String := do
  let seen ← get
  let ctor := if seen.contains identity then "laterMention" else "firstMention"
  modify (·.push identity)
  return s!"(Semantics.NounPhrase.{ctor} {toString (Lean.Json.str identity)} {inner})"

partial def render (decl : Declaration) (body : Term) (t : Term) : StateM (Array String) String := do
  match t with
  | .marked identity inner =>
    let text ← render decl body inner
    if decl.kind == "action" && body.identityCount identity ≥ 2 then
      return ← mention (declarationWord decl.name ++ "." ++ identity) text
    return text
  | .atom text => return text
  | .parameter name sort =>
    let binder :=  (decl.parameters.find? (·.name == name)).map parameterName |>.getD (leanWord name)
    let isDefinition := decl.kind ∈ ["action", "ability"]
    let inner := if isDefinition && callerSort sort then
        s!"(Semantics.CallerScope.caller {quoted binder})" else quoted binder
    if sort.name == "NounPhrase" && (isDefinition || (decl.kind == "template" && body.count name ≥ 2)) then
      return ← mention (declarationWord decl.name ++ "." ++ leanWord name) inner
    return inner
  | .list values =>
    return "[" ++ String.intercalate ", " (← values.toList.mapM (render decl body)) ++ "]"
  | .record fields =>
    let fields ← fields.toList.mapM fun (key, value) => do
      return quoted key ++ " := " ++ (← render decl body value)
    return "{ " ++ String.intercalate ", " fields ++ " }"
  | .call name args =>
    let args ← args.toList.mapM fun (key, value) => do
      let text ← render decl body value
      return match key with
        | none => "(" ++ text ++ ")"
        | some key => "(" ++ quoted key ++ " := " ++ text ++ ")"
    let text := "(" ++ name ++ " " ++ String.intercalate " " args ++ ")"
    return text

end SylGen
