card urzasArmor : Spelled :=
  { name := "Urza's Armor", cost := [generic 6], types := [.artifact],
    text :=
      [ static (replace (would (a isSource) (dealDamage anyAmount you))
          (instead (preventDamage (ofThatDamage 1)))) ] }
