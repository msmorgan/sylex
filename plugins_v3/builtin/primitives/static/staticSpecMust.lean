primitive staticSpecMust (subj : Semantics.Subject) (action : Semantics.Action) (mods : Semantics.DeonticMods) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.must subj action mods
}
