/-- "<subject> has been under its controller's control continuously since their most recent turn
began" [CR#302.6]: the summoning-sickness reading, its controller the present one. -/
-- [CR#302.6]
template controlledSinceTheirTurnBegan (subject : NounPhrase) : Condition := {
  spelling := "<subject> has been under its controller's control continuously since their most recent turn began",
  body :=
    Semantics.Templates.Condition.heldSince
      (Semantics.Templates.Condition.is subject
        (Semantics.Templates.Predicate.controlledBy (Semantics.Templates.NounPhrase.controllerOf Semantics.Templates.NounPhrase.it)))
      (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.turn Option.none
        (Semantics.Templates.TurnRef.turnRefMostRecent Semantics.Templates.NounPhrase.they))
}
