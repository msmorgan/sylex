/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` for `slice`). -/
template lookAndSortSlice (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider := []) : Action := {
  -- TODO(spelling): Format and coordinate the spillRiders list entries, including the empty case.
  body :=
    sequentially
      [expose lookAt
          (cards (slice)),
        move
          (someOf anyNumber
            (slice))
          wherever (some spill) .arrival spillRiders,
        move theRest wherever
          (some onTop) .arrival
          [inOrder anyOrder]]
}
