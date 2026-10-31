import Semantics.Card

open Lean Meta Elab Command

namespace Semantics.Registry

/-- A declaration's generated reference identity, independent of its Lean namespace. -/
initialize referenceIdentities : SimplePersistentEnvExtension (Name × String) (NameMap String) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := fun entries (name, identity) => entries.insert name identity
    addImportedFn := fun entries => entries.foldl (fun out es =>
      es.foldl (fun out (name, identity) => out.insert name identity) out) {}
  }

elab "syl_identity " name:ident identity:str : command => do
  let name ← resolveGlobalConstNoOverload name
  modifyEnv (referenceIdentities.addEntry · (name, identity.getString))

/-- Expected-result dispatch entries survive module imports. -/
structure ExpectedTypeEntry where
  bareName : Name
  returnType : Name
  declaration : Name
  deriving Inhabited

private def addExpectedTypeEntry (table : NameMap (List ExpectedTypeEntry))
    (entry : ExpectedTypeEntry) : NameMap (List ExpectedTypeEntry) :=
  let prev := table.find? entry.bareName |>.getD []
  let filtered := prev.filter fun previous =>
    previous.returnType != entry.returnType || previous.declaration != entry.declaration
  table.insert entry.bareName (entry :: filtered)

initialize expectedTypeDispatchExt : SimplePersistentEnvExtension ExpectedTypeEntry
    (NameMap (List ExpectedTypeEntry)) ← registerSimplePersistentEnvExtension {
  addEntryFn := addExpectedTypeEntry
  addImportedFn := mkStateFromImportedEntries addExpectedTypeEntry {}
}

private def expectedTypeEntries (env : Environment) (bare : Name) : List ExpectedTypeEntry :=
  expectedTypeDispatchExt.getState env |>.find? bare |>.getD []

private def resultSort? (type : Expr) : MetaM (Option Name) := do
  let type ← whnfR type
  let type := if type.isAppOfArity ``Option 1 then type.appArg! else type
  return (← whnfR type).getAppFn.constName?

def registerExpectedType (declaration : Name) : CommandElabM Unit := do
  let result ← liftTermElabM do
    forallTelescopeReducing (← getConstInfo declaration).type fun _ result =>
      resultSort? result
  if let some returnType := result then
    let bareName := Name.mkSimple declaration.getString!
    modifyEnv fun env => expectedTypeDispatchExt.addEntry env
      { bareName, returnType, declaration }
    let word := bareName.getString!
    let nounAliases := ["Creature", "Artifact", "Enchantment", "Land", "Planeswalker", "Battle",
      "Instant", "Sorcery", "Kindred", "Card", "Permanent", "Player", "Token", "Spell"]
    if returnType == `Semantics.Predicate && nounAliases.any (fun n => word == "is" ++ n) then
      let alias := Name.mkSimple ((word.drop 2).toString.toLower)
      modifyEnv fun env => expectedTypeDispatchExt.addEntry env
        { bareName := alias, returnType, declaration }
    if bareName == `this_ then
      modifyEnv fun env => expectedTypeDispatchExt.addEntry env
        { bareName := `this, returnType, declaration }

private def lastNameComponent : Name → String
  | .str _ part => part
  | _ => ""

private def addSortAlias (full : Name) : CommandElabM Unit := do
  let previous ← getConstInfo full
  let previousSort? ← liftTermElabM <| forallTelescopeReducing previous.type fun _ type =>
    resultSort? type
  if let some previousSort := previousSort? then
    if lastNameComponent full.getPrefix == previousSort.getString! then return
    let alias := full.getPrefix ++ Name.mkSimple previousSort.getString! ++
      Name.mkSimple full.getString!
    unless (← getEnv).contains alias do
      let decl := .defnDecl {
        name := alias, levelParams := previous.levelParams, type := previous.type
        value := mkConst full (previous.levelParams.map Level.param)
        hints := .abbrev, safety := .safe
      }
      -- A generated canon card is `noncomputable`, and so is its alias.
      if isNoncomputable (← getEnv) full then
        liftCoreM <| addDecl decl
        modifyEnv (addNoncomputable · alias)
      else liftCoreM <| addAndCompile decl
      for suffix in [`label, `deed] do
        if let some info := (← getEnv).find? (full ++ suffix) then
          liftCoreM <| addAndCompile <| .defnDecl {
            name := alias ++ suffix, levelParams := info.levelParams, type := info.type
            value := mkConst (full ++ suffix) (info.levelParams.map Level.param)
            hints := .abbrev, safety := .safe
          }
      registerExpectedType alias

def registerSharedAliases (declaration : Name) : CommandElabM Unit := do
  addSortAlias declaration
  let result? ← liftTermElabM <| forallTelescopeReducing (← getConstInfo declaration).type
    fun _ type => resultSort? type
  if let some result := result? then
    let ns := if lastNameComponent declaration.getPrefix == result.getString! then
      declaration.getPrefix.getPrefix else declaration.getPrefix
    let bare := Name.mkSimple declaration.getString!
    let siblings := (expectedTypeEntries (← getEnv) bare).filter fun entry =>
      entry.declaration.getString! == bare.getString! &&
        entry.returnType != result &&
        (entry.declaration.getPrefix == ns ||
          entry.declaration.getPrefix == ns ++ Name.mkSimple entry.returnType.getString!)
    unless siblings.isEmpty do
      addSortAlias declaration
      for sibling in siblings do addSortAlias sibling.declaration

/-- The keyword-action deed an Action Definition establishes its own Reference Scope for
(`.ownScope (some (.action "exile")) …`), read from its expansion with its parameters left
abstract; `none` for any other Definition, or a deed that depends on a parameter. -/
def actionDefinitionDeed? (name : Name) : MetaM (Option Expr) := do
  let some value := (← getConstInfo name).value? | return none
  lambdaTelescope value fun _ body => do
    unless (← whnf (← inferType body)).isConstOf `Semantics.Action do return none
    let body ← whnf body
    unless body.isAppOfArity `Semantics.Action.ownScope 2 do return none
    let deed ← whnf body.appFn!.appArg!
    unless deed.isAppOfArity `Option.some 2 do return none
    let deed ← instantiateMVars deed.appArg!
    let deed ← whnf deed
    unless deed.isAppOfArity `Semantics.Deed.action 2 do return none
    let label ← whnf deed.appFn!.appArg!
    let permanent ← whnf deed.appArg!
    unless label.isStringLit && !permanent.hasFVar && !permanent.hasMVar do
      return none
    return some (mkApp2 (mkConst `Semantics.Deed.action) label permanent)

initialize actionDeedsExt : SimplePersistentEnvExtension (Name × Expr) (List (Name × Expr)) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := fun entries entry => entry :: entries
    addImportedFn := fun imports => imports.toList.flatMap Array.toList
  }

/-- Definitions sharing a keyword-action label must agree on what the deed says of a
Permanent, including Definitions imported from other modules. -/
def checkActionDeed (declaration : Name) (deed : Expr) : MetaM Unit := do
  let env ← getEnv
  for (name, other) in actionDeedsExt.getState env do
    if name != declaration then
      if (← isDefEq deed.appFn!.appArg! other.appFn!.appArg!) &&
          !(← isDefEq deed.appArg! other.appArg!) then
        throwError "Action Definitions for deed {deed.appFn!.appArg!} must agree on DeedPermanent"

/-- A canon card generated into a data module: its card file and the declaration it defines.
`check_canon_card` (`Check/Card.lean`) finds the declaration by file. -/
structure CanonCardEntry where
  file : String
  declaration : Name
  deriving Inhabited

initialize canonCardsExt :
    SimplePersistentEnvExtension CanonCardEntry (Array CanonCardEntry) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := fun entries entry => entries.push entry
    addImportedFn := fun imports => imports.foldl (· ++ ·) #[]
  }

/-- The card file of a printed name: `card_filename` in `crates/deckmaste_core/src/plugin.rs`
with the source extension used by the generated registry. -/
def cardFileName (name : String) : String :=
  name.foldl (init := "") (fun out c => out ++ match c with
    | '<' => "{less}" | '>' => "{greater}" | ':' => "{colon}" | '"' => "{quote}"
    | '/' => "{slash}" | '\\' => "{backslash}" | '|' => "{pipe}" | '?' => "{question}"
    | '*' => "{star}" | c => c.toString) ++ ".syl"

end Semantics.Registry
