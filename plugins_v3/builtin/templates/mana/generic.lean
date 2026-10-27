template generic (amount : Nat) : ManaSymbol := {
  spelling := "{<amount>}",
  body :=
    Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic amount)
}
