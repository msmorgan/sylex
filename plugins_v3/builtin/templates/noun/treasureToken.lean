/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
template treasureToken : CharacteristicBundle := {
  spelling := "Treasure token",
  body :=
    {
      characteristics :=
        { types := [Semantics.CardType.artifact], subtypes := [treasure],
          text :=
            [activated
                (compound
                  [tapSymbol,
                    perform (sacrifice thisArtifact)])
                (addMana 1 (ofAnyColor .sameColor) [])] } }
}
