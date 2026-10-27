/-- "a chapter ability of <saga>" [CR#714.2,714.2a] -/
-- [CR#714.2,714.2a]
template chapterAbilityOf (saga : NounPhrase) : Predicate := {
  spelling := "a chapter ability of <saga>",
  body :=
    Semantics.Templates.Predicate.and
      [Semantics.Templates.Predicate.abilityOf saga,
        Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.chapter] Semantics.Comparator.atLeast 1]
}
