import Lean

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

private opaque FinalizeContext : Type

/-- Elaborate a mutual inductive block with casesOn, ctorIdx, ctorElim, and noConfusion,
omitting recOn, below, brecOn, SizeOf, and injectivity theorems. -/
private unsafe def runFastInductives (inductives : Array (Modifiers × Syntax)) :
    CommandElabM Unit := do
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
  let elabs ← runTermElabM fun _ =>
    inductives.mapM fun (modifiers, stx) => mkInductiveView modifiers stx
  checkNoConflicts elabs false
  elabs.forM fun e => checkValidInductiveModifier e.view.modifiers
  liftTermElabM <| elabs.forM fun e =>
    withRef e.view.ref <|
      Term.applyAttributesAt e.view.declName e.view.modifiers.attrs .beforeElaboration
  let res ← runTermElabM fun vars => do
    let view0 := elabs[0]!.view
    Term.withDeclName view0.declName <| withRef view0.ref <|
      withExporting (isExporting := !isPrivateName view0.declName) do
      let res ← mkInductiveDecl vars elabs
      let declNames := elabs.map (·.view.declName)
      compileDecls declNames
      withoutExporting (when := view0.ctors.any fun x => isPrivateName x.declName) do
        for n in declNames do
          mkCasesOn n
          mkCtorIdx n
          mkCtorElim n
          mkNoConfusion n
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
  registerTagAttribute `semantic_expression
    "Semantic expressions require template or definition authoring."

initialize literalAttribute : TagAttribute ←
  registerTagAttribute `semantic_literal "A literal leaf value, rather than an authored expression."

initialize internalAttribute : TagAttribute ←
  registerTagAttribute `internal_expansion
    "An expansion carrier without an authorable primitive template."

initialize contextualAuthoringAttribute : TagAttribute ←
  registerTagAttribute `contextual_authoring
    "A semantic declaration with generated lexical context."

initialize primitiveAttribute : TagAttribute ←
  registerTagAttribute `semantic_primitive "An authored primitive declaration."

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

end Semantics.Authoring
