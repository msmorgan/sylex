/-- Volrath's Laboratory -/
card := {
  name := "Volrath's Laboratory",
  cost := [5],
  types := [artifact],
  text := [
    static [entersChoosing thisArtifact color, entersChoosing thisArtifact (subtype creature)],
    activated (compound [[5], tapSymbol])
      (create 1
         (written
            ⟨{ types := [creature], power := 2, toughness := 2 } ,
               [withQuality (ofChosen color), withQuality (ofChosen (subtype creature))]⟩))
  ]
}
