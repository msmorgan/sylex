/-- "<subject>'s power and toughness are each equal to <amount>": a Characteristic-Defining
Ability setting both stats [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template ptDefinition (subject : NounPhrase) (amount : Amount) : StaticSpec := {
  spelling := "<subject>'s power and toughness are each equal to <amount>",
  body :=
    characteristicDefining subject
      [stat power (set amount),
        stat toughness (set amount)]
}
