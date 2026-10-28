/-- "pay <amount> life": a cost payment that subtracts that much life, its payer losing that much
life [CR#119.4]; a player who can't lose life can't pay it [CR#119.8]. In a cost slot it is
coerced to `perform (payLife amount)`, one payment of that life. -/
-- [CR#119.4]
-- [CR#119.8]
template payLife (amount : Amount := anyAmount) : Action := {
  spelling := "pay <amount> life",
  body :=
    pay (perform (loseLife amount)) .once
}
