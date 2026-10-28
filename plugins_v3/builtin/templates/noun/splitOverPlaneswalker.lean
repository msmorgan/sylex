/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
template splitOverPlaneswalker : NounPhrase := {

  -- TODO(spelling): that player or that permanent's controller; the subject's union half selects
  -- the phrase, which string substitution cannot branch on.
  body :=
    eitherOf
      (pro (.unionHalf .player) .one)
      (possessorOf .controller
        (pro (.unionHalf (.type Semantics.CardType.planeswalker))
          .one))
}
