primitive faceDown (spec : FaceDown) (cost : Cost) : PlayPayment := {
  body := Semantics.Primitives.PlayPayment.faceDown spec cost
}
