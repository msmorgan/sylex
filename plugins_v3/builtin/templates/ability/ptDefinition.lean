/-- "<subject>'s power [and toughness] [are each] [is] equal to <amount>": a
Characteristic-Defining Ability setting each stat `slots` names [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject>'s power [and toughness] [are each] [is] equal to <amount>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.StaticSpec.staticSpecCharacteristicDefining subject
      (match slots with
      | Semantics.DefinedSlots.powerAlone =>
        [Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power amount]
      | Semantics.DefinedSlots.toughnessAlone =>
        [Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness amount]
      | Semantics.DefinedSlots.bothEach =>
        [Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power amount,
          Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness amount])
}
