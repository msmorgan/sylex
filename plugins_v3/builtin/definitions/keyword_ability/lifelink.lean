/-- "Lifelink": damage dealt by this also causes its controller, or its owner if it has no
controller, to gain that much life, in addition to the damage's other results
[CR#702.15b,120.3f,109.5]. Multiple instances of lifelink on one object are redundant
[CR#702.15f]: a consumer applies this body once per object, not once per instance. -/
keyword lifelink : Ability := {
  lexical := { lemma := "lifelink" },
  spelling := "lifelink",
  body :=
    [static (asEvent (does this_ (dealDamage anyAmount)) (gainLife thatMuch))]
}
