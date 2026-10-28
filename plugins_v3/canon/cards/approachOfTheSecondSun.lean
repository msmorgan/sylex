/-- Approach of the Second Sun -/
card := {
  name := "Approach of the Second Sun",
  cost := [6, white],
  types := [sorcery],
  text := [
    spell
      (doIf
         [
           is this_ (did any (cast which (from_ := yourHand)) thisObject),
           did you (cast (a [isSpell, isOther, named "Approach of the Second Sun"])) thisGame
         ]
         winGame [putInto this_ (fromTop 7 library), gainLife 7])
  ]
}
