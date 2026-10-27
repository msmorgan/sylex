template newPerspectivesCyclingAltCost : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (onlyWhile (altCost (allOf (abilityHead (.keyword cycling))) [generic 0])
    (compareAmt (countOf (cardIn yourHand)) .atLeast 7))
}
