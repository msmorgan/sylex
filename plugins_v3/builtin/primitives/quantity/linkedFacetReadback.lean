primitive linkedFacetReadback (which : Semantics.LinkSource) (within : Option Semantics.Duration) : Semantics.LinkedFacet := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.LinkedFacet.readback which within
}
