/-- Tail Swipe: "Choose target creature you control and target creature you don't control. If
you cast this spell during your main phase, the creature you control gets +1/+1 until end of
turn. Then those creatures fight each other." A step in which a spell can be cast ends only with
the stack empty [CR#500.2], so the spell resolves in the turn it was cast, and its casting is one
this turn. -/
card := {
  name := "Tail Swipe",
  cost := [green],
  types := [instant],
  text := [
    spell
      [
        (choose (both (target [isCreature, youControl]) (target [isCreature, youDontControl]))),
        doIf (did you (cast thisSpell) (within soFarThisTurn (duringPart mainPhase you)))
          (get (the [isCreature, youControl]) (up 1) (up 1) untilEndOfTurn),
        fightEachOther (those creature)
      ]
  ]
}
