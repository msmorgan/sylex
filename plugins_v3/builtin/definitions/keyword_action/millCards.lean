/-- "<cards> are milled": the mill whose patient is a card description, as an event a trigger
watches ("Whenever one or more nonland cards are milled") (`Definition`, [CR#701.17a,701.17c]). -/
action millCards (cards : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "mill", frames := [Monotransitive] }] },
  deed := "mill",
  spelling := "mill <cards>",
  body :=
    move cards library (some graveyard)
      .arrival []
}
