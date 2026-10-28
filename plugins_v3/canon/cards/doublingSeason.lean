/-- Doubling Season -/
card := {
  name := "Doubling Season",
  cost := [4, green],
  types := [enchantment],
  text := [
    replace
      (causedBy byAnEffect
         (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) you)))
      (instead (create (times 2 groupSize) asThose [])),
    replace
      (causedBy byAnEffect (would any (putCounters anyAmount anyKind (a [isPermanent, youControl]))))
      (instead (putCounters (times 2 thatMuch) thoseCounters (that permanent)))
  ]
}
