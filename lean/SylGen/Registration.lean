import Semantics.Authoring

open Lean Meta Elab Command

namespace SylGen

private def register (kind : String) (name : Name) (file : String) :
    CommandElabM Unit := do
  if kind ∈ ["primitive", "template", "ability_word", "action", "ability", "counter",
      "designation", "subtype", "card_type", "relation"] then
    Semantics.Authoring.contextualAuthoringAttribute.setTag name
    if kind ∈ ["primitive", "template", "ability_word"] then
      Semantics.Authoring.templateAttribute.setTag name
    else Semantics.Authoring.definitionAttribute.setTag name
    if kind ∈ ["counter", "designation", "subtype", "card_type", "relation"] then
      Semantics.Authoring.bundleRegistryAttribute.setTag name
  if kind == "primitive" then Semantics.Authoring.primitiveAttribute.setTag name
  if kind == "ability" then
    Semantics.Authoring.definitionAttribute.setTag (name ++ `label)
  if kind == "action" then
    if let some deed ← liftTermElabM <| Semantics.Authoring.actionDefinitionDeed? name then
      liftTermElabM <| Semantics.Authoring.checkActionDeed name deed
      modifyEnv (Semantics.Authoring.actionDeedsExt.addEntry · (name, deed))
      Semantics.Authoring.definitionAttribute.setTag (name ++ `deed)
  Semantics.Authoring.registerExpectedType name
  Semantics.Authoring.registerSharedAliases name
  if kind == "card" then
    let file := if file.isEmpty then file else
      Semantics.Authoring.cardFileName name.getString!
    modifyEnv (Semantics.Authoring.canonCardsExt.addEntry · { file, declaration := name })

elab "syl_register " kind:str name:ident file:str : command => do
  register kind.getString (← resolveGlobalConstNoOverload name)
    file.getString

elab "syl_ability_word% " wrap:ident label:term ", " body:term : term <= expected => do
  let value ← Lean.Elab.Term.elabTermEnsuringType body expected
  let reduced ← whnf value
  if reduced.isAppOfArity `Semantics.Ability.italicHead 2 &&
      reduced.appFn!.appArg!.isAppOfArity `Semantics.ItalicWord.abilityWord 1 then return value
  let label ← Lean.Elab.Term.elabTermEnsuringType label (mkConst `String)
  mkAppM (← resolveGlobalConstNoOverload wrap)
    #[mkApp (mkConst `Semantics.AbilityWordLabel.named) label, value]

end SylGen
