/-- "{1} for each …": generic mana cost scaled by an amount. -/
template scaledGenericMana (amount : Amount) : Cost := {
  spelling := "{<amount>}",
  body :=
    scaled (mana [generic 1]) amount
}
