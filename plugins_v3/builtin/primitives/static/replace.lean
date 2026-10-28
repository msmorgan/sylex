/-- "If <event> would …, <replacement> instead", "<subject> enters tapped", "As <subject> enters,
…": a replacement effect, `event` modified by `replacement` [CR#614.1a,614.1c,614.1d]. -/
-- [CR#614.1a,614.1c,614.1d]
primitive replace (event : WouldClause) (replacement : Replacement)
    (timing : Option Timing := none) (use : ReplUse := .repeatedly)
    (limit : Option UsageLimit := none) : StaticSpec := {
  -- TODO(spelling): Include or omit the clauses supplied by timing, limit.
  body :=
    Semantics.Primitives.StaticSpec.replace event replacement timing use limit
}
