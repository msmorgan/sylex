/-- Malamet Battle Glyph: "Choose target creature you control and target creature you don't
control. If the creature you control entered this turn, put a +1/+1 counter on it. Then those
creatures fight each other." -/
card malametBattleGlyph : Spelled :=
  { name := "Malamet Battle Glyph", cost := [pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ chooseFightPair,
            doIf (did (the (and [isCreature, youControl])) enter soFarThisTurn)
              (putCounters 1 p1p1Counter it),
            fightEachOther (those creature) ]) ] }
