card sphereOfPurity : Spelled :=
  { name := "Sphere of Purity", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (replace (would (a isArtifact) (dealDamage anyAmount you))
          (instead (preventDamage (ofThatDamage 1)))) ] }
