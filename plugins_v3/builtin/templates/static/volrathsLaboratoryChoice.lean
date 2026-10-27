template volrathsLaboratoryChoice : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := conjunction [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
}
