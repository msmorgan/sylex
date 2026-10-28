/-- "each <value of `axis`> [among <domain>]" as a keyword's quality, as "protection from each
color among permanents you control": one ability per value [CR#702.16h,702.16i]. -/
-- [CR#702.16h,702.16i]
primitive eachKind (axis : KindAxis) (domain : Option NounPhrase := none) : Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by domain.
  body :=
    Semantics.Primitives.Predicate.eachKind axis domain
}
