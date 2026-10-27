/-- "each <value of `axis`> [among <domain>]" as a keyword's quality, as "protection from each
color among permanents you control": one ability per value [CR#702.16h,702.16i]. -/
-- [CR#702.16h,702.16i]
primitive eachKind (axis : KindAxis) (domain : Option NounPhrase := none) : Predicate := {
  -- TODO(spelling): Context-dependent wording: each <value of `axis`> [among <domain>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Predicate.eachKind axis domain
}
