/-- [CR#120.3g,702.164c] "Combat damage dealt to a player by a creature with toxic causes that
creature's controller to give the player a number of poison counters equal to that creature's
total toxic value, in addition to the damage's other results." One row for the creature,
reading the sum of its toxic abilities' N values [CR#702.164b], not one per toxic ability: the
keyword is bodiless, so combat damage from a creature with toxic 2 and toxic 1 gives three poison
counters once. -/
conferral toxicPoison := {
  body :=
    { scope := .and [isCreature, .hasKeyword (.the (.named "toxic"))],
      confer := .property
        (asEvent (does this_ (dealDamage anyAmount (a isPlayer) (some .combat)))
          (putCounters (totalToxicValue this_) poison (that player))) }
}
