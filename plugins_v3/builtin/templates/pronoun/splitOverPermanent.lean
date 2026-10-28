/-- The same over "any target": the player, or the permanent's controller. -/
template splitOverPermanent : NounPhrase := {
  spelling := "any target",
  body :=
    eitherOf
      (nounPhrasePro (.unionHalf .player) .one)
      (possessorOf .controller
        (nounPhrasePro (.unionHalf .permanent) .one))
}
