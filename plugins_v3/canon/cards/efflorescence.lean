/-- Efflorescence: "Put two +1/+1 counters on target creature. / Infusion — If you gained life
this turn, that creature also gains trample and indestructible until end of turn." -/
card efflorescence : Spelled :=
  { name := "Efflorescence", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ spell (putCounters 2 p1p1Counter (target isCreature)),
        infusion (spell (doIf (did you (gainLife anyAmount) soFarThisTurn)
          (establishFor (that creature) [abilityGrant it trample, abilityGrant it indestructible]
            untilEndOfTurn))) ] }
