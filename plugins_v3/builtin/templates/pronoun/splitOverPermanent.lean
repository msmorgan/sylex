/-- The same over "any target": the player, or the permanent's controller. -/
template splitOverPermanent : NounPhrase := {
  spelling := "any target",
  body :=
    Semantics.Templates.NounPhrase.eitherOf
      (Semantics.Templates.NounPhrase.nounPhrasePro (Semantics.Reach.unionHalf Semantics.NounWord.player) Semantics.Plurality.one)
      (Semantics.Templates.NounPhrase.possessorOf Semantics.PossessorAxis.controller
        (Semantics.Templates.NounPhrase.nounPhrasePro (Semantics.Reach.unionHalf Semantics.NounWord.permanent) Semantics.Plurality.one))
}
