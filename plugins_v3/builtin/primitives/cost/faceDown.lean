primitive faceDown (spec : FaceDown) (cost : Cost) : PlayPayment := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.PlayPayment.faceDown spec cost
}
