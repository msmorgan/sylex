primitive playPaymentFaceDown (spec : Semantics.FaceDown) (cost : Semantics.Cost) : Semantics.PlayPayment := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.PlayPayment.faceDown spec cost
}
