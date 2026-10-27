/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
template foodToken : CharacteristicBundle := {
  spelling := "Food token",
  body :=
    {
      characteristics :=
        { types := [Semantics.CardType.artifact], subtypes := [Semantics.Definitions.food],
          text :=
            [Semantics.Templates.Ability.activated
                (Semantics.Templates.Cost.compound
                  [Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 2], Semantics.Templates.Cost.tapSymbol,
                    Semantics.Templates.Cost.perform (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisArtifact)])
                (Semantics.Templates.Action.gainLife 3)] } }
}
