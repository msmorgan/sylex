/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
template splitOverPlaneswalker : NounPhrase := {

  -- TODO(spelling): that player or that permanent's controller; the subject's union half selects
  -- the phrase, which string substitution cannot branch on.
  body :=
    eitherOf
      (nounPhrasePro (Semantics.Reach.unionHalf Semantics.NounWord.player) Semantics.Plurality.one)
      (possessorOf Semantics.PossessorAxis.controller
        (nounPhrasePro (Semantics.Reach.unionHalf (Semantics.NounWord.type Semantics.CardType.planeswalker))
          Semantics.Plurality.one))
}
