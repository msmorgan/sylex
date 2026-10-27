/-- "Put a lore counter on this Saga", done by the game as a turn-based action [CR#714.3c]. -/
template sagaLoreIncrement : Action := {
  spelling := "put a lore counter on this Saga",
  body :=
    .putCounters 1 (.printed (.named "loreCounter")) thisSaga
}
