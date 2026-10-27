template isInstantOrSorcery : Predicate := {
  spelling := "instant or sorcery",
  body :=
    or [isInstant, isSorcery]
}
