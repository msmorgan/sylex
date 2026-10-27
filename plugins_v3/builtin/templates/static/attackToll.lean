template attackToll : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := gatedBy (scaledGenericMana
      (forEach 2
         (and [isCreature, controlledBy they, inCombat .attackerOf you]))) (actor
                                                                                     (allOf
                                                                                        isCreature)) (attack you)
}
