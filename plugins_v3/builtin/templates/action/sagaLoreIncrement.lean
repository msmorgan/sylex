/-- "Put a lore counter on this Saga", done by the game as a turn-based action [CR#714.3c]. -/
template sagaLoreIncrement : Action := {
  spelling := "put a lore counter on this Saga",
  body :=
    Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind (Semantics.CounterKind.named "loreCounter"))
      Semantics.Templates.NounPhrase.thisSaga
}
