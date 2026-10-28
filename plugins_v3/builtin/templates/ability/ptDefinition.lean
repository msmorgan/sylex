/-- "<subject>'s power [and toughness] [are each] [is] equal to <amount>": a
Characteristic-Defining Ability setting each stat `slots` names [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject>'s power [and toughness] [are each] [is] equal to <amount>; string substitution cannot express its omitted or inflected parts.
  body :=
    staticSpecCharacteristicDefining subject
      (match slots with
      | Semantics.DefinedSlots.powerAlone =>
        [characteristicEditStat .sets .power amount]
      | Semantics.DefinedSlots.toughnessAlone =>
        [characteristicEditStat .sets .toughness amount]
      | Semantics.DefinedSlots.bothEach =>
        [characteristicEditStat .sets .power amount,
          characteristicEditStat .sets .toughness amount])
}
