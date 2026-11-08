import Semantics.Registry
import SylGen.ReadOrder

open Lean Meta Elab Command

namespace SylGen

private def register (kind : String) (name : Name) (file : String) :
    CommandElabM Unit := do
  if kind == "action" then
    if let some deed ← liftTermElabM <| Semantics.Registry.actionDefinitionDeed? name then
      liftTermElabM <| Semantics.Registry.checkActionDeed name deed
      modifyEnv (Semantics.Registry.actionDeedsExt.addEntry · (name, deed))
  Semantics.Registry.registerExpectedType name
  Semantics.Registry.registerSharedAliases name
  if kind == "card" then
    let file := if file.isEmpty then file else
      Semantics.Registry.cardFileName name.getString!
    modifyEnv (Semantics.Registry.canonCardsExt.addEntry · { file, declaration := name })

elab "syl_register " kind:str name:ident file:str : command => do
  register kind.getString (← resolveGlobalConstNoOverload name)
    file.getString

elab "syl_def " name:ident binders:bracketedBinder* " : " result:term " := " value:term
    " doc " documentation:str : command => do
  let value ← liftTermElabM do
    let typed ← `(($value : $result))
    let binders : TSyntaxArray ``Parser.Term.funBinder := binders.map fun b => ⟨b.raw⟩
    let termSyntax ← if binders.isEmpty then pure typed else `(fun $binders* => $typed)
    let expr ← Term.elabTerm termSyntax none
    Term.synthesizeSyntheticMVarsNoPostponing
    instantiateMVars expr
  let value ← liftTermElabM <| ReadOrder.assign value
  let type ← liftTermElabM <| inferType value
  liftCoreM <| addAndCompile <| .defnDecl {
    name := name.getId, levelParams := [], type := type, value := value,
    hints := .regular 0, safety := .safe }
  unless documentation.getString.isEmpty do
    addDocStringCore name.getId documentation.getString

/-- Emit an ability word, preserving an already headed body at declaration time. -/
elab "syl_ability_word " name:ident binders:bracketedBinder* " := " value:term
    " doc " documentation:str : command => do
  let value ← liftTermElabM do
    let typed ← `(($value : (Semantics.AbilityWordLabel → Semantics.Ability →
      Semantics.Ability) × String × Semantics.Ability))
    let binders : TSyntaxArray ``Parser.Term.funBinder := binders.map fun b => ⟨b.raw⟩
    let termSyntax ← if binders.isEmpty then pure typed else `(fun $binders* => $typed)
    let expr ← Term.elabTerm termSyntax none
    Term.synthesizeSyntheticMVarsNoPostponing
    let expr ← instantiateMVars expr
    lambdaTelescope expr fun parameters tuple => do
      let tuple ← whnf tuple
      let wrap := tuple.getAppArgs[2]!
      let pair ← whnf tuple.getAppArgs[3]!
      let label := pair.getAppArgs[2]!
      let body := pair.getAppArgs[3]!
      let reduced ← whnf body
      -- The word is a primitive's application, so it is reduced too.
      let headed ← if reduced.getAppFn.isConstOf `Semantics.Ability.italicHead then do
          let word ← whnf reduced.getAppArgs[0]!
          pure (word.getAppFn.isConstOf `Semantics.ItalicWord.abilityWord)
        else pure false
      let result := if headed then body else
        mkApp2 wrap (mkApp (mkConst `Semantics.AbilityWordLabel.named) label) body
      mkLambdaFVars parameters result
  let value ← liftTermElabM <| ReadOrder.assign value
  let type ← liftTermElabM <| inferType value
  liftCoreM <| addAndCompile <| .defnDecl {
    name := name.getId, levelParams := [], type := type, value := value,
    hints := .regular 0, safety := .safe }
  unless documentation.getString.isEmpty do
    addDocStringCore name.getId documentation.getString

end SylGen
