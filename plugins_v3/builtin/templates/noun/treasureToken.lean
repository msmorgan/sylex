/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
template treasureToken : CharacteristicBundle := {
  spelling := "Treasure token",
  body :=
    {
      characteristics :=
        { types := [Semantics.CardType.artifact], subtypes := [Semantics.Definitions.treasure],
          text :=
            [Semantics.Templates.Ability.activated
                (Semantics.Templates.Cost.compound
                  [Semantics.Templates.Cost.tapSymbol,
                    Semantics.Templates.Cost.perform (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisArtifact)])
                (Semantics.Templates.Action.addMana 1 (Semantics.Templates.ProducedMana.ofAnyColor Semantics.ColorFreedom.sameColor) [])] } }
}
