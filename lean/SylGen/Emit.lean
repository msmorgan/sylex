import SylGen.Resolve

open Lean Meta
namespace SylGen

private partial def sortNamespace (sort : SortRef) : Name :=
  sort.arguments.foldl (fun ns arg => ns ++ sortNamespace arg) sort.name.toName

/-- A declaration's Lean name. A word declared at more than one sort, or whose plain name a
hand-written Lean constant already has (`taken`), goes under its sort's namespace. -/
def declarationName (root : Name) (d : Declaration) (ds : Array Declaration)
    (taken : Name → Bool := fun _ => false) : Name :=
  let ns := match d.kind with
    | "primitive" | "template" => `Semantics.Templates
    | "card" => `Semantics.Cards
    | _ => `Semantics.Definitions
  let word := Name.mkSimple (declarationWord d.name)
  let shared := ds.any (fun other => other.name == d.name && other.sort != d.sort) ||
    taken (root ++ ns ++ word)
  root ++ ns ++ (if shared then sortNamespace d.sort else .anonymous) ++ word

private def allowedFields (kind : String) : List String :=
  ["lexical", "spelling"] ++ match kind with
    | "primitive" => ["body"]
    | "template" => ["body", "identity"]
    | "action" => ["body", "deed", "permanent", "ofAbility"]
    | "ability" => ["body", "label", "cost", "reminder"]
    | "ability_word" => ["body", "label"]
    | "counter" => ["holder", "confers", "rules"]
    | "designation" => ["scope", "sole", "confers", "rules"]
    | "subtype" => ["kind", "name", "confers", "rules"]
    | "card_type" => ["name", "cardClass", "stats", "confers", "rules"]
    | "relation" => []
    | "token" => ["name", "token", "body"]
    | "sba" => ["scope", "when", "then", "then_", "body"]
    | "conferral" => ["scope", "confer", "body"]
    | "card" => ["name", "cost", "colors", "supertypes", "types", "subtypes", "text",
        "power", "toughness", "loyalty", "defense"]
    | _ => []

def loadDeclarations (root : Name) (ds : Array Declaration) (_fallback : Registry)
    (ctors : Registry) : MetaM Registry := do
  let mut table : Registry := {}
  for d in ds do
    for (key, value) in d.fields do
      unless (allowedFields d.kind).contains key do
        throwError value.position.error s!"unknown {d.kind} field '{key}'"
    let key := (d.name, d.sort.text)
    if table.contains key then throwError d.pos.error s!"Duplicate bundle declaration {key}"
    let env ← getEnv
    let name := declarationName root d ds env.contains
    let identity : Option String ← match (d.fields.find? (·.1 == "identity")).map (·.2) with
      | none => pure none
      | some (Value.string _ value) =>
        unless d.kind == "template" && d.sort.name == "NounPhrase" &&
            d.parameters.isEmpty && !value.isEmpty do
          throwError d.pos.error "identity requires a parameterless NounPhrase template and a nonempty string"
        pure (some value)
      | some v => throwError v.position.error "identity must be a string"
    let built := if d.kind != "primitive" then none else
      d.constructor.bind (ctors.constructor? d.sort)
    let refusal := built.bind (fun ctor => sourceConstructorRefusal ctor.constant)
    let sig : Signature := { constant := name, result := d.sort, identity, reserved := refusal,
                             fields := d.parameters.map fun p =>
      { name := leanWord p.name, sort := p.sort, hasDefault := p.defaultValue.isSome,
        emittedName := parameterName p } }
    table := table.insert key sig
    if d.kind == "ability" then
      for sort in ["KeywordLabel", "KeywordTerm"] do
        if table.contains (d.name, sort) then
          throwError d.pos.error s!"duplicate declaration {(d.name, sort)}"
      table := table.insert (d.name, "KeywordLabel")
        { constant := name ++ `label, fields := #[], result := { name := "KeywordLabel" } }
    if d.kind == "action" && !(d.fields.any (·.1 == "ofAbility")) &&
        !(d.fields.any (fun f => match f with
          | ("deed", .word _ "none" #[]) => true
          | _ => false)) then
      if table.contains (d.name, "Deed") then
        throwError d.pos.error s!"duplicate declaration {(d.name, "Deed")}"
      table := table.insert (d.name, "Deed")
        { constant := name ++ `deed, fields := #[], result := { name := "Deed" } }
  return table

private def field (d : Declaration) (name : String) : Option Value :=
  (d.fields.find? (·.1 == name)).map (·.2)

private def required (d : Declaration) (name : String) : ResolveM Value := do
  let some v := field d name | throwError d.pos.error s!"{d.kind} requires '{name}'"
  return v

private def str (d : Declaration) (text : String) : Value := .string d.pos text
private def word (d : Declaration) (text : String) : Value := .word d.pos text #[]
private def empty (d : Declaration) : Value := .list d.pos #[]

private def direct (d : Declaration) (constant : Name) (args : Array (Option String × Value)) :
    ResolveM Term := do
  resolveCall d.pos (← signature constant) args

private def rendered (d : Declaration) (t : Term) : String :=
  Id.run ((render d t t).run' #[])

private def body (d : Declaration) : ResolveM Term := do
  let get := fun name defaultValue => (field d name).getD defaultValue
  match d.kind with
  | "primitive" =>
    if (field d "body").isSome then throwError d.pos.error "primitive must be body-less"
    let some clause := d.constructor
      | throwError d.pos.error "primitive requires a constructor clause"
    let some sig := (← read).constructors.constructor? d.sort clause
      | throwError d.pos.error
          s!"primitive must name a constructor of its result sort; no {d.sort.text}.{leanConstructor d.sort clause}"
    if let some message := sourceConstructorRefusal sig.constant (← read).legacyScope then
      throwError d.pos.error message
    unless d.parameters.size == sig.fields.size do
      throwError d.pos.error "primitive must bind every constructor field"
    for p in d.parameters do
      unless sig.fields.any (fun f => f.name == leanWord p.name && f.sort == p.sort) do
        throwError p.pos.error "primitive parameter does not match its constructor field"
    return .call (leanName sig.constant)
      (d.parameters.map fun p => (some (leanWord p.name), .parameter p.name p.sort))
  | "template" => return ← resolve (← required d "body") d.sort
  | "action" =>
    if (field d "ofAbility").isSome && ((field d "deed").isSome || (field d "permanent").isSome) then
      throwError d.pos.error "ofAbility cannot be combined with deed or permanent"
    let action ← resolve (← required d "body") { name := "Action" }
    if let some (.word _ "none" #[]) := field d "deed" then
      return .call "Semantics.Action.ownScope" #[(none, .atom "none"), (none, action)]
    let deed ← if let some v := field d "ofAbility" then
        direct d `Semantics.Deed.ofAbility #[(none, v)]
      else
        let label := get "deed" (str d d.name)
        let mut args := #[(none, label)]
        if let some permanent := field d "permanent" then args := args.push (none, permanent)
        direct d `Semantics.Deed.action args
    return .call "Semantics.Action.ownScope"
      #[(none, .call "some" #[(none, deed)]), (none, action)]
  | "ability" =>
    let name := (← read).declarations[(d.name, "Ability")]!.constant
    let mut params := #[]
    for p in d.parameters do
      if p.sort.name ∈ ["NounPhrase", "Action"] then continue
      let ctor ← match p.sort.name with
        | "Cost" => pure "cost" | "Amount" => pure "number"
        | "Predicate" => pure (if p.name == "subject" then "subject" else "quality")
        | "DeckCondition" => pure "deckCondition" | "Ability" => pure "ability"
        | _ => throwError p.pos.error "unsupported keyword parameter"
      params := params.push (.call ("Semantics.KeywordParam." ++ ctor)
        #[(none, .parameter p.name p.sort)])
    return .call "Semantics.Ability.keyword" #[(none, .atom (leanName (name ++ `label))),
      (none, .list params),
      (none, ← resolve (get "body" (empty d)) { name := "List", arguments := #[{ name := "Ability" }] })]
  | "ability_word" =>
    let label ← resolve (get "label" (str d d.name)) { name := "String" }
    let content ← resolve (← required d "body") { name := "Ability" }
    let some wrap ← findSignature "abilityWord" { name := "Ability" }
      | throwError d.pos.error "ability_word requires the abilityWord Template"
    if wrap.identity.isSome then throwError d.pos.error "abilityWord must be a Template"
    let wrapName := leanName wrap.constant
    return .atom s!"({wrapName}, ({rendered d label}, ({rendered d content})))"
  | "counter" =>
    return ← direct d `Semantics.CounterKind.named
        #[(none, str d d.name), (none, get "holder" (word d "object"))]
  | "designation" =>
    return ← direct d `Semantics.DesignationLabel.named
        #[(none, str d d.name), (none, ← required d "scope"),
        (some "sole", get "sole" (word d "false"))]
  | "relation" => return ← direct d `Semantics.RelationLabel.named #[(none, str d d.name)]
  | "subtype" =>
    let kind ← required d "kind"
    if let .word _ "spell" _ := kind then
      return ← direct d `Semantics.Subtype.spell #[(none, get "name" (str d d.name))]
    else return ← direct d `Semantics.Subtype.of #[(none, kind), (none, get "name" (str d d.name))]
  | "card_type" =>
    return ← direct d `Semantics.CardType.mk #[
      (some "name", get "name" (str d d.name)),
      (some "cardClass", get "cardClass" (word d "none")),
      (some "stats", get "stats" (empty d))]
  | "card" =>
    let some value := d.cardValue | throwError d.pos.error "missing card value"
    let discardLexical := fun fields => fields.filter fun f => !(f.1 ∈ ["lexical", "spelling"])
    let value := match value with
      | .record p fields => .record p (discardLexical fields)
      | .word p head #[(none, .record q fields)] =>
        .word p head #[(none, .record q (discardLexical fields))]
      | other => other
    return ← resolve value { name := "Card" }
  | _ =>
    let v := (field d "body").getD (.record d.pos (d.fields.filter fun f =>
      !(f.1 ∈ ["lexical", "spelling"])))
    return ← resolve v d.sort

structure Output where
  name : Name
  text : String
  prelude : String := ""
  check : String := ""
  checksRules : Bool := false
  dependencies : Array Name
  deriving Inhabited

def emit (d : Declaration) : ResolveM Output := do
  let name := (← read).declarations[(d.name, d.sort.text)]!.constant
  let mut binders := ""
  for p in d.parameters do
    let defaultText ← if let some v := p.defaultValue then do
        let term ← withReader (fun c => { c with parameters := d.parameters.takeWhile (·.name != p.name) }) (resolve v p.sort)
        pure (" := " ++ rendered { d with kind := "primitive" } term)
      else pure ""
    binders := binders ++ s!" ({quoted (parameterName p)} : {p.sort.lean}{defaultText})"
  let b ← body d
  let value := rendered d b
  let mut text := ""
  if d.kind == "ability" then
    let label ← resolve ((field d "label").getD (str d d.name)) { name := "String" }
    let label := rendered d label
    text := s!"def {leanName (name ++ `label)} : Semantics.KeywordLabel := .named {label}\n"
  if d.kind == "action" then
    if let .call _ args := b then
      if let .call _ deed := args[0]!.2 then
        let term := deed[0]!.2
        if let .call ctor _ := term then
          if ctor == leanName `Semantics.Deed.action then
            text := text ++ s!"def {leanName (name ++ `deed)} : Semantics.Deed := {rendered d term}\n"
  let mut prelude := text
  let documentation := if d.documentation.isEmpty then "" else
    "/--" ++ d.documentation.replace "-/" "- /" ++ "-/\n"
  let modifier := if d.kind == "card" then "noncomputable " else ""
  text := if d.kind == "ability_word" then
    s!"syl_ability_word {leanName name}{binders} := {value} doc {repr d.documentation}\n"
  else if d.kind ∈ ["template", "action", "ability"] then
    s!"syl_def {leanName name}{binders} : {d.sort.lean} := {value} doc {repr d.documentation}\n"
  else documentation ++ modifier ++ s!"def {leanName name}{binders} : {d.sort.lean} :=\n  {value}\n"
  if d.kind ∈ ["counter", "designation", "subtype", "card_type", "relation"] then
    prelude := prelude ++ text
    text := ""
    let ctor := if d.kind == "card_type" then "cardType" else d.kind
    let rules ← resolve ((field d "rules" <|> field d "confers").getD (empty d))
      { name := "List", arguments := #[{ name := "Conferral" }] }
    let rules := if d.kind == "relation" then "" else rendered d rules
    let row := name.getPrefix ++ Name.mkSimple (name.getString! ++ "_definition")
    text := text ++ s!"def {leanName row} : Semantics.RegistryDefinition := .{ctor} {leanName name} {rules}\n"
    let checkName := name.getPrefix ++ Name.mkSimple ("ok_" ++ name.getString!)
    text := text ++ s!"theorem {leanName checkName} : Semantics.RegistryDefinition.check {leanName row} = [] := by decide\n"
  if d.kind ∈ ["sba", "conferral", "token"] then
    let checkName := name.getPrefix ++ Name.mkSimple ("ok_" ++ name.getString!)
    text := text ++ s!"theorem {leanName checkName} : {d.sort.lean}.check {leanName name} = [] := by decide\n"
  let check := if d.kind == "card" then
      s!"check_card {leanName name}\n"
    else ""
  let cardFile := if d.kind == "card" then d.name ++ ".syl" else ""
  if let some identity := (← read).declarations[(d.name, d.sort.text)]!.identity then
    text := text ++ s!"syl_identity {leanName name} {repr identity}\n"
  text := text ++ s!"syl_register {toString (Json.str d.kind)} {leanName name} {toString (Json.str cardFile)}\n"
  return { name, text, prelude, check
           checksRules := d.kind ∈ ["counter", "designation", "subtype", "card_type", "relation",
             "sba", "conferral", "token"]
           dependencies := (← get).dependencies }

end SylGen
