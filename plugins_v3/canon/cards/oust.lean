/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
card oust : Spelled :=
  { name := "Oust", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putInto (target isCreature) (fromTop 2 library),
            instruct (controllerOf it) (gainLife 3) ]) ] }
