/-- Destructive Revelry -/
card destructiveRevelry : Spelled :=
  { name := "Destructive Revelry", cost := [pip .red, pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ destroy (target (or [isArtifact, isEnchantment])),
            instruct this_ (dealDamage 2 (controllerOf (that permanent))) ]) ] }
