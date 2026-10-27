template castFromTop (p : Predicate) : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := can you (cast (bare p) (from_ := onTop))
}
