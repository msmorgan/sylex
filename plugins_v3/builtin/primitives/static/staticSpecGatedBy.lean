primitive staticSpecGatedBy (cost : Semantics.Cost) (subj : Semantics.Subject) (action : Semantics.Action) (mods : Semantics.DeonticMods) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.gatedBy cost subj action mods
}
