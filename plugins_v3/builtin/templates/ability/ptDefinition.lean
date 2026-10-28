/-- "<subject>'s power [and toughness] [are each] [is] equal to <amount>": a
Characteristic-Defining Ability setting each stat `slots` names [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject>'s power [and toughness] [are each] [is] equal to <amount>; string substitution cannot express its omitted or inflected parts.
  body :=
    characteristicDefining subject
      (match slots with
      | Semantics.DefinedSlots.powerAlone =>
        [stat .sets .power amount]
      | Semantics.DefinedSlots.toughnessAlone =>
        [stat .sets .toughness amount]
      | Semantics.DefinedSlots.bothEach =>
        [stat .sets .power amount,
          stat .sets .toughness amount])
}
