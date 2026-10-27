card aladdinsRing : Spelled :=
  { name := "Aladdin's Ring", cost := [generic 8], types := [.artifact],
    text :=
      [ activated (compound [[generic 8], tapSymbol])
          (instruct thisArtifact (dealDamage 4 (target isAnyTarget))) ] }
