/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
action fight (other : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "fight", past := "fought", frames := [Monotransitive, Intransitive] }] },
  permanent := performs,
  spelling := "fight <other>",
  body :=
    Semantics.Templates.Action.doIf
      (Semantics.Templates.Condition.and
        [Semantics.Templates.Condition.is other Semantics.Templates.Predicate.isCreature,
          Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.theActor Semantics.Templates.Predicate.isCreature])
      (Semantics.Templates.Action.simultaneously
        [Semantics.Templates.Action.dealDamage
            (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Templates.NounPhrase.theActor) other,
          Semantics.Templates.Action.instruct other
            (Semantics.Templates.Action.dealDamage (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) other)
              Semantics.Templates.NounPhrase.theActor)])
}
