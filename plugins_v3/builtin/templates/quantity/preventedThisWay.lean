/-- "that much damage prevented this way" -/
template preventedThisWay : Amount := {
  spelling := "that much damage prevented this way",
  body := theOutcome .damagePrevented
}
