/-- Forage: exile three cards from your graveyard or sacrifice a Food. [CR#701.61a] -/
action forage : Action := {
  lexical := { verbs := [{ lemma := "forage", frames := [Intransitive] }] },
  spelling := "forage",
  body :=
    chooseOne
      (exileFrom
        (counted (exactly 3)
          (cardIn (graveyardOf actor)))
        (graveyardOf actor))
      (sacrifice
        (a
          (and
            [hasSubtype food, isPermanent,
              actorControls])))
}
