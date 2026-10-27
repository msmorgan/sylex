primitive worth (n : Nat) : Quantity := {
  spelling := "<n> worth of",
  body :=
    Semantics.Primitives.Quantity.worth n
}
