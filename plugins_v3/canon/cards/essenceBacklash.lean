/-- Essence Backlash: "Counter target creature spell. Essence Backlash deals damage equal to
that spell's power to its controller." Both information reads name the departed spell
[CR#608.2h]. -/
card essenceBacklash : Spelled :=
  { name := "Essence Backlash", cost := [generic 2, pip .blue, pip .red], types := [.instant],
    text := [spell (sequentially
      [ counter (target (and [isCreature, isSpell])),
        instruct thisSpell (dealDamage (statOf (.stat .power) (that .spell)) (controllerOf it)) ])] }
