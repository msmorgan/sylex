---
needs: [lean-retirement]
---
# A replacement that excepts the nth event

*Notion Thief*: "If an opponent would draw a card except the first one they draw in each of their
draw steps, instead you skip that draw and you draw a card." The would-clause can select the nth
event (`WouldClause.nthOccurrence`) but cannot exclude it, so the card is unspelled
(`idris-retire-cards-idr`, which migrated the rest of `Cards.idr`). Add an exclusion of the nth
occurrence in a period to the would-clause (or a general "except" on an event pattern), spell
*Notion Thief* in `plugins/canon/cards/`, and test that the first draw in its controller's
opponent's draw step is not replaced while a second draw that step is.
