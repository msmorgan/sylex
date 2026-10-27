/-- `action` done as part of the replaced event, which still happens ("with" it): "that player may
put … counters on it as it enters", "you may have this permanent enter with an additional +1/+1
counter on it" [CR#614.1c,702.104a,702.136a]. -/
-- [CR#614.1c,702.104a,702.136a]
primitive alongside (action : Action) : Replacement := {
  -- TODO(spelling): with; wording varies with action, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Replacement.alongside action
}
