card pinpointAvalanche : Spelled :=
  { name := "Pinpoint Avalanche", cost := [generic 3, pip .red, pip .red],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage 4 (target isCreature)),
            establish (cant any (preventDamage allDamage)) modifiesPrecedingAction ]) ] }
