/-- The same over "any target": the player, or the permanent's controller. -/
template splitOverPermanent : NounPhrase := {
  spelling := "any target",
  body :=
    eitherOf
      (nounPhrasePro (Semantics.Reach.unionHalf Semantics.NounWord.player) Semantics.Plurality.one)
      (possessorOf Semantics.PossessorAxis.controller
        (nounPhrasePro (Semantics.Reach.unionHalf Semantics.NounWord.permanent) Semantics.Plurality.one))
}
