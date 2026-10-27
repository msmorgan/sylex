/-- "while you're searching your library" -/
template whileSearchingYourLibrary : Timing := {
  spelling := "while you're searching your library",
  body :=
    Semantics.Templates.Timing.whileDoing
      (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
        (Semantics.Templates.Action.actionSearch Semantics.Templates.Location.yourLibrary Semantics.Templates.NounPhrase.any))
}
