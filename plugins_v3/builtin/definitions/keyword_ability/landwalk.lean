/-- A creature with landwalk cannot be blocked while the defending player controls a land matching
the land type, card type, supertype, or combination specified by its landwalk ability.
[CR#702.14c] -/
keyword landwalk (quality : Predicate) : Ability := {
  lexical := {
    lemma := "landwalk",
    parameters := [[], [Quality (Some ("of"))]],
    bound := { surface := "walk", stems := [LandType, CardType] }
  },
  -- TODO(spelling): Islandwalk / nonbasic landwalk; the Predicate needs a lowercase bound stem
  -- rather than its ordinary rendered phrase.
  body :=
    [static
        (onlyWhile
          (cant any (block thisCreature))
          (exists_
            (and
              [isLand, quality,
                controlledBy defendingPlayer])))]
}
