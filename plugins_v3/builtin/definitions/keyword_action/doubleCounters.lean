/-- "double the number of each kind of counter on <on>" (`Definition`, [CR#701.10e]): for each
kind of counter on it, put as many of that kind on it as it already has. -/
action doubleCounters (on : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "double", frames := [Monotransitive] }] },
  deed := "double",
  spelling := "double the number of each kind of counter on <on>",
  body :=
    Semantics.Templates.Action.actionDoForEachKind Semantics.KindAxis.counterKind (Option.some on) Semantics.QualitySort.counterKind
      (Semantics.Templates.Action.putCounters (Semantics.Templates.Amount.statOf Semantics.ProjAxis.boundCounter on)
        Semantics.Templates.CounterKindSource.thatKind on)
}
