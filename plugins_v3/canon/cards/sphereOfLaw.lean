card sphereOfLaw : Spelled :=
  { name := "Sphere of Law", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (replace (would (a (and [isSource, hasColor .red])) (dealDamage anyAmount you))
          (instead (preventDamage (ofThatDamage 2)))) ] }
