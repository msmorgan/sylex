---
needs: [lean-generated-canon-modules, lean-retirement]
---

# A condition-first static: "As long as <condition>, <spec>"

## Why

The only "as long as" static is the Template `onlyWhile`, spelled "<spec> as long as
<condition>", over the primitive `conditional`. Cards that print the condition first (*Angelic
Cub*: "As long as this creature has three or more +1/+1 counters on it, it has flying"; also
*Escaped Shapeshifter*, *Martyrs of Korlis*, *Guardian Beast*, *Thunderstaff*, *Thranduil's
Company*) cannot be written in printed order. Written with `onlyWhile`, the static is read before
the condition, so the round trip breaks and the static's "it" is read before the condition that
introduces its antecedent. `Angelic Cub.syl` therefore carries only its triggered ability.

## Done when

- A condition-first Template exists whose parameters are in printed order, condition then static.
- *Angelic Cub* carries both printed abilities, with "it" in the grant written as `it`, and its
  "Fragment" comment is gone; the other condition-first cards in canon use the new Template.
- Everything passes the `.syl` checks. Once reference resolution exists, a test pins that the
  grant's "it" reads this creature from the condition.

## Pointers

`plugins/builtin/templates/static/onlyWhile.syl`,
`plugins/canon/cards/Angelic Cub.syl`, `docs/decisions/syl.md` §5.
