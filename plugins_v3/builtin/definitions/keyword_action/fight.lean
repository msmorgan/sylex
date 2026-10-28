/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
action fight (other : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "fight", past := "fought", frames := [Monotransitive, Intransitive] }] },
  permanent := performs,
  spelling := "fight <other>",
  body :=
    doIf
      (and
        [is other isCreature,
          is theActor isCreature])
      (simultaneously
        [dealDamage
            (statOf (.stat .power) theActor) other,
          instruct other
            (dealDamage (statOf (.stat .power) other)
              theActor)])
}
