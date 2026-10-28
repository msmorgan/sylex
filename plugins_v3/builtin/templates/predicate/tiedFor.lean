/-- "tied for least toughness", "tied for lowest life total": with the extreme `axis` among
`domain`, as `superlative` is. The subject's count ("two or more creatures are tied …") makes it
a tie. -/
template tiedFor (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) : Predicate := {
  spelling := "tied for the <op> <axis> among <domain>",
  body := superlative op axis domain
}
