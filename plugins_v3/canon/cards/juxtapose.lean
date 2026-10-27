/-- Juxtapose: "You and target player exchange control of the creature you each control with the
greatest mana value. Then exchange control of artifacts the same way. If two or more permanents
a player controls are tied for greatest, their controller chooses one of them." "The same way"
is written out as the second exchange. -/
card juxtapose : Spelled :=
  { name := "Juxtapose", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exchangeControl (greatestManaValue isCreature you)
              (greatestManaValue isCreature (target isPlayer)),
            exchangeControl (greatestManaValue isArtifact you)
              (greatestManaValue isArtifact (that player)),
            juxtaposeTie ]) ] }
