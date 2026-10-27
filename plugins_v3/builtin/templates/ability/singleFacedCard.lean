/-- A single-faced card, written as the characteristics of its one Card Face. -/
template singleFacedCard (characteristics : Characteristics) : Card := {
  -- TODO(spelling): A single-faced card, written as the characteristics of its one Card Face.
  body :=
    Semantics.Card.singleFaced { characteristics := characteristics }
}
