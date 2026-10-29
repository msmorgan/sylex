/-- "<subject>'s toughness is equal to <amount>": a Characteristic-Defining Ability setting the
toughness alone [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template toughnessDefinition (subject : NounPhrase) (amount : Amount) : StaticSpec := {
  spelling := "<subject>'s toughness is equal to <amount>",
  body :=
    characteristicDefining subject [stat toughness (set amount)]
}
