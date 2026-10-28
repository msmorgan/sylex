/-- The untap-symbol cost, {Q}. -/
primitive untapSymbol : Cost := {
  spelling := "{Q}",
  body :=
    Semantics.Primitives.Cost.untapSymbol
}
