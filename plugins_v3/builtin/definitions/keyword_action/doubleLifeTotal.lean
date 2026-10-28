/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set
[CR#119.5]. -/
action doubleLifeTotal : Action := {
  lexical := { verbs := [{ lemma := "double", frames := [Monotransitive] }] },
  deed := "double",
  spelling := "double your life total",
  body :=
    changeLife
      (setTo (times 2 (lifeTotalOf actor)))
}
