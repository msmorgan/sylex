/-- A Clue token's characteristics: "a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" [CR#111.10f] -/
template clueToken : CharacteristicBundle := {
  spelling := "Clue token",
  body :=
    {
      characteristics :=
        { types := [Semantics.CardType.artifact], subtypes := [Semantics.Definitions.clue],
          text :=
            [Semantics.Templates.Ability.activated
                (Semantics.Templates.Cost.compound
                  [Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 2],
                    Semantics.Templates.Cost.perform (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisArtifact)])
                (Semantics.Templates.Action.draw 1)] } }
}
