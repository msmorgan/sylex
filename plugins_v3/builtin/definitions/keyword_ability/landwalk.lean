keyword landwalk (quality : Predicate) : Ability := {
  lexical := {
    lemma := "landwalk",
    parameters := [[], [Quality (Some ("of"))]],
    bound := { surface := "walk", stems := [LandType, CardType] }
  },
  -- TODO(spelling): Islandwalk / nonbasic landwalk; the Predicate needs a lowercase bound stem
  -- rather than its ordinary rendered phrase.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))
          (Semantics.Templates.Condition.exists_
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isLand, quality,
                Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.defendingPlayer])))]
}
