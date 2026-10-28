/-- "{A/B}" -/
template hybridPip (left right : Color) : ManaSymbol := {
  spelling := "{<left>/<right>}",
  body :=
    .hybrid (.specific (.of left)) right
}
