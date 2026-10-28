/-- The same over "any target": the player, or the permanent's controller. -/
template splitOverPermanent : NounPhrase := {
  spelling := "any target",
  body :=
    eitherOf
      (pro (.unionHalf .player) .one)
      (possessorOf .controller
        (pro (.unionHalf .permanent) .one))
}
