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
            (statOf (Semantics.ProjAxis.stat Semantics.Stat.power) theActor) other,
          instruct other
            (dealDamage (statOf (Semantics.ProjAxis.stat Semantics.Stat.power) other)
              theActor)])
}
