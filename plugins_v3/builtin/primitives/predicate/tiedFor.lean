/-- "tied for least toughness", "tied for lowest life total": with the extreme `axis` among
`domain`, as `superlative` is. The subject's count ("two or more creatures are tied …") makes it
a tie. -/
primitive tiedFor (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) : Predicate := {
  -- TODO(spelling): tied for least toughness; wording varies with axis, domain, op, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Predicate.superlative op axis domain
}
