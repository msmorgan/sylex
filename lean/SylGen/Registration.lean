import Semantics.Authoring

open Lean Meta Elab Command

namespace SylGen

private def register (kind : String) (name bare : Name) (file : String) :
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
  let entries := Semantics.Authoring.expectedTypeDispatchExt.getState (← getEnv)
  for entry in (entries.find? (Name.mkSimple name.getString!)).getD [] do
    if entry.declaration == name then
      modifyEnv (Semantics.Authoring.expectedTypeDispatchExt.addEntry ·
        { entry with bareName := bare })
  Semantics.Authoring.registerSharedAliases name
  if kind == "card" then
    let file := if file.isEmpty then file else
      (Semantics.Authoring.cardFileName name.getString!).replace ".lean" ".syl"
    modifyEnv (Semantics.Authoring.canonCardsExt.addEntry · { file, declaration := name })

elab "syl_register " kind:str name:ident bare:str file:str : command => do
  register kind.getString (← resolveGlobalConstNoOverload name)
    (Name.mkSimple bare.getString) file.getString

-- Inspect the elaborated body, including references to other generated modules. The same
-- head test as ability_word_body% applies without requiring the builtin abilityWord template.
elab "syl_ability_word% " label:term ", " body:term : term <= expected => do
  let value ← Lean.Elab.Term.elabTermEnsuringType body expected
  let reduced ← whnf value
  if reduced.isAppOfArity `Semantics.Ability.italicHead 2 &&
      reduced.appFn!.appArg!.isAppOfArity `Semantics.ItalicWord.abilityWord 1 then return value
  let label ← Lean.Elab.Term.elabTermEnsuringType label (mkConst `String)
  return mkApp2 (mkConst `Semantics.Ability.italicHead)
    (mkApp (mkConst `Semantics.ItalicWord.abilityWord)
      (mkApp (mkConst `Semantics.AbilityWordLabel.named) label)) value

end SylGen
