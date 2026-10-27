/-- A Clue token's characteristics: "a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" [CR#111.10f] -/
template clueToken : CharacteristicBundle := {
  spelling := "Clue token",
  body :=
    { characteristics :=
        { types := [.artifact], subtypes := [clue],
          text :=
            [ activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact)])
                (draw 1) ] } }
}
