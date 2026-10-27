/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` for `slice`). -/
template lookAndSortSlice (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider := []) : Action := {
  -- TODO(spelling): Context-dependent wording: look at <slice>, put any number of them into <spill> [riders] and the rest on top of its owner's library in any order; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.expose Semantics.ExposeVerb.lookAt
          (Semantics.Templates.Exposed.cards (slice)),
        Semantics.Templates.Action.actionMove
          (Semantics.Templates.NounPhrase.someOf Semantics.Templates.Quantity.anyNumber
            (slice))
          Semantics.Templates.Location.wherever (Option.some spill) Semantics.ObservationPoint.arrival spillRiders,
        Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theRest Semantics.Templates.Location.wherever
          (Option.some Semantics.Templates.Location.onTop) Semantics.ObservationPoint.arrival
          [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.anyOrder]]
}
