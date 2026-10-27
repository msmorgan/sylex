/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
card dread : Spelled :=
  { name := "Dread", cost := [generic 3, pip .black, pip .black, pip .black],
    types := [.creature], subtypes := [elemental, incarnation],
    text :=
      [ fear,
        whenever (does (a isCreature) (dealDamage (to := you))) (destroy it),
        when (does any (putInto this_ graveyard (from_ := some any)))
          (instruct (ownerOf it) (shuffleInto it)) ],
    power := stat 6, toughness := stat 6 }
