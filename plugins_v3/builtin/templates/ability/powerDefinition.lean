/-- "<subject>'s power is equal to <amount>": a Characteristic-Defining Ability setting the power
alone [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template powerDefinition (subject : NounPhrase) (amount : Amount) : StaticSpec := {
  spelling := "<subject>'s power is equal to <amount>",
  body :=
    characteristicDefining subject [stat sets power amount]
}
