/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` for `slice`). -/
template lookAndSortSlice (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider := []) : Action := {
  -- TODO(spelling): Context-dependent wording: look at <slice>, put any number of them into <spill> [riders] and the rest on top of its owner's library in any order; string substitution cannot express its omitted or inflected parts.
  body :=
    sequentially
      [expose .lookAt
          (cards (slice)),
        actionMove
          (someOf anyNumber
            (slice))
          wherever (some spill) .arrival spillRiders,
        actionMove theRest wherever
          (some onTop) .arrival
          [inOrder .anyOrder]]
}
