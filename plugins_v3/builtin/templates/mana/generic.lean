template generic (amount : Nat) : ManaSymbol := {
  spelling := "{<amount>}",
  body :=
    .simple (.generic amount)
}
