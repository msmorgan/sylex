card dryadOfTheIlysianGrove : Spelled :=
  { name := "Dryad of the Ilysian Grove", cost := [generic 2, pip .green],
    types := [.enchantment, .creature], subtypes := [nymph, dryad],
    text :=
      [ static
          (can you (play (counted (additional (exactly 1)) isLand))
             (timing := onEachOfYourTurns)),
        static (qualityChange (allOf (and [isLand, youControl])) .adds
          (everyTypeOf .basicLand)) ],
    power := stat 2, toughness := stat 4 }
