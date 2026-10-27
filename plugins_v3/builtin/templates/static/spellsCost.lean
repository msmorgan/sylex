template spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := costShift (allOf p) shift
}
