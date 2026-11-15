---
needs: [cleanup-step6-surface]
specs: [voice-and-object-phrases]
---

# A replacement printed as one clause over a "would" subject is written as that clause

## Why

Some replacement effects print no "If … would": the event is a `would` relative on the clause's
subject, and the verb phrase says what happens in its place. *Carom* prints "The next 1 damage
that would be dealt to target creature this turn is dealt to another target creature instead."
[CR#614.9]; *Worship* prints "damage that would reduce your life total to less than 1 reduces it
to 1 instead"; *Trinisphere* prints "each spell that would cost less than three mana to cast
costs three mana to cast", with no "instead". 36 Vintage-legal cards print "damage that would be
dealt to … is dealt to …" (a regular expression over Oracle text, reminder text removed).

The corpus has one word for one case of it, `redirectDamage(cut, to, damage)` under
`replace(would(…), instead(…))`, which prints none of these sentences: *Carom* holds
`replace(would(any, dealDamage(recipient = target(isCreature))), instead(redirectDamage(theNext(1),
target(…))))`. *Worship* and *Trinisphere* are not written. The spec reads such a clause as a
replacement off its subject, as `prevent` reads its object (§8).

Written against the surface (`cleanup-step6-surface`), which has the damage phrase, the passive
and `would` as an operator.

## Open questions

- **What becomes of `redirectDamage`**, which the spec does not place (§6). Its 12 canon callers
  print three forms:
  - a passive clause over the `would` subject, six cards: *Carom*, *Daughter of Autumn*, *Martyrs
    of Korlis*, *Pariah* ("All damage that would be dealt to you is dealt to enchanted creature
    instead."), *Turn the Tables*, *Ward of Piety*;
  - an ordinary "The next time … would" replacement whose "instead" clause deals the damage, four
    cards: *Aegis of Honor* ("that spell deals that damage to its controller instead"), *Jade
    Monolith*, *Mirrorwood Treefolk* ("that damage is dealt to any target instead"), *Reflect
    Damage*;
  - an object relative under `would`, two cards: *Harm's Way* ("The next 2 damage that a source
    of your choice would deal to you and/or permanents you control this turn is dealt to any
    target instead.") and *Shining Shoal*. The spec does not design that relative (§11).

  Recommendation: the ten cards of the first two forms write the clauses they print, with `deal`
  and its passive; `redirectDamage` stays for *Harm's Way* and *Shining Shoal* until an object
  relative under `would` is designed, and goes then.
- **Where "instead" is written in a clause that is not under `instead`.** In the first form no
  `replace` and no `instead` word is in the term, and the card prints "instead" at the end of
  the verb phrase; *Trinisphere* prints none. The spec makes "instead" inside a clause a Template
  per placement (§11). Recommendation: a Template at `VerbPhrase`, worded "$phrase instead",
  whose body is its argument, as `syl-focus-also` writes "also".

## Done when

- The open questions are answered.
- The six cards of the first form are each one clause whose subject is the damage phrase with
  its `would` relative and whose verb phrase is the passive of `deal`, and the four of the second
  form write the clause they print under `instead`; each is checked against its Oracle text.
- *Worship* and *Trinisphere* are in `plugins/canon/cards/`, each a clause whose subject carries
  the `would` relative, checked against its Oracle text.
- That each of these clauses is a replacement, and the damage ones a redirection [CR#614.9], is
  semantics' reading (`docs/design/semantics.md` §6) and not checkable until it exists; the landing shows each
  term.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `prevent` over the same kind of phrase: `syl-prevent-damage-phrase`.
- *Lava Burst*'s "that damage can't be prevented or dealt instead to another permanent or
  player", a prohibition over two participles, which needs no `redirectDamage`.
- A line whose agent stands before another complement (*Impact Resonance*): not designed
  (spec §13).

## Pointers

- `docs/specs/voice-and-object-phrases.md` §6, §8, §11
- `plugins/builtin/primitives/action/redirectDamage.syl`
- `plugins/canon/cards/{Carom,Daughter of Autumn,Martyrs of Korlis,Pariah,Turn the Tables,Ward of Piety,Aegis of Honor,Jade Monolith,Mirrorwood Treefolk,Reflect Damage,Harm's Way,Shining Shoal}.syl`
