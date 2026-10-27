/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
card manaLeak : Spelled :=
  { name := "Mana Leak", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless
          (instruct you (counter it))
          [generic 3])) ] }
