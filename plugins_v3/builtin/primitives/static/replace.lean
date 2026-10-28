/-- "If <event> would …, <replacement> instead", "<subject> enters tapped", "As <subject> enters,
…": a replacement effect, `event` modified by `replacement` [CR#614.1a,614.1c,614.1d]. -/
-- [CR#614.1a,614.1c,614.1d]
primitive replace (event : WouldClause) (replacement : Replacement)
    (timing : Option Timing := none) (use : ReplUse := .repeatedly)
    (limit : Option UsageLimit := none) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: If <event> would …, <replacement> instead; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.StaticSpec.replace event replacement timing use limit
}
