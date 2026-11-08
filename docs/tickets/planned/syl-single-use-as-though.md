---
needs: [lean-v3-checker-and-drt-fixes, plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, syl-keyword-verb-events]
---

# A single-use "as though" statement

**Why.** *Biomancer's Familiar*: "{T}: The next time target creature adapts this turn, it adapts
as though it had no +1/+1 counters on it." `asThough`, an "as though" statement [CR#609.4], covers
every event it names for as long as it is in force; only replacements carry a single use
(`nextTimeOnly`). The card is not a replacement [CR#614.1]: its Gatherer ruling says it "just lets
the creature adapt despite already having +1/+1 counters on it", so the creature still adapts. So
the card cannot be written: `asThough … thisTurn` covers every adapt that turn, and `nextTimeOnly`
makes it a replacement.

**Done when.**
- An as-though statement can be limited to the next matching event (a use field on `asThough`,
  or a usage limit on `establish`; the landing says which and why).
- *Biomancer's Familiar* is in `plugins/canon/cards/`, its "adapts" being the event
  `syl-keyword-verb-events` defines, and passes the `.syl` checks.
- A test pins that a second adapt the same turn is not covered.

**Pointers.** `plugins/builtin/primitives/static/asThough.syl`,
`plugins/builtin/templates/action/replaceNextEvent.syl`,
`plugins/builtin/definitions/keyword_action/adapt.syl`.
