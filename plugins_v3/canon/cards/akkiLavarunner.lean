card akkiLavarunner : Spelled := { body := .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := [generic 3, pip .red], types := [.creature],
      subtypes := [goblin, warrior],
      text :=
        [ haste,
          whenever (does thisCreature (dealDamage (to := anOpponent)))
            (flip thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Tok-Tok, Volcano Born", supertypes := [.legendary], types := [.creature],
      subtypes := [goblin, shaman],
      text :=
        [ protection (hasColor .red),
          static
            (replace (would (a (and [isSource, hasColor .red])) (dealDamage anyAmount (a isPlayer)))
              (instead (instruct it (dealDamage (plus thatMuch 1) (that player))))) ],
      power := stat 2, toughness := stat 2 } } }
