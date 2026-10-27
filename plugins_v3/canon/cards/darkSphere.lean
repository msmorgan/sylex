card darkSphere : Spelled :=
  { name := "Dark Sphere", cost := some [], types := [.artifact],
    text :=
      [ activated (compound [tapSymbol, sacrifice thisArtifact])
          (establishThroughout
            (replace (would (aYourChoice isSource) (dealDamage anyAmount you))
              (instead (preventDamage (half .down))) (use := .nextTimeOnly))
            thisTurn) ] }
