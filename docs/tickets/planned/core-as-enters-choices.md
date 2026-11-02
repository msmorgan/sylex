---
needs: []
---
"As this enters, choose …" (color/type/number/opponent) with stored choices
readable by other abilities. Feeds Leyline of Sanctity-style naming and
Morophon-style type-choice statics.

**Named-mode choice (Citadel Siege, Outpost Siege).** "As this enchantment enters, choose Khans
or Dragons." followed by two bulleted abilities, each in force only for the chosen mode. The
`.syl` model cannot write it yet: an entry choice chooses a quality (`QualitySort`: color,
subtype, name, number, card type, counter kind, ability), not one of two printed mode labels,
and no condition reads which label was chosen. The retired `idris/src/Cards.idr` modeled the
choice as one ability wrapping both triggered abilities, each gated on the chosen mode (the
unchosen one's effect inert, not the ability absent):

- **Citadel Siege** — Khans: at the beginning of combat on your turn, put two +1/+1 counters on
  target creature you control. Dragons: at the beginning of combat on each opponent's turn, tap
  target creature that player controls.
- **Outpost Siege** — Khans: at the beginning of your upkeep, exile the top card of your
  library; until end of turn, you may play that card. Dragons: whenever a creature you control
  leaves the battlefield, this enchantment deals 1 damage to any target.
