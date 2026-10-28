/-- "This Saga has one or more chapter abilities": its final chapter number is at least I
[CR#714.2d]. -/
template sagaHasChapterAbilities : Condition := {
  spelling := "this Saga has one or more chapter abilities",
  body :=
    compareAmt (finalChapterNumber thisSaga)
      Semantics.Comparator.atLeast 1
}
