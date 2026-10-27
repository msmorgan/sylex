card prismRing : Spelled :=
  { name := "Prism Ring", cost := [generic 1], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        whenever (does you (cast (a
          (and [isSpell, ofChosen .color])))) (gainLife 1) ] }
