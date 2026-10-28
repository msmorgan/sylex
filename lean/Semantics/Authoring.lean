import Lean

/-!
# Semantics.Authoring

The authoring boundary inspects elaborated terms before reduction erases their template and definition calls.
Its retained tree records named expansion calls and data composition. The kernel checks
that the tree contains no raw semantic constructor; correspondence between the elaborated
term and this tree is the elaborator's responsibility.
-/

namespace Semantics

/-- Expansion-time lexical depth. This argument is eliminated when a template or definition
constructs its body. -/
class AuthoringContext where
  depth : Nat

abbrev AuthoringContext.root : AuthoringContext := ⟨0⟩

instance : AuthoringContext := AuthoringContext.root

abbrev AuthoringContext.child (context : AuthoringContext) : AuthoringContext := ⟨context.depth + 1⟩

end Semantics

namespace Semantics.Authoring

open Lean Meta Elab Command Term

inductive Form where
  | data (children : List Form)
  | literal (value : Literal)
  | constructor (name : Name) (fields : List Form)
  | call (name : Name) (arguments : List Form)
  | parameter (name : Name)
  | binder (name : Name) (body : Form)
  | raw (name : Name)
  deriving Repr, ToExpr

mutual
  def Form.onlyAuthored : Form → Bool
    | .data children | .call _ children | .constructor _ children => Forms.onlyAuthored children
    | .literal _ => true
    | .parameter _ => true
    | .binder _ body => body.onlyAuthored
    | .raw _ => false

  def Forms.onlyAuthored : List Form → Bool
    | [] => true
    | form :: forms => form.onlyAuthored && Forms.onlyAuthored forms
end

mutual
  def Form.rawNames : Form → List Name
    | .data children | .call _ children | .constructor _ children => Forms.rawNames children
    | .literal _ => []
    | .parameter _ => []
    | .binder _ body => body.rawNames
    | .raw name => [name]

  def Forms.rawNames : List Form → List Name
    | [] => []
    | form :: forms => form.rawNames ++ Forms.rawNames forms
end

private opaque FinalizeContext : Type

/-- Elaborate a mutual inductive block with all standard recursors, `casesOn`, `noConfusion`, and
`below`, omitting the eager `mkBRecOn`, `SizeOf`, and injectivity-theorem loops across all nested
occurrences (`mkBRecOn` can be invoked on demand for individual spine types). -/
private unsafe def runFastInductives (inductives : Array (Modifiers × Syntax)) : CommandElabM Unit := do
  let env ← getEnv
  let checkNoConflicts ← IO.ofExcept <| env.evalConst
    (Array InductiveElabStep1 → Bool → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.checkNoInductiveNameConflicts".toName
  let mkInductiveDecl ← IO.ofExcept <| env.evalConst
    (Array Expr → Array InductiveElabStep1 → TermElabM FinalizeContext)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.mkInductiveDecl".toName
  let elabInductiveViewsFinalize ← IO.ofExcept <| env.evalConst
    (Array InductiveView → FinalizeContext → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.elabInductiveViewsFinalize".toName
  let elabInductiveViewsPostprocessing ← IO.ofExcept <| env.evalConst
    (Array InductiveView → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.elabInductiveViewsPostprocessing".toName
  let elabs ← runTermElabM fun _ => inductives.mapM fun (modifiers, stx) => mkInductiveView modifiers stx
  checkNoConflicts elabs false
  elabs.forM fun e => checkValidInductiveModifier e.view.modifiers
  liftTermElabM <| elabs.forM fun e =>
    withRef e.view.ref <| Term.applyAttributesAt e.view.declName e.view.modifiers.attrs .beforeElaboration
  let res ← runTermElabM fun vars => do
    let view0 := elabs[0]!.view
    Term.withDeclName view0.declName <| withRef view0.ref <| withExporting (isExporting := !isPrivateName view0.declName) do
      let res ← mkInductiveDecl vars elabs
      let declNames := elabs.map (·.view.declName)
      compileDecls declNames
      withoutExporting (when := view0.ctors.any fun x => isPrivateName x.declName) do
        for n in declNames do
          mkCasesOn n
          mkRecOn n
          mkCtorIdx n
          mkCtorElim n
          mkNoConfusion n
          mkBelow n
      for e in elabs do
        enableRealizationsForConst e.view.declName
        for ctor in e.view.ctors do
          enableRealizationsForConst ctor.declName
      pure res
  let views := elabs.map (·.view)
  elabInductiveViewsFinalize views res
  elabInductiveViewsPostprocessing views

@[implemented_by runFastInductives]
private opaque fastInductives (inductives : Array (Modifiers × Syntax)) : CommandElabM Unit

@[scoped command_elab Lean.Parser.Command.mutual]
def elabFastMutual : CommandElab := fun stx => do
  if ← isMutualInductive stx then
    let mut inductives := #[]
    for elem in stx[1].getArgs do
      let modifiers ← elabModifiers ⟨elem[0]⟩
      inductives := inductives.push (modifiers, elem[1])
    fastInductives inductives
  else
    throwUnsupportedSyntax


initialize definitionAttribute : TagAttribute ←
  registerTagAttribute `semantic_definition "A rules-defined semantic declaration."

initialize templateAttribute : TagAttribute ←
  registerTagAttribute `semantic_template "A transparent semantic phrasing template."

initialize expressionAttribute : TagAttribute ←
  registerTagAttribute `semantic_expression "Semantic expressions require template or definition authoring."

initialize literalAttribute : TagAttribute ←
  registerTagAttribute `semantic_literal "A literal leaf value, rather than an authored expression."

initialize internalAttribute : TagAttribute ←
  registerTagAttribute `internal_expansion "An expansion carrier without an authorable primitive template."

initialize contextualAuthoringAttribute : TagAttribute ←
  registerTagAttribute `contextual_authoring "A semantic declaration with generated lexical context."

private def addSpliceSchema (schemas : NameMap (Array Name)) (entry : Name × Array Name) :=
  schemas.insert entry.1 entry.2

initialize spliceSchemas : SimplePersistentEnvExtension (Name × Array Name) (NameMap (Array Name)) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := addSpliceSchema
    addImportedFn := mkStateFromImportedEntries addSpliceSchema {}
  }

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

private def authoredResultSort? (type : Expr) : MetaM (Option Name) := do
  let type ← whnfR type
  let type := if type.isAppOfArity ``Option 1 then type.appArg! else type
  return (← whnfR type).getAppFn.constName?

private def registerExpectedType (declaration : Name) : CommandElabM Unit := do
  let result ← liftTermElabM do
    forallTelescopeReducing (← getConstInfo declaration).type fun _ result =>
      authoredResultSort? result
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

initialize primitiveAttribute : TagAttribute ←
  registerTagAttribute `semantic_primitive "An authored primitive declaration."

private def addSortAlias (full : Name) : CommandElabM Unit := do
  let previous ← getConstInfo full
  let previousSort? ← liftTermElabM <| forallTelescopeReducing previous.type fun _ type =>
    authoredResultSort? type
  if let some previousSort := previousSort? then
    if lastNameComponent full.getPrefix == previousSort.getString! then return
    let alias := full.getPrefix ++ Name.mkSimple previousSort.getString! ++
      Name.mkSimple full.getString!
    unless (← getEnv).contains alias do
      liftCoreM <| addAndCompile <| .defnDecl {
        name := alias, levelParams := previous.levelParams, type := previous.type
        value := mkConst full (previous.levelParams.map Level.param)
        hints := .abbrev, safety := .safe
      }
      for attr in [definitionAttribute, templateAttribute, contextualAuthoringAttribute,
          primitiveAttribute] do
        if attr.hasTag (← getEnv) full then attr.setTag alias
      let splices := (spliceSchemas.getState (← getEnv) |>.find? full).getD #[]
      modifyEnv fun env => spliceSchemas.addEntry env (alias, splices)
      for suffix in [`label, `deed] do
        if let some info := (← getEnv).find? (full ++ suffix) then
          liftCoreM <| addAndCompile <| .defnDecl {
            name := alias ++ suffix, levelParams := info.levelParams, type := info.type
            value := mkConst (full ++ suffix) (info.levelParams.map Level.param)
            hints := .abbrev, safety := .safe
          }
          definitionAttribute.setTag (alias ++ suffix)
      registerExpectedType alias

private def registerSharedAliases (declaration : Name) : CommandElabM Unit := do
  addSortAlias declaration
  let result? ← liftTermElabM <| forallTelescopeReducing (← getConstInfo declaration).type
    fun _ type => authoredResultSort? type
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

private def needsExpectedTypeDispatch (entries : List ExpectedTypeEntry) (bare : Name) : Bool :=
  entries.any (fun e =>
    lastNameComponent e.declaration.getPrefix == e.returnType.getString! ||
    e.declaration.getString! != bare.getString!) ||
    (entries.map (·.returnType)).eraseDups.length > 1

private def isCallableLocal (id : Name) : TermElabM Bool := do
  let some ldecl := (← getLCtx).findFromUserName? id | return false
  if ldecl.isAuxDecl then return false
  return (← whnf ldecl.type).isForall

private def isShadowedByNonCallableLocal (id : Name) : TermElabM Bool := do
  let some ldecl := (← getLCtx).findFromUserName? id | return false
  if ldecl.isAuxDecl then return true
  return !(← whnf ldecl.type).isForall

private def dispatchTarget? (bare sort : Name) (allowSingle : Bool := false) :
    TermElabM (Option Name) := do
  let entries := expectedTypeEntries (← getEnv) bare
  unless allowSingle || needsExpectedTypeDispatch entries bare do return none
  let ns ← getCurrNamespace
  let opens ← getOpenDecls
  let visible := fun (decl : Name) =>
    let decl := (privateToUserName? decl).getD decl
    let scopeName := if lastNameComponent decl.getPrefix == sort.getString! then
        decl.getPrefix.getPrefix else decl.getPrefix
    scopeName.isPrefixOf ns || opens.any (fun
      | .simple n _ => n == scopeName
      | .explicit _ n => n == decl)
  return (entries.find? fun e => e.returnType == sort &&
    visible e.declaration).map (·.declaration)

private def hasOrdinaryNamespaceBinding (stx : Syntax) (expected : Expr)
    (application : Bool := false) : TermElabM Bool := do
  let ns ← getCurrNamespace
  let env ← getEnv
  let candidates ← try resolveGlobalConst stx catch _ => pure []
  for name in candidates do
    let visible := (privateToUserName? name).getD name
    if !visible.getPrefix.isAnonymous && visible.getPrefix.isPrefixOf ns &&
        !definitionAttribute.hasTag env name && !templateAttribute.hasTag env name then
      let saved ← saveState
      let type ← inferType (← mkConstWithFreshMVarLevels name)
      let matchesExpected ← if application && (← whnf type).isForall then pure true
        else isDefEq type expected
      saved.restore
      if matchesExpected then return true
  return false

@[scoped term_elab Lean.Parser.Term.app]
def dispatchOnExpectedType : TermElab := fun stx expected? => do
  unless (← getEnv).contains `Semantics.Color do throwUnsupportedSyntax
  let fn := stx[0]
  unless fn.isIdent do throwUnsupportedSyntax
  if ← isCallableLocal fn.getId then throwUnsupportedSyntax
  let shadowed ← isShadowedByNonCallableLocal fn.getId
  let entries := expectedTypeEntries (← getEnv) fn.getId
  unless shadowed || needsExpectedTypeDispatch entries fn.getId do
    throwUnsupportedSyntax
  let some expected := expected? | throwUnsupportedSyntax
  if ← hasOrdinaryNamespaceBinding fn expected (application := true) then throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  let expectedWhnf ← whnfR expected
  let expectedHead :=
    if expectedWhnf.isAppOfArity ``Option 1 then expectedWhnf.appArg! else expectedWhnf
  let some sort := (← whnfR expectedHead).getAppFn.constName? | throwUnsupportedSyntax
  let exact ← dispatchTarget? fn.getId sort (allowSingle := shadowed)
  let targets ← if let some target := exact then pure [target] else do
    -- A noun phrase may be requested through its Subject coercion.
    let visible ← entries.mapM fun entry =>
      dispatchTarget? fn.getId entry.returnType (allowSingle := shadowed)
    pure (visible.filterMap id).eraseDups
  let some target := targets.head? | throwUnsupportedSyntax
  if fn.getId == target then throwUnsupportedSyntax
  let alternatives ← try resolveGlobalConst fn catch _ => pure []
  for candidate in (targets ++ alternatives).eraseDups do
    let env ← getEnv
    unless definitionAttribute.hasTag env candidate || templateAttribute.hasTag env candidate do
      continue
    let saved ← saveState
    try
      let call := stx.setArg 0 (mkCIdentFrom fn candidate)
      let value ← withMacroExpansion stx call <| Term.elabTerm call expected?
      unless ← MonadLog.hasErrors do return value
    catch _ => pure ()
    saved.restore
  let call := stx.setArg 0 (mkCIdentFrom fn target)
  withMacroExpansion stx call <| Term.elabTerm call expected?

/-- A supplied expression or parameterized body retaining its caller's scope. -/
syntax (name := spliceMode) "splice " term : term

/-- Resolve supplied-body aliases in the declaration's ordinary binder context. -/
elab "caller_splice_type% " type:term : term => do
  let type ← Term.elabType type
  if (← whnf type).isForall then
    return mkForall .anonymous .instImplicit (mkConst ``AuthoringContext) (type.liftLooseBVars 0 1)
  return type

/-- Whether `name` is an authored declaration: a `semantic_definition` or a `semantic_template`
(the generated primitives are templates). -/
def isAuthored (env : Environment) (name : Name) : Bool :=
  definitionAttribute.hasTag env name || templateAttribute.hasTag env name

private def acceptsParameterIdentity (env : Environment) (fn : Expr) : Bool :=
  match fn with
  | .const name _ =>
    isAuthored env name || match env.find? name with
      -- Data constructors (including `Option.some`) store semantic arguments without
      -- inspecting them, just as the semantic syntax constructors do.
      | some (.ctorInfo _) => true
      | _ => false
  | _ => false

-- Keep application order and authored calls intact: reducing here would erase the boundary
-- between a semantic use and an ordinary helper that inspects a parameter's syntax.
private partial def bindParameters (env : Environment) (parameters : List (FVarId × String))
    (body : Expr) (acceptsIdentity : Bool := true) : StateT (List FVarId) TermElabM Expr := do
  match body with
  | .fvar parameter =>
    let some (_, identity) := parameters.find? (·.1 == parameter) | return body
    unless acceptsIdentity do
      throwError "NounPhrase parameter requires a semantic constructor or authored declaration"
    let seen ← get
    let mention := if seen.contains parameter then `Semantics.NounPhrase.laterMention
      else `Semantics.NounPhrase.firstMention
    set (parameter :: seen)
    return mkApp2 (mkConst mention) (mkStrLit identity)
      (mkApp (mkConst `Semantics.NounPhrase.inCaller) body)
  | .app fn arg =>
    if body.isAppOfArity `Semantics.NounPhrase.inCaller 1 && arg.isFVar then
      return body
    if body.isAppOfArity `Semantics.NounPhrase.firstMention 2 ||
        body.isAppOfArity `Semantics.NounPhrase.laterMention 2 ||
        body.isAppOfArity `Semantics.Primitives.NounPhrase.firstMention 2 ||
        body.isAppOfArity `Semantics.Primitives.NounPhrase.laterMention 2 then
      if let .lit (.strVal identity) := body.getAppArgs[0]! then
        if parameters.any (·.2 == identity) then
          throwError "parameter identity is generated: {identity}"
    let acceptsArgument := acceptsParameterIdentity env body.getAppFn
    return .app (← bindParameters env parameters fn acceptsIdentity)
      (← bindParameters env parameters arg acceptsArgument)
  | .lam name type value info =>
    return .lam name type (← bindParameters env parameters value) info
  | .letE name type value rest nondep =>
    return .letE name type (← bindParameters env parameters value)
      (← bindParameters env parameters rest) nondep
  | .mdata data value =>
    return .mdata data (← bindParameters env parameters value acceptsIdentity)
  | .proj name index value =>
    return .proj name index (← bindParameters env parameters value false)
  | _ => return body

/-- Generate a Definition's noun-parameter Binding Identities after ordinary elaboration.
An explicitly written `NounPhrase.inCaller p` opts that occurrence out. -/
elab "bind_parameters% " declaration:str body:term : term <= expected => do
  let value ← Term.elabTermEnsuringType body expected
  Term.synthesizeSyntheticMVarsNoPostponing
  let value ← instantiateMVars value
  let mut parameters := []
  for localDecl in ← getLCtx do
    unless localDecl.isImplementationDetail do
      if (← whnf localDecl.type).isConstOf `Semantics.NounPhrase then
        parameters := parameters ++ [(localDecl.fvarId,
          s!"{declaration.getString}.{localDecl.userName}")]
  return (← (bindParameters (← getEnv) parameters value).run []).1

/-- Declare one named semantic expansion, preserving ordinary Lean binders, with its generated
lexical context and splice schema. The caller tags it a definition or a template. -/
private def elabAuthoredDecl (mods : TSyntax ``Parser.Command.declModifiers)
    (name : TSyntax ``Parser.Command.declId) (signature : TSyntax ``Parser.Command.optDeclSig)
    (value : TSyntax ``Parser.Command.declVal) : CommandElabM Unit := do
    let context ← `(bracketedBinder| [Semantics.AuthoringContext])
    let mut binders := #[context.raw]
    let mut splices : Array (TSyntax `ident) := #[]
    for binder in signature.raw[0].getArgs do
      let mut binder := binder
      if binder[2].getNumArgs == 2 then
        let annotated := binder[2][1]
        if annotated.isOfKind ``spliceMode then
          let resultType : Term := ⟨annotated[1]⟩
          let type := (← `(term| caller_splice_type% $resultType)).raw
          binder := binder.setArg 2 (binder[2].setArg 1 type)
          for identifier in binder[1].getArgs do
            unless identifier.isIdent do throwErrorAt identifier "Authored parameters must be named"
            splices := splices.push ⟨identifier⟩
      binders := binders.push binder
    let signature : TSyntax ``Lean.Parser.Command.optDeclSig :=
      ⟨signature.raw.setArg 0 (mkNullNode binders)⟩
    let mut value := value
    if !splices.isEmpty then
      unless value.raw.isOfKind ``Parser.Command.declValSimple do
        throwErrorAt value "A parameterized template or definition uses `:=` with one expansion body"
      let closeFn := mkIdent `Semantics.CallerSplice.close
      let mut body : Term := ⟨value.raw[1]⟩
      for identifier in splices.reverse do
        body ← `(term| let $identifier := $closeFn $identifier; $body)
      body ← `(term|
        letI : Semantics.AuthoringContext :=
          Semantics.AuthoringContext.child (inferInstance : Semantics.AuthoringContext)
        $body)
      value := ⟨value.raw.setArg 1 body.raw⟩
    elabCommand (← `($mods:declModifiers def $name:declId $signature:optDeclSig $value:declVal))
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    elabCommand (← `(attribute [contextual_authoring] $identifier:ident))
    let declaration ← resolveGlobalConstNoOverload identifier
    modifyEnv fun env => spliceSchemas.addEntry env (declaration, splices.map (·.getId))
    registerExpectedType declaration

/-- The label a keyword ability Definition is written under (`.keyword (.named "kicker") …`), read
from its expansion with its parameters left abstract; `none` for any other Definition. -/
def keywordDefinitionLabel? (name : Name) : MetaM (Option String) := do
  let some value := (← getConstInfo name).value? | return none
  lambdaTelescope value fun _ body => do
    unless (← whnf (← inferType body)).isConstOf `Semantics.Ability do return none
    let body ← whnf body
    unless body.isAppOfArity `Semantics.Ability.keyword 3 do return none
    let label ← whnf body.appFn!.appFn!.appArg!
    unless label.isAppOfArity `Semantics.KeywordLabel.named 1 do return none
    match ← whnf label.appArg! with
    | .lit (.strVal label) => return some label
    | _ => return none

/-- The authored `KeywordLabel` a keyword ability Definition generates: `kicker.label` is the
label `kicker` is written under. A card names a keyword through it (`elabKeywordName`), never by
the internal `KeywordLabel.named`. -/
def keywordLabelConstant (name : Name) : Name := name ++ `label

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
private def checkActionDeed (declaration : Name) (deed : Expr) : MetaM Unit := do
  let env ← getEnv
  for (name, other) in actionDeedsExt.getState env do
    if name != declaration then
      if (← isDefEq deed.appFn!.appArg! other.appFn!.appArg!) &&
          !(← isDefEq deed.appArg! other.appArg!) then
        throwError "Action Definitions for deed {deed.appFn!.appArg!} must agree on DeedPermanent"

/-- The authored `Deed` an Action Definition generates: `exile.deed` is the keyword action
`exile` performs. A card names a deed through it (`elabDeedName`), never by the internal
`Deed.action`. -/
def deedConstant (name : Name) : Name := name ++ `deed

private partial def countFVarOccurrences (env : Environment) (fvar : FVarId) (e : Expr) : Nat := Id.run do
  if let some info := Lean.Meta.getMatcherInfoCore? env (e.getAppFn.constName?.getD .anonymous) then
    let args := e.getAppArgs
    if args.size == info.arity then
      let first := info.getFirstAltPos
      let prefixCount := (args.extract 0 first).foldl (fun n arg => n + countFVarOccurrences env fvar arg) 0
      let branches := (args.extract first args.size).foldl
        (fun n arg => max n (countFVarOccurrences env fvar arg)) 0
      return prefixCount + branches
  return match e with
  | .fvar id => if id == fvar then 1 else 0
  | .app fn arg =>
    if e.isAppOfArity `Semantics.NounPhrase.inCaller 1 && arg.isFVar then 0
    else countFVarOccurrences env fvar fn + countFVarOccurrences env fvar arg
  | .lam _ ty body _ | .forallE _ ty body _ =>
    countFVarOccurrences env fvar ty + countFVarOccurrences env fvar body
  | .letE _ ty val body _ =>
    countFVarOccurrences env fvar ty + countFVarOccurrences env fvar val + countFVarOccurrences env fvar body
  | .mdata _ b | .proj _ _ b => countFVarOccurrences env fvar b
  | _ => 0

private def isTheActorExpr (e : Expr) : Bool :=
  e.isAppOfArity `Semantics.Templates.theActor 1 ||
    e.isAppOfArity `Semantics.Templates.NounPhrase.theActor 1

private def countActorOccurrences (e : Expr) : Nat :=
  if isTheActorExpr e then 1
  else match e with
  | .app fn arg => countActorOccurrences fn + countActorOccurrences arg
  | .lam _ ty body _ | .forallE _ ty body _ =>
    countActorOccurrences ty + countActorOccurrences body
  | .letE _ ty val body _ =>
    countActorOccurrences ty + countActorOccurrences val + countActorOccurrences body
  | .mdata _ b | .proj _ _ b => countActorOccurrences b
  | _ => 0

private partial def rejectGeneratedIdentities (identities : List String) (e : Expr) :
    TermElabM Unit := do
  if [`Semantics.NounPhrase.firstMention, `Semantics.NounPhrase.laterMention,
      `Semantics.Primitives.NounPhrase.firstMention,
      `Semantics.Primitives.NounPhrase.laterMention].any (e.isAppOfArity · 2) then
    if let .lit (.strVal identity) := e.getAppArgs[0]! then
      if identities.contains identity then
        throwError "parameter identity is generated: {identity}"
  match e with
  | .app fn arg => rejectGeneratedIdentities identities fn; rejectGeneratedIdentities identities arg
  | .lam _ _ body _ | .mdata _ body | .proj _ _ body => rejectGeneratedIdentities identities body
  | .letE _ _ value body _ =>
    rejectGeneratedIdentities identities value; rejectGeneratedIdentities identities body
  | _ => pure ()

private partial def rewriteMentions (wrapInCaller : Bool) (actorIdentity? : Option String)
    (parameters : List (FVarId × String)) (e : Expr) (acceptsIdentity : Bool := true) :
    StateT (List String) TermElabM Expr := do
  -- Predicate alternatives are checked in separate Reference Scopes. A mention introduced
  -- in one alternative cannot provide the identity used in another, or after the disjunction.
  if e.getAppFn.constName? ∈ [some `Semantics.Predicate.or,
      some `Semantics.Templates.Predicate.or] then
    let args := e.getAppArgs
    if let some choices := args.back? then
      let rec alternatives (list : Expr) : StateT (List String) TermElabM Expr := do
        if list.isAppOfArity ``List.cons 3 then
          let parts := list.getAppArgs
          let incoming ← get
          let head ← rewriteMentions wrapInCaller actorIdentity? parameters parts[1]!
          set incoming
          let tail ← alternatives parts[2]!
          return mkApp3 list.getAppFn parts[0]! head tail
        return list
      return mkAppN e.getAppFn (args.set! (args.size - 1) (← alternatives choices))
  if let some actorId := actorIdentity? then
    if isTheActorExpr e then
      let seen ← get
      let mention := if seen.contains actorId then `Semantics.NounPhrase.laterMention
        else `Semantics.NounPhrase.firstMention
      set (actorId :: seen)
      return mkApp2 (mkConst mention) (mkStrLit actorId) e
  match e with
  | .fvar id =>
    let some (_, identity) := parameters.find? (·.1 == id) | return e
    unless acceptsIdentity do
      throwError "NounPhrase parameter requires a semantic constructor or authored declaration"
    let seen ← get
    let mention := if seen.contains identity then `Semantics.NounPhrase.laterMention
      else `Semantics.NounPhrase.firstMention
    set (identity :: seen)
    let inner := if wrapInCaller then mkApp (mkConst `Semantics.NounPhrase.inCaller) e else e
    return mkApp2 (mkConst mention) (mkStrLit identity) inner
  | .app fn arg =>
    if [`Semantics.NounPhrase.firstMention, `Semantics.NounPhrase.laterMention,
        `Semantics.Primitives.NounPhrase.firstMention,
        `Semantics.Primitives.NounPhrase.laterMention].any (e.isAppOfArity · 2) then
      if let .lit (.strVal identity) := e.getAppArgs[0]! then
        if parameters.any (·.2 == identity) || actorIdentity? == some identity then
          throwError "parameter identity is generated: {identity}"
    if e.isAppOfArity `Semantics.NounPhrase.inCaller 1 && arg.isFVar then
      return e
    let acceptsArgument := acceptsParameterIdentity (← getEnv) e.getAppFn
    return .app (← rewriteMentions wrapInCaller actorIdentity? parameters fn acceptsIdentity)
      (← rewriteMentions wrapInCaller actorIdentity? parameters arg acceptsArgument)
  | .lam n ty b bi =>
    return .lam n ty (← rewriteMentions wrapInCaller actorIdentity? parameters b) bi
  | .letE n ty v b nd =>
    return .letE n ty (← rewriteMentions wrapInCaller actorIdentity? parameters v)
      (← rewriteMentions wrapInCaller actorIdentity? parameters b) nd
  | .mdata d b =>
    return .mdata d (← rewriteMentions wrapInCaller actorIdentity? parameters b acceptsIdentity)
  | .proj n i b =>
    return .proj n i (← rewriteMentions wrapInCaller actorIdentity? parameters b false)
  | _ => return e

syntax (name := bindDeclBody) "bind_decl_body% " ident str term : term
elab_rules : term <= expected
  | `(bind_decl_body% $isDef:ident $declName:str $body:term) => do
    let isDefinition := isDef.getId.eraseMacroScopes != `false
    let bindsActor := isDef.getId.eraseMacroScopes == `true
    let value ← Term.elabTermEnsuringType body expected
    Term.synthesizeSyntheticMVarsNoPostponing
    let mut value ← instantiateMVars value
    if isDefinition then
      let mut substitutions : Array (FVarId × Expr) := #[]
      for localDecl in ← getLCtx do
        unless localDecl.isImplementationDetail do
          if !(← whnf localDecl.type).isConstOf `Semantics.NounPhrase then
            let classType := mkApp (mkConst `Semantics.CallerScope) localDecl.type
            if let .some _ ← trySynthInstance classType then
              let wrapped ← mkAppM `Semantics.CallerScope.caller #[mkFVar localDecl.fvarId]
              substitutions := substitutions.push (localDecl.fvarId, wrapped)
      value := value.replace fun e =>
        if e.isFVar then (substitutions.find? (·.1 == e.fvarId!)).map (·.2) else none
    let mut reserved := if bindsActor then [s!"{declName.getString}.actor"] else []
    let mut npParams : List (FVarId × String) := []
    for localDecl in ← getLCtx do
      unless localDecl.isImplementationDetail do
        if (← whnf localDecl.type).isConstOf `Semantics.NounPhrase then
          let idStr := s!"{declName.getString}.{localDecl.userName}"
          reserved := idStr :: reserved
          if isDefinition || countFVarOccurrences (← getEnv) localDecl.fvarId value ≥ 2 then
            npParams := npParams ++ [(localDecl.fvarId, idStr)]
    rejectGeneratedIdentities reserved value
    let actorId? :=
      if bindsActor && countActorOccurrences value ≥ 2 then
        some s!"{declName.getString}.actor"
      else none
    return (← (rewriteMentions isDefinition actorId? npParams value).run []).1

syntax (name := semanticDefinitionDecl) declModifiers "semantic_definition " declId
  ppIndent(optDeclSig) declVal : command

syntax (name := semanticTemplateDecl) declModifiers "semantic_template " declId
  ppIndent(optDeclSig) declVal : command

elab_rules : command
  | `($mods:declModifiers semantic_definition $name:declId
      $signature:optDeclSig $value:declVal) => do
    unless value.raw.isOfKind ``Parser.Command.declValSimple do
      throwErrorAt value "A Definition uses `:=` with one expansion body"
    let declarationName := (⟨name.raw[0]⟩ : TSyntax `ident).getId
    let identityPrefix := Syntax.mkStrLit declarationName.getString!
    let body : Term := ⟨value.raw[1]⟩
    let body ← if body.raw.getKind == ``bindDeclBody then
      pure body
    else `(term| bind_parameters% $identityPrefix $body)
    let value := ⟨value.raw.setArg 1 body.raw⟩
    elabAuthoredDecl mods name signature value
    -- A failed body may have recovered to `sorry`; do not diagnose its missing scope too.
    if ← MonadLog.hasErrors then return
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    let declaration ← resolveGlobalConstNoOverload identifier
    liftTermElabM do
      let info ← getConstInfo declaration
      if let .defnInfo definition := info then
        lambdaTelescope definition.value fun _ body => do
          if (← whnf (← inferType body)).isConstOf `Semantics.Action then
            unless (← whnf body).getAppFn.isConstOf `Semantics.Action.ownScope do
              throwError "An Action Definition must establish its own Reference Scope"
    elabCommand (← `(attribute [«semantic_definition»] $identifier:ident))
    if let some label ← liftTermElabM (keywordDefinitionLabel? declaration) then
      let labelName := keywordLabelConstant declaration
      unless (← getEnv).contains labelName do liftCoreM <| addAndCompile <| .defnDecl {
        name := labelName
        levelParams := []
        type := mkConst `Semantics.KeywordLabel
        value := mkApp (mkConst `Semantics.KeywordLabel.named) (mkStrLit label)
        hints := .regular 0
        safety := .safe
      }
      definitionAttribute.setTag labelName
    if let some deed ← liftTermElabM (actionDefinitionDeed? declaration) then
      liftTermElabM <| checkActionDeed declaration deed
      modifyEnv fun env => actionDeedsExt.addEntry env (declaration, deed)
      let deedName := deedConstant declaration
      liftCoreM <| addAndCompile <| .defnDecl {
        name := deedName
        levelParams := []
        type := mkConst `Semantics.Deed
        value := deed
        hints := .regular 0
        safety := .safe
      }
      definitionAttribute.setTag deedName
    registerSharedAliases declaration
  | `($mods:declModifiers semantic_template $name:declId
      $signature:optDeclSig $value:declVal) => do
    elabAuthoredDecl mods name signature value
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    elabCommand (← `(attribute [«semantic_template»] $identifier:ident))
    registerSharedAliases (← resolveGlobalConstNoOverload identifier)

/-- Every exposed primitive has a named declaration to which a spelling can attach.
Internal expansion carriers are deliberately absent from this generated interface. -/
elab "declare_semantic_primitives" : command => do
  let env ← getEnv
  let constructors := env.constants.toList.filterMap fun (name, info) =>
    match info with
    | .ctorInfo ctor => some (name, ctor)
    | _ => none
  for (name, ctor) in constructors.mergeSort (fun a b => a.2.numFields ≤ b.2.numFields) do
      if expressionAttribute.hasTag env ctor.induct &&
          (!internalAttribute.hasTag env name ||
            name ∈ [`Semantics.NounPhrase.pro, `Semantics.NounPhrase.which]) &&
          !internalAttribute.hasTag env ctor.induct then
        let publicName := name.replacePrefix `Semantics `Semantics.Primitives
        let current ← getEnv
        let type := ctor.type.replace fun expr =>
          match expr with
          | .const original levels =>
            let primitiveName := original.replacePrefix `Semantics `Semantics.Primitives
            if isAuthored current primitiveName then some (mkConst primitiveName levels)
            else none
          | _ => none
        let declaration : Declaration := .defnDecl {
          name := publicName
          levelParams := ctor.levelParams
          type := type
          value := mkConst name (ctor.levelParams.map Level.param)
          hints := .regular 0
          safety := .safe
        }
        liftCoreM <| addAndCompile declaration
        templateAttribute.setTag publicName

/-- Semantic content includes expressions nested inside otherwise ordinary data records. -/
partial def containsExpression (env : Environment) (type : Expr)
    (visited : List Name := []) : Bool :=
  match type with
  | .const name _ =>
    if expressionAttribute.hasTag env name then true
    else if visited.contains name then false
    else match env.find? name with
      | some (.inductInfo info) => info.ctors.any fun ctor =>
        match env.find? ctor with
        | some (.ctorInfo info) => containsExpression env info.type (name :: visited)
        | _ => false
      | some (.defnInfo info) => containsExpression env info.value (name :: visited)
      | _ => false
  | .app fn arg => containsExpression env fn visited || containsExpression env arg visited
  | .forallE _ domain body _ | .lam _ domain body _ =>
    containsExpression env domain visited || containsExpression env body visited
  | .letE _ type value body _ => containsExpression env type visited ||
    containsExpression env value visited || containsExpression env body visited
  | .mdata _ value | .proj _ _ value => containsExpression env value visited
  | _ => false

/-- Grammar modules classify their non-record, non-enumeration syntax uniformly. -/
elab "classify_semantic_syntax" : command => do
  let env ← getEnv
  for (name, info) in env.constants.toList do
    if (env.getModuleIdxFor? name).isNone && !isStructure env name then
      if let .inductInfo induct := info then
        let hasFields := induct.ctors.any fun ctor =>
          match env.find? ctor with
          | some (.ctorInfo ci) => ci.numFields > 0
          | _ => false
        if hasFields && !expressionAttribute.hasTag env name then
          expressionAttribute.setTag name

/-- Ordinary helper bodies can be skipped only after checking the reachable definitions
of their instantiated expansion. Semantic types matter too: they can hide lambda parameters
inside containers or discarded values. Opaque code remains subject to ordinary inspection. -/
private partial def helperNeedsInspection (env : Environment) (body : Expr)
    (clean : IO.Ref (Std.HashSet Name)) : IO Bool := do
  let known ← clean.get
  let rec visit (pending : List Name) (seen : Std.HashSet Name) : Bool × Std.HashSet Name :=
    match pending with
    | [] => (false, seen)
    | name :: rest =>
      if known.contains name || seen.contains name then visit rest seen
      else
        let seen := seen.insert name
        if isAuthored env name || expressionAttribute.hasTag env name then (true, seen)
        else match env.find? name with
          | some (.ctorInfo ctor) =>
            if expressionAttribute.hasTag env ctor.induct && !literalAttribute.hasTag env name then
              (true, seen)
            else visit rest seen
          | some (.defnInfo info) =>
            visit (info.value.foldConsts rest fun name names => name :: names) seen
          | some (.axiomInfo _) | some (.opaqueInfo _) => (true, seen)
          | _ => visit rest seen
  let (needed, seen) := visit (body.foldConsts [] fun name names => name :: names) {}
  unless needed do clean.modify fun previous => seen.fold (fun acc name => acc.insert name) previous
  return needed

private structure InspectionKey where
  term : Expr
  visited : List Name
  context : Expr
  parameters : List FVarId
  bodyParameter : Bool
  deriving BEq, Hashable

private abbrev InspectionCache := Std.HashMap InspectionKey Form

private partial def inspectCore (cache : IO.Ref InspectionCache)
    (clean : IO.Ref (Std.HashSet Name)) (term : Expr) (visited : List Name) (context : Expr) (parameters : List FVarId)
    (bodyParameter : Bool) : MetaM Form := do
  let term := (← instantiateMVars term).consumeMData
  let key : InspectionKey := ⟨term, visited, context, parameters, bodyParameter⟩
  if let some form := (← cache.get)[key]? then return form
  let form ← do
    if (← isProof term) || (← isType term) then return .data []
    match term with
    | .lit value => return .literal value
    | .fvar id =>
      if parameters.contains id then return .parameter (← id.getDecl).userName
      if containsExpression (← getEnv) (← inferType term) then
        throwError "Semantic parameter {term} has no authoring evidence"
      return .data []
    | .lam name type body info =>
      withLocalDecl name info type fun arg => do
        if bodyParameter && (← isDefEq type (mkConst ``AuthoringContext)) then
          return ← inspectCore cache clean (body.instantiate1 arg) visited arg parameters true
        -- A supplied body binds a parameter of any type holding semantic content: a noun
        -- phrase, an ability, or a record of them (`chooseModesOver`'s frame over its items).
        if bodyParameter && containsExpression (← getEnv) type then
          return .binder name (← inspectCore cache clean (body.instantiate1 arg) visited context
            (arg.fvarId! :: parameters) true)
        -- A binder of an ordinary record that only holds semantic expressions (a `KeywordTerm`,
        -- holding a `KeywordLabel`) is filled by the template the function is passed to, from
        -- arguments inspected where they are written: an applied function is beta-reduced
        -- first. Such a record has no Reference Scope to close, so unlike a semantic
        -- expression's binder (`NounPhrase`) it needs no supplied body.
        let env ← getEnv
        if let .const typeName _ := (← whnf type).getAppFn then
          if !expressionAttribute.hasTag env typeName && containsExpression env type then
            return .binder name (← inspectCore cache clean (body.instantiate1 arg) visited context
              (arg.fvarId! :: parameters) bodyParameter)
        inspectCore cache clean (body.instantiate1 arg) visited context parameters bodyParameter
    | .letE _ _ value body _ =>
      return .data [← inspectCore cache clean value visited context parameters bodyParameter,
        ← inspectCore cache clean (body.instantiate1 value) visited context parameters bodyParameter]
    | .proj _ _ object => inspectCore cache clean object visited context parameters bodyParameter
    | _ =>
      let fn := term.getAppFn
      let args := term.getAppArgs
      match fn with
      | .const name levels =>
        let env ← getEnv
        let info ← getConstInfo name
        -- Core coercions predate the bundle. Their authoring evidence is the corresponding
        -- public primitive call, checked for definitional equality before recording it.
        let coercion := (privateToUserName? name).getD name
        let coercions : List (Name × Name) := [
          (`Semantics.Templates.coeSubtypePredicate, `Semantics.Templates.hasSubtype),
          (`Semantics.Templates.coeManaCostCost, `Semantics.Templates.mana),
          (`Semantics.Templates.coeActionCost, `Semantics.Templates.perform),
          (`Semantics.Templates.coeCharacteristicBundleTokenSpec, `Semantics.Templates.TokenSpec.written),
          (`Semantics.Templates.coeCounterKindCounterKindSource, `Semantics.Templates.printedKind),
          (`Semantics.Templates.coeStringNameSource, `Semantics.Templates.printedName),
          (`Semantics.Templates.coeConditionAsThough, `Semantics.Templates.holds),
          (`Semantics.Templates.coeStaticSpecAbility, `Semantics.Templates.static)]
        if let some target := coercions.lookup coercion then
          if env.contains target then
            let expanded := mkAppN (mkConst target) (#[context] ++ args)
            unless ← isDefEq term expanded do
              throwError "Core coercion {name} disagrees with authored primitive {target}"
            return ← inspectCore cache clean expanded visited context parameters bodyParameter
        if isAuthored env name then
          let mut arguments := []
          let splices := (spliceSchemas.getState env |>.find? name).getD #[]
          let mut type := info.type.instantiateLevelParams info.levelParams levels
          for arg in args do
            let mut isBody := false
            if let .forallE parameter _ next _ := ← whnf type then
              type := next.instantiate1 arg
              isBody := splices.contains parameter.eraseMacroScopes
            if contextualAuthoringAttribute.hasTag env name &&
                (← isDefEq (← inferType arg) (mkConst ``AuthoringContext)) then
              unless ← isDefEq arg context do
                throwError "{name} uses a lexical context outside its authoring scope"
            else
              arguments := arguments ++ [← inspectCore cache clean arg visited context parameters isBody]
          return .call name arguments
        match info with
        | .ctorInfo ctor =>
          if expressionAttribute.hasTag env ctor.induct && !literalAttribute.hasTag env name then
            return .raw name
          return .constructor name (← args.toList.mapM fun arg =>
            inspectCore cache clean arg visited context parameters bodyParameter)
        | .defnInfo defn =>
          let arguments ← args.toList.mapM fun arg =>
            inspectCore cache clean arg visited context parameters bodyParameter
          if visited.contains name then return .data arguments
          let resultType ← whnf (← inferType term)
          let semanticFunction := resultType.isForall && containsExpression env resultType
          let expansion := (defn.value.instantiateLevelParams defn.levelParams levels).beta args
          if !semanticFunction && !(← helperNeedsInspection env expansion clean) then
            return .data arguments
          let body ← inspectCore cache clean expansion
            (name :: visited) context parameters bodyParameter
          return .data (body :: arguments)
        | _ =>
          if containsExpression env (← inferType term) then
            throwError "Definition {name} has no authoring evidence"
          return .data (← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter)
      | _ =>
        if args.isEmpty then throwError "Cannot establish authorship for {term}"
        let arguments ← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter
        let body ← inspectCore cache clean (if fn.isLambda then fn.beta args else fn) visited context parameters bodyParameter
        return .data (body :: arguments)
  cache.modify (·.insert key form)
  return form

/-- Cache only identical inspections, including lexical context, permitted parameters,
and the unfolding path. Each occurrence still retains its own place in the authoring tree. -/
def inspect (term : Expr) (visited : List Name := [])
    (context : Expr := mkConst ``AuthoringContext.root) (parameters : List FVarId := [])
    (bodyParameter : Bool := false) : MetaM Form := do
  let cache ← IO.mkRef ({} : InspectionCache)
  let clean ← IO.mkRef ({} : Std.HashSet Name)
  inspectCore cache clean term visited context parameters bodyParameter

private def resolveKeywordLabelConst? (stx : Syntax) : TermElabM (Option Name) := do
  let env ← getEnv
  let localLabel := (← getCurrNamespace) ++ stx.getId ++ `label
  if env.contains localLabel then
    return some localLabel
  let sortedLabel := (← getCurrNamespace) ++ `Ability ++ stx.getId ++ `label
  if env.contains sortedLabel then return some sortedLabel
  if let some target ← dispatchTarget? stx.getId.eraseMacroScopes `Semantics.Ability then
    if env.contains (target ++ `label) then return some (target ++ `label)
  let candidates ← try resolveGlobalConst stx catch _ => pure []
  let some defn := candidates.find? (Authoring.definitionAttribute.hasTag env) | return none
  let lbl := Authoring.keywordLabelConstant defn
  return if env.contains lbl then some lbl else none

@[scoped term_elab ident] def elabBareIdentDispatch : TermElab := fun stx expected? => do
  unless (← getEnv).contains `Semantics.Color do throwUnsupportedSyntax
  let some expected := expected? | throwUnsupportedSyntax
  if let some ldecl := (← getLCtx).findFromUserName? stx.getId then
    unless ldecl.isAuxDecl do throwUnsupportedSyntax
  let entries := expectedTypeEntries (← getEnv) stx.getId
  let inTable := needsExpectedTypeDispatch entries stx.getId
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then
    if inTable || (← resolveKeywordLabelConst? stx).isSome then
      tryPostpone
    throwUnsupportedSyntax
  let expectedWhnf ← whnfR expected
  let (expectedHead, isOption) ←
    if expectedWhnf.isAppOfArity ``Option 1 then
      pure (expectedWhnf.appArg!, true)
    else if expectedWhnf.isForall then
      forallTelescopeReducing expectedWhnf fun _ body => pure (body, false)
    else
      pure (expectedWhnf, false)
  let some sort := (← whnfR expectedHead).getAppFn.constName? | throwUnsupportedSyntax
  let kwLabel? ← if sort == `Semantics.KeywordLabel || sort == `Semantics.KeywordTerm then
      resolveKeywordLabelConst? stx
    else pure none
  if !inTable && kwLabel?.isNone then
    throwUnsupportedSyntax
  if ← hasOrdinaryNamespaceBinding stx expected then throwUnsupportedSyntax
  -- All result sorts use the declaration registry.
  if let some targetDecl ← dispatchTarget? stx.getId sort then
    if stx.getId == targetDecl then throwUnsupportedSyntax
    let term ← if isOption then `(term| some $(mkCIdentFrom stx targetDecl))
      else pure (mkCIdentFrom stx targetDecl)
    return ← withMacroExpansion stx term <| elabTerm term (some expected)
  -- Labels are available even while their keyword body is being elaborated.
  if sort == `Semantics.KeywordLabel || sort == `Semantics.KeywordTerm then
    if let some labelConst := kwLabel? then
      let term ←
        if sort == `Semantics.KeywordLabel then
          if isOption then `(term| some $(mkCIdentFrom stx labelConst))
          else pure (mkCIdentFrom stx labelConst)
        else
          if isOption then
            `(term| some ($(mkIdent `Semantics.Templates.theKeyword) $(mkCIdentFrom stx labelConst)))
          else `(term| $(mkIdent `Semantics.Templates.theKeyword) $(mkCIdentFrom stx labelConst))
      return ← withMacroExpansion stx term <| elabTerm term (some expected)
  throwUnsupportedSyntax

private def elabClosedWord (application : Bool) : TermElab := fun stx expected? => do
  unless (← getEnv).contains `Semantics.Color do throwUnsupportedSyntax
  let fn := if application then stx[0] else stx
  unless fn.isIdent && fn.getId.getPrefix.isAnonymous do throwUnsupportedSyntax
  if fn.getId.eraseMacroScopes ∈ [`some, `none] then throwUnsupportedSyntax
  let some expected := expected? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then throwUnsupportedSyntax
  let expectedWhnf := expected
  let isOption := expectedWhnf.isAppOfArity ``Option 1
  let inner := if isOption then expectedWhnf.appArg! else expectedWhnf
  let some sort := inner.getAppFn.constName? | throwUnsupportedSyntax
  let env ← getEnv
  unless (`Semantics).isPrefixOf sort && sort != `Semantics.Card do throwUnsupportedSyntax
  let ctor := sort ++ fn.getId.eraseMacroScopes
  let some (.ctorInfo _) := env.find? ctor | throwUnsupportedSyntax
  if expressionAttribute.hasTag env sort || internalAttribute.hasTag env sort then
    throwUnsupportedSyntax
  if let some localDecl := (← getLCtx).findFromUserName? fn.getId then
    unless localDecl.isAuxDecl do
      let saved ← saveState
      let compatible ← isDefEq localDecl.type expected <||> isDefEq localDecl.type inner
      saved.restore
      if compatible || (application && (← whnf localDecl.type).isForall) then
        throwUnsupportedSyntax
  if ← hasOrdinaryNamespaceBinding fn expected application then throwUnsupportedSyntax
  if let some target ← dispatchTarget? fn.getId sort (allowSingle := true) then
    if env.contains target then throwUnsupportedSyntax
  let call := if application then stx.setArg 0 (mkCIdentFrom fn ctor)
    else mkCIdentFrom fn ctor
  let call ← if isOption then `(term| some $(⟨call⟩)) else pure ⟨call⟩
  withMacroExpansion stx call <| elabTerm call expected?

@[scoped term_elab ident]
def elabClosedWordIdent : TermElab := elabClosedWord false

@[scoped term_elab Lean.Parser.Term.app]
def elabClosedWordApp : TermElab := elabClosedWord true

/-- Top-down structure literal elaborator: allows writing `{ subtypes := [subtype] }` directly
where `CharacteristicBundle` is expected. -/
@[scoped term_elab Lean.Parser.Term.structInst] def elabCharacteristicBundleStruct : TermElab :=
    fun stx expected? => do
  unless (← getEnv).contains `Semantics.Color do throwUnsupportedSyntax
  let some expected := expected? | throwUnsupportedSyntax
  unless stx[1].isNone do throwUnsupportedSyntax
  let fields := stx[2][0].getSepArgs
  if fields.any (fun field => field[0][0].getId.eraseMacroScopes ∈ [`characteristics, `qualities]) then
    throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  let expectedWhnf ← whnfR expected
  let structTerm : Term := ⟨stx⟩
  let expanded ← if expectedWhnf.isConstOf `Semantics.CharacteristicBundle then
      `(term| ({ characteristics := $structTerm } : $(mkIdent `Semantics.CharacteristicBundle)))
    else if expectedWhnf.isConstOf `Semantics.CardFace then
      `(term| ({ characteristics := $structTerm } : $(mkIdent `Semantics.CardFace)))
    else if expectedWhnf.isConstOf `Semantics.Card then
      `(term| $(mkIdent `Semantics.Card.singleFaced) { characteristics := $structTerm })
    else throwUnsupportedSyntax
  withMacroExpansion stx expanded <| elabTerm expanded (some expected)


/-- Top-down list literal elaborator (`Action` → `sequentially`, `Predicate` → `Predicate.and`,
`Condition` → `Condition.and`, `StaticSpec` → `conjunction`, `Cost` → `mana` or `compound`,
`Option (List α)` → `some [...]`). Intercepts `List.cons` after `«term[_]»` macro expansion so
expected element types flow top-down into every list element! -/
@[scoped term_elab Lean.Parser.Term.app] def elabDomainList : TermElab := fun stx expected? => do
  unless (← getEnv).contains `Semantics.Color do throwUnsupportedSyntax
  let fn := if stx.isIdent then stx else stx[0]
  unless fn.isIdent && fn.getId.eraseMacroScopes ∈ [``List.cons, ``List.nil] do
    throwUnsupportedSyntax
  let some expected := expected? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  let expectedWhnf ← whnfR expected
  let listTerm : Term := ⟨stx⟩
  if expectedWhnf.isAppOfArity ``Option 1 then
    let inner ← whnfR expectedWhnf.appArg!
    if inner.isAppOfArity ``List 1 ||
        inner.getAppFn.constName? ∈ [some `Semantics.Action, some `Semantics.Predicate,
          some `Semantics.Condition, some `Semantics.StaticSpec, some `Semantics.Cost] then
      let expanded ← `(term| some $listTerm)
      return ← withMacroExpansion stx expanded <| elabTerm expanded (some expected)
  let some sort := expectedWhnf.getAppFn.constName? | throwUnsupportedSyntax
  match sort with
  | `Semantics.Action =>
    let expanded ← `(term| $(mkIdent `Semantics.Templates.sequentially) $listTerm)
    withMacroExpansion stx expanded <| elabTerm expanded (some expected)
  | `Semantics.Predicate =>
    let expanded ← `(term| $(mkIdent `Semantics.Templates.Predicate.and) $listTerm)
    withMacroExpansion stx expanded <| elabTerm expanded (some expected)
  | `Semantics.Condition =>
    let expanded ← `(term| $(mkIdent `Semantics.Templates.Condition.and) $listTerm)
    withMacroExpansion stx expanded <| elabTerm expanded (some expected)
  | `Semantics.StaticSpec =>
    let expanded ← `(term| $(mkIdent `Semantics.Templates.conjunction) $listTerm)
    withMacroExpansion stx expanded <| elabTerm expanded (some expected)
  | `Semantics.Cost =>
    let saved ← saveState
    try
      let manaExpanded ← `(term| $(mkIdent `Semantics.Templates.mana) $listTerm)
      let res ← withMacroExpansion stx manaExpanded <| elabTerm manaExpanded (some expected)
      Term.synthesizeSyntheticMVarsNoPostponing
      if ← MonadLog.hasErrors then
        restoreState saved
        let compExpanded ← `(term| $(mkIdent `Semantics.Templates.compound) $listTerm)
        withMacroExpansion stx compExpanded <| elabTerm compExpanded (some expected)
      else
        return res
    catch _ =>
      restoreState saved
      let compExpanded ← `(term| $(mkIdent `Semantics.Templates.compound) $listTerm)
      withMacroExpansion stx compExpanded <| elabTerm compExpanded (some expected)
  | _ =>
    throwUnsupportedSyntax


@[scoped term_elab ident] def elabEmptyDomainList : TermElab := elabDomainList

syntax (name := keywordArguments) "keyword_arguments% " "[" term,* "]" : term
elab_rules : term
  | `(keyword_arguments% [$arguments,*]) => do
    let mut values := #[]
    for argument in arguments.getElems do
      let value ← elabTerm argument none
      let type ← whnf (← inferType value)
      if type.isConstOf `Semantics.NounPhrase || type.isConstOf `Semantics.Action then continue
      let ctor ← match type.getAppFn.constName? with
        | some `Semantics.Cost => pure `cost
        | some `Semantics.Amount => pure `number
        | some `Semantics.Predicate => pure <|
          if argument.raw.getId.eraseMacroScopes == `subject then `subject else `quality
        | some `Semantics.DeckCondition => pure `deckCondition
        | some `Semantics.Ability => pure `ability
        | _ => throwErrorAt argument "No KeywordParam constructor for {type}"
      values := values.push (← mkAppM (`Semantics.KeywordParam ++ ctor) #[value])
    mkListLit (mkConst `Semantics.KeywordParam) values.toList

elab "ability_word_body% " label:str body:term : term <= expected => do
  let value ← elabTermEnsuringType body expected
  let reduced ← whnf value
  if reduced.isAppOfArity `Semantics.Ability.italicHead 2 &&
      reduced.appFn!.appArg!.isAppOfArity `Semantics.ItalicWord.abilityWord 1 then return value
  let label := mkStrLit label.getString
  mkAppM `Semantics.Templates.abilityWord
    #[mkApp (mkConst `Semantics.AbilityWordLabel.named) label, value]


syntax declField := ident " := " term
syntax declBlock := "{" withoutPosition(sepBy(declField, ",", ", ", allowTrailingSep)) "}"
syntax (name := authoredDeclCmd) declModifiers ident ident ppIndent(optDeclSig)
  " := " (declBlock <|> term) : command

syntax (name := anonymousCardCmd) (priority := high) declModifiers ident " := " term : command
syntax (name := cardNamedTerm) "cardNamed " str : term
syntax (name := cardNamedOkTerm) "cardNamedOk " str : term

private def resolvePrintedCard (key : String) : TermElabM Name := do
  let env ← getEnv
  let bare := Name.mkSimple key
  let ns ← getCurrNamespace
  let opens ← getOpenDecls
  let candidates := [ns ++ bare, ns ++ `Spelled ++ bare] ++ (opens.filterMap fun
    | .simple n _ => some (n ++ bare)
    | .explicit alias n => if alias == bare then some n else none) ++
    [`Semantics.Cards ++ bare, `Semantics.Cards.Spelled ++ bare]
  for name in candidates.eraseDups do
    if let some info := env.find? name then
      if (← whnfR info.type).isConstOf `Semantics.Spelled then return name
  throwError "Unknown card {key}"

@[term_elab cardNamedTerm]
def elabCardNamed : TermElab := fun stx expected? => do
  let name ← resolvePrintedCard (stx[1].isStrLit?.getD "")
  let value := Lean.mkConst name
  if let some expected := expected? then
    if (← whnfR (← instantiateMVars expected)).isConstOf `Semantics.Card then
      return mkApp (mkConst `Semantics.Spelled.card) value
  return value

@[term_elab cardNamedOkTerm]
def elabCardNamedOk : TermElab := fun stx _ => do
  let name ← resolvePrintedCard (stx[1].isStrLit?.getD "")
  return mkConst (name.getPrefix ++ Name.mkSimple ("ok_" ++ name.getString!))

private def cardRecordFields (stx : Syntax) : Except String (List (Name × Syntax)) := do
  if let `(term| {}) := stx then return []
  unless stx.isOfKind ``Parser.Term.structInst && stx[1].isNone do
    throw "Anonymous card requires a named card record"
  let mut fields := []
  for field in stx[2][0].getSepArgs do
    let name := field[0][0].getId.eraseMacroScopes
    if fields.any (·.1 == name) then throw s!"Duplicate card field {name}"
    if name == `body then throw "Anonymous card does not accept body :="
    if field[1].isNone then throw s!"Card field {name} requires :="
    fields := fields ++ [(name, field[1][2][2])]
  return fields

@[scoped term_elab Lean.Parser.Term.app]
def elabCardRecord : TermElab := fun stx expected? => do
  let some expected := expected? | throwUnsupportedSyntax
  unless (← whnfR (← instantiateMVars expected)).isConstOf `Semantics.Card do
    throwUnsupportedSyntax
  let fn := stx[0]
  let name := if fn.isOfKind ``Parser.Term.dotIdent then fn[1].getId else fn.getId
  unless name ∈ [`transforming, `modalDfc, `split, `sharedLineSplit, `adventurer,
      `flip, `leveler, `prototype] && stx[1].getArgs.size == 1 do throwUnsupportedSyntax
  let record := stx[1][0]
  unless record.isOfKind ``Parser.Term.structInst do throwUnsupportedSyntax
  let fields ← match cardRecordFields record with
    | .ok fields => pure fields
    | .error message => throwErrorAt record message
  let ctor := `Semantics.Card ++ name
  let info ← getConstInfo ctor
  let args ← forallTelescopeReducing info.type fun params _ => do
    let mut remaining := fields
    let mut args : Array Term := #[]
    for param in params do
      let field := (← param.fvarId!.getDecl).userName
      let some value := remaining.lookup field | throwErrorAt record "Missing card field {field}"
      args := args.push ⟨value⟩
      remaining := remaining.filter (·.1 != field)
    unless remaining.isEmpty do throwErrorAt record "Unknown card field {remaining.head!.1}"
    return args
  let expanded ← `(term| $(mkCIdent ctor) $args*)
  withMacroExpansion stx expanded <| elabTerm expanded expected?

private partial def printedCardName (stx : Syntax) : Except String String := do
  if stx.isOfKind ``Parser.Term.paren then return ← printedCardName stx[1]
  let (record, parts) ← if stx.isOfKind ``Parser.Term.app then do
    let fn := stx[0]
    let ctor := if fn.isOfKind ``Parser.Term.dotIdent then fn[1].getId else fn.getId
    let parts ← match ctor with
      | `transforming | `modalDfc => pure [`front, `back]
      | `split | `sharedLineSplit => pure [`left, `right]
      | `adventurer => pure [`normal, `adventure]
      | `flip => pure [`normal, `alternative]
      | `leveler | `prototype => pure [`inner]
      | _ => throw "Expected a card constructor"
    unless stx[1].getArgs.size == 1 do throw "Expected named card parts"
    pure (stx[1][0], parts)
    else pure (stx, [])
  let fields ← cardRecordFields record
  if !parts.isEmpty then
    let names ← parts.mapM fun part => do
      let some value := fields.lookup part | throw s!"Missing card part {part}"
      printedCardName value
    return " // ".intercalate names
  let some name := fields.lookup `name | throw "Anonymous card requires a printed name"
  if let some name := name.isStrLit? then return name
  if let `(term| some $value:str) := name then return value.getString
  throw "Anonymous card requires a printed name"

private def namedCardSyntax (stx : Syntax) : Except String Syntax := do
  unless stx[1].getId == `card do throw "Only card accepts an anonymous declaration"
  let name ← printedCardName stx[3]
  return mkNode ``authoredDeclCmd #[stx[0],
    mkIdent `card, mkIdent (Name.mkSimple name),
    mkNode ``Parser.Command.optDeclSig #[mkNullNode #[], mkNullNode #[]],
    mkAtom ":=", stx[3]]

private def commandReturnType? (kind : Name) : Option Name :=
  match kind with
  | `action => some `Action | `keyword | `ability_word => some `Ability
  | `counter => some `CounterKind | `subtype => some `Subtype
  | `designation => some `DesignationLabel | `card_type => some `CardType
  | `relation => some `RelationLabel | `token => some `PredefinedToken
  | `sba => some `SbaRule | `conferral => some `ConferralRule | `card => some `Spelled
  | _ => none

private def authoredKinds : List Name :=
  [`primitive, `template, `action, `keyword, `ability_word, `counter, `designation,
   `subtype, `card_type, `relation, `token, `sba, `conferral, `card]

private def lexicalFields : List Name :=
  [`lexical, `spelling]

private def semanticFields : Name → List Name
  | `primitive | `template => [`body]
  | `ability_word => [`body, `label]
  | `keyword => [`body, `label, `cost, `reminder]
  | `action => [`body, `deed, `permanent, `ofAbility]
  | `counter => [`holder, `confers, `rules]
  | `subtype => [`kind, `name, `confers, `rules]
  | `designation => [`scope, `sole, `confers, `rules]
  | `card_type => [`name, `cardClass, `stats, `confers, `rules]
  | `relation => []
  | `token => [`name, `token, `body]
  | `sba => [`scope, `when, `then_, `body]
  | `conferral => [`scope, `confer, `body]
  | `card => [`name, `cost, `colors, `supertypes, `types, `subtypes, `text,
              `power, `toughness, `loyalty, `defense]
  | _ => []

private def blockFields (block : Syntax) : CommandElabM (List (Name × Term)) := do
  let mut fields := []
  for field in block[1].getSepArgs do
    let key := field[0].getId.eraseMacroScopes
    if fields.any (·.1 == key) then throwErrorAt field "Duplicate declaration field {key}"
    if key ∈ [`rule, `parameters] then throwErrorAt field "Unsupported declaration field {key}"
    fields := fields ++ [(key, ⟨field[2]⟩)]
  return fields

private def makeStruct (fields : List (Name × Term)) : CommandElabM Term := do
  let fs ← fields.toArray.mapM fun (key, value) =>
    `(Parser.Term.structInstField| $(mkIdent key):ident := $value)
  `(term| { $fs:structInstField,* })

private def generatedCheck (name : Ident) (sort : Name) : CommandElabM Unit := do
  let ok := mkIdent (name.getId.getPrefix ++ Name.mkSimple ("ok_" ++ name.getId.getString!))
  let check := mkIdent (`Semantics ++ sort ++ `check)
  elabCommand (← `(theorem $ok : $check $name = [] := by decide))

private def registerKeywordLabel (name : Name) (label : Term) : CommandElabM Unit := do
  let value ← liftTermElabM <| elabTermEnsuringType label (mkConst ``String)
  let value := mkApp (mkConst `Semantics.KeywordLabel.named) value
  if (← getEnv).contains name then
    unless ← liftTermElabM <| isDefEq (mkConst name) value do
      throwError "Conflicting keyword label {name}"
  else
    liftCoreM <| addAndCompile <| .defnDecl {
      name, levelParams := [], type := mkConst `Semantics.KeywordLabel
      value, hints := .regular 0, safety := .safe
    }
  definitionAttribute.setTag name

private def registryKinds : List Name :=
  [`counter, `subtype, `designation, `card_type, `relation]

initialize bundleRegistryAttribute : TagAttribute ←
  registerTagAttribute `bundle_registry "A catalog value registered before its bundle rules."

/-- Check the author's elaborated term before command-generated constructors are introduced.
Closed word tags are data; exposed AST constructors require a primitive declaration. -/
elab "primitive_boundary% " body:term : term <= expected => do
  let value ← elabTermEnsuringType body expected
  synthesizeSyntheticMVarsNoPostponing
  let value ← instantiateMVars value
  let env ← getEnv
  for name in value.foldConsts [] (fun name names => name :: names) do
    let exposed := (`Semantics).isPrefixOf name && env.contains (name.replacePrefix `Semantics `Semantics.Primitives)
    let raw ← match env.find? name with
      | some (.ctorInfo _) => pure (exposed && name.getPrefix != `Semantics.Delta)
      | some (.defnInfo info) =>
        if let .abbrev := info.hints then
          if (`Semantics.Templates).isPrefixOf name || (`Semantics.Definitions).isPrefixOf name then
            pure false
          else lambdaTelescope info.value fun _ body => do
            let head := body.getAppFn.constName?.getD .anonymous
            return (`Semantics).isPrefixOf head && head.getPrefix != `Semantics.Delta &&
              env.contains (head.replacePrefix `Semantics `Semantics.Primitives)
        else pure false
      | _ => pure false
    if (`Semantics.Primitives).isPrefixOf name || raw then
      throwErrorAt body "Only primitive declarations may reference raw semantic constructor {name}"
  return value

private partial def guardAuthoredDefaults (stx : Syntax) : CommandElabM Syntax := do
  if stx.isOfKind ``Parser.Term.binderDefault then
    let value : Term := ⟨stx[1]⟩
    return stx.setArg 1 (← `(term| primitive_boundary% $value)).raw
  match stx with
  | .node info kind args => return .node info kind (← args.mapM guardAuthoredDefaults)
  | _ => return stx

private def validateAuthoredDefaults (signature : Syntax) : CommandElabM Unit := do
  unless (signature.find? (·.isOfKind ``Parser.Term.binderDefault)).isSome do return
  let context ← `(bracketedBinder| [Semantics.AuthoringContext])
  let mut binders := #[context.raw]
  for binder in signature[0].getArgs do
    let mut binder := binder
    if binder[2].getNumArgs == 2 && binder[2][1].isOfKind ``spliceMode then
      let resultType : Term := ⟨binder[2][1][1]⟩
      binder := binder.setArg 2 (binder[2].setArg 1 (← `(term| caller_splice_type% $resultType)).raw)
    binders := binders.push binder
  let checks : Array (TSyntax ``Parser.Term.funBinder) := binders.map (⟨·⟩)
  let check ← `(term| fun $checks* => ())
  liftTermElabM <| withoutErrToSorry do
    discard <| elabTerm check none
    synthesizeSyntheticMVarsNoPostponing

private def elabAuthoredCommandImpl (registryOnly : Bool) (stx : Syntax) : CommandElabM Unit := do
  let mods : TSyntax ``Parser.Command.declModifiers := ⟨stx[0]⟩
  let kind := stx[1].getId.eraseMacroScopes
  unless authoredKinds.contains kind do throwUnsupportedSyntax
  let original : Ident := ⟨stx[2]⟩
  let mut name := original
  let mut sig : TSyntax ``Parser.Command.optDeclSig := ⟨stx[3]⟩
  let mut payload := stx[5]
  let mut fields ← if payload.isOfKind ``declBlock then blockFields payload else pure []
  if kind != `primitive then
    sig := ⟨← guardAuthoredDefaults sig.raw⟩
    validateAuthoredDefaults sig.raw
    fields ← fields.mapM fun (key, value) => do
      if lexicalFields.contains key || key ∈ [`name, `label, `kind, `deed, `permanent, `ofAbility] then
        return (key, value)
      return (key, ← `(term| primitive_boundary% $value))
    unless payload.isOfKind ``declBlock do
      let body : Term := ⟨payload⟩
      payload := (← `(term| primitive_boundary% $body)).raw
  for (field, value) in fields do
    unless lexicalFields.contains field || (semanticFields kind).contains field do
      throwErrorAt value "Unknown field {field} in {kind} declaration"
  let required := fun key => match fields.lookup key with
    | some value => pure value
    | none => throwErrorAt payload "{kind} requires `{key} := ...`"
  let declarationLabel := Syntax.mkStrLit original.getId.getString!
  let label := (fields.lookup `name).getD declarationLabel
  let rules := (fields.lookup `rules <|> fields.lookup `confers).getD (← `(term| []))
  let resultSort? ← if sig.raw[1].isNone then
      pure ((commandReturnType? kind).map (`Semantics ++ ·))
    else liftTermElabM do
      let type : Term := ⟨sig.raw[1][0][1]⟩
      let type ← match type with
        | `(Option $inner) => pure inner.raw
        | _ => pure type.raw
      unless type.isIdent do return none
      let candidates ← try resolveGlobalConst type catch _ => pure []
      if let some candidate := candidates.head? then
        authoredResultSort? (← mkConstWithFreshMVarLevels candidate)
      else return none
  let resultName := resultSort?.map (fun sort => Name.mkSimple sort.getString!)
  if sig.raw[1].isNone then
    let some result := resultName | throwErrorAt sig "{kind} requires an explicit return type"
    let typeSpec ← `(Parser.Term.typeSpec| : $(mkIdent (`Semantics ++ result)))
    sig := ⟨sig.raw.setArg 1 (mkNullNode #[typeSpec.raw])⟩
  if authoredKinds.contains kind then
    let full := (← getCurrNamespace) ++ original.getId
    if (← getEnv).contains full &&
        !(registryKinds.contains kind && bundleRegistryAttribute.hasTag (← getEnv) full) then
      let some result := resultName | throwErrorAt sig "Expected a named return type"
      addSortAlias full
      name := mkIdentFrom original (result ++ original.getId)
  if authoredKinds.contains kind then
    if let some result := resultSort? then
      let declaration := (← getCurrNamespace) ++ name.getId
      modifyEnv fun env => expectedTypeDispatchExt.addEntry env {
        bareName := Name.mkSimple original.getId.getString!
        returnType := result
        declaration
      }
  let declId ← `(Parser.Command.declId| $name:ident)
  let emit := fun (body : Term) (definition : Bool) => do
    let value ← `(Parser.Command.declVal| := $body)
    if definition && registryKinds.contains kind &&
        bundleRegistryAttribute.hasTag (← getEnv) ((← getCurrNamespace) ++ name.getId) then
      let existing := Lean.mkConst ((← getCurrNamespace) ++ name.getId)
      let sameValue ← liftTermElabM do
        isDefEq existing (← elabTermEnsuringType body (← inferType existing))
      unless sameValue do
        throwError "Conflicting catalog value {name}"
    else if definition then
      elabCommand (← `($mods:declModifiers semantic_definition $declId:declId
        $sig:optDeclSig $value:declVal))
    else
      elabCommand (← `($mods:declModifiers semantic_template $declId:declId
        $sig:optDeclSig $value:declVal))
  if kind ∈ [`primitive, `template, `action, `keyword, `ability_word] then
    let mut body ← if payload.isOfKind ``declBlock then
      if kind == `keyword then pure ((fields.lookup `body).getD (← `(term| [])))
      else required `body
      else pure ⟨payload⟩
    let identity := Syntax.mkStrLit original.getId.getString!
    if kind == `action then
      let deed := (fields.lookup `deed).getD (Syntax.mkStrLit original.getId.getString!)
      let deed : Term := if deed.raw.isIdent then Syntax.mkStrLit deed.raw.getId.toString else deed
      let permanent := (fields.lookup `permanent).getD (mkIdent `Semantics.DeedPermanent.none)
      let permanent : Term := if permanent.raw.getId ∈ [`performs, `isAbout] then
        mkIdent (`Semantics.DeedPermanent ++ permanent.raw.getId) else permanent
      let deed ← if let some ability := fields.lookup `ofAbility then
          if (fields.lookup `deed).isSome || (fields.lookup `permanent).isSome then
            throwErrorAt payload "ofAbility cannot be combined with deed or permanent"
          unless ability.raw.isIdent do
            throwErrorAt ability "ofAbility requires a keyword identifier"
          let some label ← liftTermElabM <| resolveKeywordLabelConst? ability.raw
            | throwErrorAt ability "Unknown keyword ability label {ability}"
          let ability := mkCIdent label
          `(term| $(mkIdent `Semantics.Deed.ofAbility) $ability)
        else `(term| $(mkIdent `Semantics.Deed.action) $deed $permanent)
      let deed ← if (fields.lookup `deed).any (fun value => value.raw.isIdent &&
          value.raw.getId == `none) then `(term| none) else `(term| some $deed)
      body ← `(term| $(mkIdent `Semantics.Action.ownScope) $deed $body)
    if kind == `keyword then
      let labelName := (← getCurrNamespace) ++ name.getId ++ `label
      let labelTerm := (fields.lookup `label).getD declarationLabel
      registerKeywordLabel labelName labelTerm
      let mut args : Array Term := #[]
      for binder in sig.raw[0].getArgs do
        if binder.isOfKind ``Parser.Term.explicitBinder then
          for id in binder[1].getArgs do
            let argument : Term := ⟨id⟩
            args := args.push argument
      body ← `(term| $(mkIdent `Semantics.Ability.keyword) $(mkIdent labelName)
        (keyword_arguments% [$args,*]) $body)
    if kind == `ability_word then
      let label := (fields.lookup `label).getD declarationLabel
      unless label.raw.isStrLit?.isSome do throwErrorAt label "Ability-word label must be a string literal"
      let label : StrLit := ⟨label.raw⟩
      body ← `(term| ability_word_body% $label:str $body)
    if kind == `template then body ← `(term| bind_decl_body% false $identity $body)
    if kind == `action then body ← `(term| bind_decl_body% true $identity $body)
    if kind == `keyword then body ← `(term| bind_decl_body% keyword $identity $body)
    emit body (kind ∈ [`action, `keyword])
    if kind == `primitive then
      elabCommand (← `(attribute [semantic_primitive] $name))
      if let some result := resultName then
        let alias := (← getCurrNamespace) ++ result ++ Name.mkSimple original.getId.getString!
        if (← getEnv).contains alias then primitiveAttribute.setTag alias
  else if kind ∈ [`counter, `subtype, `designation, `card_type, `relation] then
    let body ← match kind with
      | `counter =>
        let holder := (fields.lookup `holder).getD (mkIdent `Semantics.Kind.object)
        `(term| $(mkIdent `Semantics.CounterKind.named) $declarationLabel $holder)
      | `subtype =>
        let k ← required `kind
        if k.raw.getId == `spell || (k.raw.reprint.getD "").trimAscii.toString == ".spell" then
          `(term| $(mkIdent `Semantics.Subtype.spell) $label)
        else `(term| $(mkIdent `Semantics.Subtype.of) $k $label)
      | `designation =>
        let scope ← required `scope
        let sole := (fields.lookup `sole).getD (← `(term| false))
        `(term| $(mkIdent `Semantics.DesignationLabel.named) $declarationLabel $scope (sole := $sole))
      | `card_type =>
        let cardClass := (fields.lookup `cardClass).getD (← `(term| none))
        let stats := (fields.lookup `stats).getD (← `(term| []))
        let label := (fields.lookup `name).getD label
        `(term| { name := $label, cardClass := $cardClass, stats := $stats })
      | _ => `(term| $(mkIdent `Semantics.RelationLabel.named) $declarationLabel)
    emit body true
    if registryOnly then
      bundleRegistryAttribute.setTag ((← getCurrNamespace) ++ name.getId)
      let declaration ← resolveGlobalConstNoOverload name
      registerExpectedType declaration
      registerSharedAliases declaration
      return
    let rowName := name.getId.getPrefix ++ Name.mkSimple (name.getId.getString! ++ "_definition")
    let row := mkIdent rowName
    let ctorKind := if kind == `card_type then `cardType else kind
    let ctor := mkIdent (`Semantics.RegistryDefinition ++ ctorKind)
    let rowBody ← if kind == `relation then `(term| $ctor $name) else `(term| $ctor $name $rules)
    elabCommand (← `(def $row : $(mkIdent `Semantics.RegistryDefinition) := $rowBody))
    let ok := mkIdent (name.getId.getPrefix ++ Name.mkSimple ("ok_" ++ name.getId.getString!))
    let check := mkIdent `Semantics.RegistryDefinition.check
    elabCommand (← `(theorem $ok : $check $row = [] := by decide))
  else
    let mut body ← if payload.isOfKind ``declBlock then
      if let some body := fields.lookup `body then pure body
      else makeStruct (fields.filter fun f =>
        !lexicalFields.contains f.1)
      else pure ⟨payload⟩
    if kind == `card then
      body := ⟨mkNode `Semantics.spelledTerm #[mkAtom "spelled", mkAtom "<|", body.raw]⟩
    let value ← `(Parser.Command.declVal| := $body)
    elabCommand (← `($mods:declModifiers def $declId:declId $sig:optDeclSig $value:declVal))
    if kind == `card then
      let ok := mkIdent (name.getId.getPrefix ++ Name.mkSimple ("ok_" ++ name.getId.getString!))
      let check := mkIdent `Semantics.Card.check
      let cardValue := mkIdent `Semantics.Spelled.card
      elabCommand (← `(theorem $ok : $check ($cardValue $name) = [] :=
        $(mkIdent `Semantics.Spelled.ok) $name))
    else generatedCheck name (resultName.getD .anonymous)
  let declaration ← resolveGlobalConstNoOverload name
  registerExpectedType declaration
  registerSharedAliases declaration

@[command_elab authoredDeclCmd]
def elabAuthoredCommand : CommandElab := fun stx => do
  if stx[1].getId == `card then
    throwError "Named card declarations are no longer supported; use card := ..."
  elabAuthoredCommandImpl false stx

@[command_elab anonymousCardCmd]
def elabAnonymousCard : CommandElab := fun stx => do
  let named ← match namedCardSyntax stx with
    | .ok named => pure named
    | .error message => throwError message
  elabAuthoredCommandImpl false named

/-- A declaration's bare spelling and semantic result sort identify a bundle node. -/
structure BundleKey where
  bareName : Name
  returnType : Name
  deriving BEq, Hashable, Inhabited, Repr

structure BundleItem where
  key : BundleKey
  file : String
  references : List Name := []
  primitiveReferences : List Name := []
  valueRegistered : Bool := false
  primitive : Bool := false
  stx : Syntax := .missing
  deriving Inhabited

private def bundleKeyLess (a b : BundleKey) : Bool :=
  a.bareName.toString < b.bareName.toString ||
    (a.bareName == b.bareName && a.returnType.toString < b.returnType.toString)

def bundleDependencies (items : List BundleItem) (item : BundleItem) : List BundleKey :=
  if item.primitive then [] else
    let refSet := Std.HashSet.ofList item.references
    let primRefSet := Std.HashSet.ofList (item.references ++ item.primitiveReferences)
    items.filterMap fun other =>
      let qualified := Name.mkSimple other.key.returnType.getString! ++ other.key.bareName
      let row := Name.mkSimple (other.key.bareName.getString! ++ "_definition")
      let references := if other.primitive then primRefSet else refSet
      if ((other.valueRegistered && references.contains row) ||
          (!other.valueRegistered &&
            (references.contains other.key.bareName || references.contains qualified))) &&
        other.key != item.key &&
        (other.key.bareName != item.key.bareName || other.primitive ||
          (item.key.returnType == `Semantics.Ability && other.key.returnType == `Semantics.Action))
      then some other.key else none

/-- Kahn's algorithm, choosing the least ready (bare name, return type) at every step. -/
def topoSortBundle (items : List BundleItem) : Except String (List BundleItem) := Id.run do
  let mut seen : List BundleKey := []
  for item in items do
    if seen.contains item.key then
      return .error (s!"Duplicate bundle declaration: {item.key.bareName} : " ++
        s!"{item.key.returnType} ({item.file})")
    seen := item.key :: seen
  let mut byKey : Std.HashMap BundleKey BundleItem := {}
  let mut degrees : Std.HashMap BundleKey Nat := {}
  let mut dependents : Std.HashMap BundleKey (List BundleKey) := {}
  let mut ready := []
  for item in items do
    byKey := byKey.insert item.key item
    let deps := bundleDependencies items item
    degrees := degrees.insert item.key deps.length
    if deps.isEmpty then ready := item.key :: ready
    for dep in deps do
      dependents := dependents.insert dep (item.key :: dependents.getD dep [])
  ready := ready.mergeSort bundleKeyLess
  let mut sorted := []
  while !ready.isEmpty do
    let key := ready.head!
    ready := ready.tail!
    sorted := sorted ++ [byKey.getD key default]
    for dependent in dependents.getD key [] do
      let degree := degrees.getD dependent 0 - 1
      degrees := degrees.insert dependent degree
      if degree == 0 then ready := (dependent :: ready).mergeSort bundleKeyLess
  if sorted.length != items.length then
    let remaining := items.filter (fun item => degrees.getD item.key 0 > 0)
      |>.mergeSort (fun a b => bundleKeyLess a.key b.key)
    let rec reaches (fuel : Nat) (target : BundleKey) (pending seen : List BundleKey) : Bool :=
      match fuel, pending with
      | 0, _ | _, [] => false
      | fuel + 1, key :: rest =>
        if key == target then true
        else if seen.contains key then reaches fuel target rest seen
        else reaches fuel target
          (bundleDependencies items (byKey.getD key default) ++ rest) (key :: seen)
    let cyclic := remaining.filter fun item => reaches (items.length * (items.length + 1) + 1)
      item.key (bundleDependencies items item) []
    let summary := cyclic.map fun item =>
      s!"{item.key.bareName} : {item.key.returnType} ({item.file})"
    return .error s!"Dependency cycle in bundle: {", ".intercalate summary}"
  return .ok sorted

private partial def referencedIdents (bound : List Name) (stx : Syntax) : List Name :=
  if stx.isOfKind ``declField then
    if lexicalFields.contains stx[0].getId.eraseMacroScopes || stx[0].getId == `ofAbility then []
    else referencedIdents bound stx[2]
  else if stx.isOfKind ``Parser.Term.dotIdent then []
  else if stx.isOfKind ``Parser.Term.structInstField then
    referencedIdents bound stx[1]
  else match stx with
    | .ident _ _ name _ =>
      let name := name.eraseMacroScopes
      if bound.contains name then []
      else if name == `this then [`this, `this_]
      else if [`creature, `artifact, `enchantment, `land, `planeswalker, `battle,
          `instant, `sorcery, `kindred, `card, `permanent, `player, `token, `spell].contains name then
        [name, Name.mkSimple ("is" ++ name.toString.capitalize)]
      else [name, name.replacePrefix `Semantics.Templates .anonymous,
        name.replacePrefix `Templates .anonymous,
        name.replacePrefix `Semantics.Definitions .anonymous]
    | .node _ _ args => args.toList.flatMap (referencedIdents bound)
    | _ => []

private def bundleItem (file : String) (stx : Syntax) : Except String BundleItem := do
  if stx.isOfKind ``authoredDeclCmd && stx[1].getId == `card then
    throw "Named card declarations are no longer supported; use card := ..."
  let stx ← if stx.isOfKind ``anonymousCardCmd then namedCardSyntax stx else pure stx
  unless stx.isOfKind ``authoredDeclCmd && authoredKinds.contains stx[1].getId do
    throw s!"Unsupported command in bundle file {file}"
  let kind := stx[1].getId.eraseMacroScopes
  let sig := stx[3]
  let result ← if let some result := commandReturnType? kind then pure result
    else if sig[1].isNone then
      throw s!"{file}: {kind} requires a named return type"
    else if sig[1][0][1].isIdent then pure sig[1][0][1].getId
    else pure (Name.mkSimple (sig[1][0][1].reprint.getD ""))
  let returnType := if (`Semantics).isPrefixOf result then result else `Semantics ++ result
  let declared := stx[2].getId
  let bareName := if declared.getPrefix.isAnonymous ||
      declared.getPrefix == Name.mkSimple returnType.getString! then
    Name.mkSimple declared.getString! else declared
  let bound := sig[0].getArgs.toList.flatMap fun binder =>
    binder[1].getArgs.toList.filterMap fun id => if id.isIdent then some id.getId else none
  return { key := { bareName, returnType }, file, stx,
           valueRegistered := registryKinds.contains kind,
           primitive := kind == `primitive,
           -- Bound names may denote primitive functions in application position.
           primitiveReferences := referencedIdents [] stx[3] ++ referencedIdents [] stx[5],
           references := referencedIdents bound stx[3] ++ referencedIdents bound stx[5] ++
             (if kind == `ability_word then [`abilityWord] else []) }

private def parseBundleFile (env : Environment) (opts : Options) (path : System.FilePath) :
    IO (List BundleItem) := do
  let content ← IO.FS.readFile path
  let input := Parser.mkInputContext content path.toString
  let mut state : Parser.ModuleParserState := {}
  let mut items := []
  repeat
    let trailing := Parser.whitespace.run input { env, options := opts }
      (Parser.getTokenTable env) { cache := Parser.initCacheForInput content, pos := state.pos }
    if trailing.errorMsg.isNone && input.atEnd trailing.pos then break
    let (stx, next, messages) := Parser.parseCommand input { env, options := opts } state {}
    if messages.hasErrors then
      let errors ← messages.toList.mapM (·.toString)
      throw <| IO.userError ("\n".intercalate errors)
    state := next
    if Parser.isTerminalCommand stx then break
    items := items ++ [← IO.ofExcept (bundleItem path.toString stx)]
  if items.length != 1 then
    throw <| IO.userError s!"bundle file '{path}' must contain exactly one declaration, found {items.length}"
  return items

/-- Load declarations from loose Lean files, independent of their paths and source ordering. -/
syntax (name := loadBundle) "load_bundle " str (" templatesIn " ident)? : command

@[command_elab loadBundle]
def elabLoadBundle : CommandElab := fun stx => do
  let directory : StrLit := ⟨stx[1]⟩
  let templateNamespace := if stx[2].isNone then none else some stx[2][1].getId
  let paths ← liftIO <| (System.FilePath.mk directory.getString).walkDir
  let paths := paths.toList.filter (·.extension == some "lean")
    |>.mergeSort (fun a b => a.toString < b.toString)
  let mut items := []
  for path in paths do
    items := items ++ (← liftIO <| parseBundleFile (← getEnv) (← getOptions) path)
  let sorted ← match topoSortBundle items with
    | .ok sorted => pure sorted
    | .error error => throwError error
  -- Catalog values are leaves; their conferrals may refer back through action and noun helpers.
  for item in sorted do
    if item.valueRegistered then
      withRef item.stx <| elabAuthoredCommandImpl true item.stx
  -- Labels do not depend on bodies. Register them before elaborating an ability's own deed.
  for item in sorted do
    if item.stx[1].getId == `keyword then
      let original := item.stx[2].getId
      let ns ← getCurrNamespace
      let full := ns ++ original
      let name := if (← getEnv).contains full then
        ns ++ `Ability ++ original else full
      let fields ← if item.stx[5].isOfKind ``declBlock then blockFields item.stx[5] else pure []
      let label := (fields.lookup `label).getD (Syntax.mkStrLit original.getString!)
      registerKeywordLabel (name ++ `label) label
  for item in sorted do
    try
      let messageCount := (← get).messages.toList.length
      let content ← liftIO <| IO.FS.readFile item.file
      withReader (fun ctx => { ctx with fileName := item.file, fileMap := FileMap.ofString content }) do
        let callerNamespace ← getCurrNamespace
        let ns := templateNamespace
        if let some ns := ns then
          if item.stx[1].getId ∈ [`primitive, `template] then
            withScope (fun scope => { scope with
                currNamespace := ns
                openDecls := .simple callerNamespace [] :: scope.openDecls }) do
              withRef item.stx <| elabAuthoredCommandImpl false item.stx
          else withRef item.stx <| elabAuthoredCommandImpl false item.stx
        else withRef item.stx <| elabAuthoredCommandImpl false item.stx
      if (← get).messages.toList.length > messageCount then
        logInfo m!"While loading {item.file}"
    catch error =>
      throwError "{item.file}: {error.toMessageData}"

end Semantics.Authoring

/-- Inspect an expansion explicitly without changing its semantic result or granting authorship. -/
def Semantics.expand.{u} {α : Sort u} (value : α) : α := value
