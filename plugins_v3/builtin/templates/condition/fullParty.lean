template fullParty : Condition := {
  spelling := "you have a full party",
  body :=
    Semantics.Templates.Condition.fullPartyOf Semantics.Templates.NounPhrase.you
}
