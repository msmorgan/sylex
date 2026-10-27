/-- "This Saga is the source of a chapter ability that has triggered but not yet left the stack"
[CR#714.4,113.7]. -/
template sagaHasTriggeredChapter : Condition := {
  spelling := "this Saga is the source of a chapter ability that has triggered but not yet left the stack",
  body :=
    exists_ (Semantics.Templates.Predicate.and [chapterAbilityOf thisSaga, triggeredNotLeftStack])
}
