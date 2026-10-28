/-- "<subject>'s power is equal to <power> and its toughness is equal to <toughness>": one
Characteristic-Defining Ability setting both, the toughness free to read the power's number
back ("that number plus 1") [CR#604.3a,613.4a]. -/
-- [CR#604.3a,613.4a]
template ptDefinitions (subject : NounPhrase) (power toughness : Amount) : StaticSpec := {
  spelling := "<subject>'s power is equal to <power> and its toughness is equal to <toughness>",
  body :=
    characteristicDefining subject
      [stat sets power power,
        stat sets toughness toughness]
}
