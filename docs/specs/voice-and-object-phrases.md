# Voice, object phrases and shared phrases: a clause is a subject and a verb phrase

## Status: DRAFT

This spec gives direction, not a settled design. Three of the tickets that build it have landed
(`syl-verb-phrase-sort-name`, `syl-definition-actor-parameter`, `syl-misnamed-declarations`); the
rest of it has not been built or run through `cargo xtask dsl check`. Where building it shows a
rule here to be wrong or incomplete, the finding stands and this text is revised; a builder
reports the conflict and does not bend the code to fit the spec.

Known to be unsettled, each a question on the ticket named:

- **A participle is a sort of its own** (`docs/specs/kind-sorts-and-position-unions.md` §2).
  `done` produces a `Participle`, as does `ing` for the active participle, `be` takes one, and
  where this spec says a verb phrase stands in a nominal's `post` as a participle, a `Participle`
  does. §3 still writes `shared` over an active verb phrase and a `done` phrase in `post` (*Abu
  Ja'far*) and `be` over `shared` (*Cephalid Illusionist*), and says `be` over an active phrase
  is the progressive: `syl-passive-participle-and-trigger-events`.
- **A lookback after a head.** §8 puts `did(phrase, within)`, a verb phrase, in `post`, and writes
  "creatures destroyed this way" as `did` over a `done` phrase; neither types once a verb phrase
  is no member of `PostModifier`: `syl-lookback-operator-and-holds`.
- **Negation over `be`** ("isn't blocked", "wasn't cast", about 85 lines), which §11 leaves
  undesigned: `syl-passive-participle-and-trigger-events`.

A clause's subject is the noun phrase the card prints as its subject, and everything else a
clause says is its verb phrase: the verb with its complements, a passive, a modal. Damage,
counters, life and tokens are noun phrases like any other. A phrase that coordinated verb phrases
share is written once, after them.

The `.syl` below is the shape of the design. It has not been run through `cargo xtask dsl check`.
A word is marked NEW where this spec adds it and SKETCH where another ticket owns its final form
(`nominal` and its modifiers, the relative words, the coordinators and sequencing words at each
sort); every other word exists today, apart from the retyping this spec states. Corpus
counts are evidence of size, each with its measure:

- **Lines** are distinct Oracle lines matched by a regular expression in the `mtg-rules` corpus,
  which is not restricted to Vintage-legal cards. They are orders of magnitude, not parses.
- **Units** are Corpus Units of Vintage-legal cards in `data/derived/english-forests.sqlite` with
  at least one alternative built by the named Production. A unit counts if any of its Readings
  uses the Production, so these are upper bounds.
- **Calls** are `grep` counts over `plugins/canon/` and `plugins/builtin/`.

## 1. The clause

Two sorts, named for the English grammar's Categories of the same names
(`crates/english/grammar/english.fug`):

- **`VerbPhrase`** (NEW name for the sort `Action`): a verb phrase without its subject. Every
  declaration at `Action` today stands at it, and so do the subjectless continuous-effect phrases
  of `syl-continuous-effect-subject` ("gets +1/+0", "has flying"), the "enters" family ("enters
  tapped") and the modal and negative phrases of §5 ("can't be blocked").
- **`Clause`** (NEW): a subject with its verb phrase, or an imperative.

      primitive clause(subject: NounPhrase, phrase: VerbPhrase): Clause = Clause {   # NEW
        spelling = "$subject $phrase@Present"
      }
      # NEW. The grammar's imperative governs the plain form, so its phrase takes no label.
      primitive imperative(phrase: VerbPhrase): Clause = Imperative

`clause` is the one declaration that gives a verb phrase a printed subject. It is the
subject-giving declaration `syl-continuous-effect-subject` asks for, and it replaces `instruct`:
"each opponent gains that much life" (*Grollub*) and "Target creature gets +3/+3 and gains
trample until end of turn" (*Predator's Strike*) are both `clause(…, …)`. Whether the subject
performs anything is the verb phrase's to say (§4), not the declaration's, so one declaration
serves a player who discards and a creature that gets +3/+3.

**No coercion joins the two sorts.** An effect with no printed subject is written
`imperative(…)` at each site; the controller of the spell or ability follows its instructions
[CR#608.2c], so its understood subject is that player. The grammar coordinates at both levels
with the same printed words: *Faithless Looting*'s "Draw two cards, then discard two cards" is a
coordination of two imperative clauses, and *Wheel of Fortune*'s "Each player discards their
hand, then draws seven cards" a coordination of two verb phrases under one subject. So the
coordinators and the sequencing words are declared at `Clause` and at `VerbPhrase`, which nothing
refuses while no coercion links the sorts (`syl.md` §3).

The sort `Subject` and its three words `actor(np)`, `any` and `by(np)` are removed, and so is
`instruct`. What took a `Subject` and an `Action` takes a noun phrase and a verb phrase:

| Position | Takes | Today |
|---|---|---|
| an effect with a printed subject | `clause(subject, phrase)` | `instruct(subj, action)`; the subject-first declarations |
| an effect with no printed subject | `imperative(phrase)` | an `Action` |
| a cost | a `VerbPhrase` | an `Action` |
| a static ability's statement | a `Clause`, standing at `StaticSpec` | a `StaticSpec` that holds its own subject |
| a trigger event | `does(subject: NounPhrase, phrase: VerbPhrase)` | `does(subj: Subject, action)` |
| a condition stated as a clause | `holds(subject: NounPhrase, phrase: VerbPhrase)` (NEW) | `did(subj, action, within)` |
| a lookback | the operator `did(phrase, within)` (§8) | `did(subj: Subject, action, within)`, `was` |
| a replacement's event | a `Clause` whose phrase is `would(…)` (§8) | `would(subj: Subject, action)` |
| a deontic rule, a negative declarative | a verb-phrase operator (§5) | `cant(subj, action, …)`, `doesnt(subj, action)` |

**The words that hold effects take a `Clause`**: `sequentially`, `doIf`, `doUnless`, `instead`,
and the effect of `spell`, `activated` and the trigger words. `instruct` stands inside them 209
times in canon and 29 in builtin, by `grep`; each becomes a `clause`.

**`StaticSpec` stays, and a clause stands in it through one coercion**, from `Clause` to
`StaticSpec` (NEW; `syl.md` §2 counts three coercions today). Of the 82 declarations at
`StaticSpec` under `plugins/builtin/`:

- 60 take a parameter named `subject` or `subj` (`getsPt`, `abilityGrant`, `cant`,
  `entersTapped`). Each that prints it as its subject gives it up and becomes a `VerbPhrase`
  under `clause`. Some print it elsewhere (`altCost`'s "You may $cost rather than pay $subject's
  mana cost", `controlGrant`'s "$player controls $subject"); `syl-continuous-effect-subject`
  lists which. `getsBoth`, declared at both `Action` and `StaticSpec` today, becomes one
  declaration.
- 10 wrap another `StaticSpec` (`asLongAs`, `if`, `conditionally`, `conjunction`, `onlyWhile`).
  They keep their sort and wrap a clause through the coercion.
- 12 do neither (`replace`, `additionalTriggers`, `specialAction`, `partSkip`, `manaRetention`,
  `visibility`, `letterDefinition`, `asEvent`, `whileHolds`, `rollModifier`, `modeChoice`,
  `resolutionDelayedTrigger`) and are unchanged.

The self-replacement `replace` ("X. Y instead", today at `Action`) holds sentences, so it stands
at `Clause` and takes another name: `Clause` is in Reaches(`StaticSpec`), where the static
`replace` stands.

**`does` and `holds` make an event and a condition of a subject and a phrase.** A trigger event
is a `GameEvent`, which durations and `reached` also build, so `does` stays. `holds` is its twin
at `Condition`: *Mardu Hordechief*'s "if you attacked this turn" is
`holds(you, did(attack, within = thisTurn))`. Both word as `clause` does.

**`may` and `have`.** `may` is a verb-phrase operator, "may $body". The sentences that follow an
optional action belong to a word at `Clause` that holds the optional clause and them, today's
`withContinuation` retyped. *Apathy*'s "that player may discard a card at random. If the player
does, untap that creature." is

    withContinuation(clause(that(player), may(discard(…))),
      ifDid = imperative(untap(that(creature))))

§13 item 6 reopens this shape, so `syl-clause-and-imperative` does not build it: until that is
settled `may` keeps its `ifDid` and `ifNot`, retyped to clauses, and holds the follow-up sentence
inside the verb phrase as it does today.

A printed "have" with a noun phrase and a bare verb phrase is `have` (NEW), a verb phrase in which
the one named performs the inner phrase: *Dire Undercurrents*'s "you may have target player draw
a card" is `clause(you, may(have(target(player), draw(a(card)))))`.

    primitive have(performer: NounPhrase, phrase: VerbPhrase): VerbPhrase = Have   # NEW

**The form of a clause's verb.** Nothing governs the verb of a clause whose subject and verb
phrase are two Holes: the grammar's clause takes any finite verb (`SubjectPredicate`,
`head.VerbForm = Finite`). So `clause` states the form, the present, in which an effect and a
static statement are printed. A form label does not apply where the argument's own wording
already fixes its head's form, by a fixed word or by a label of its own. The lexicon declares
"would" as the preterite of *will*, and "damage would be dealt" stands under `clause` all the
same; `did` labels its own phrase under `holds`; and `done`'s participle stands under `did`
(§8).

Every declaration that prints its own subject inside its wording gives it up, and its subject is
the clause's: the 60 above, `becomeBlocked` ("$subject becomes blocked"), `resolve`, and `may`.
`syl-continuous-effect-subject` recommends leaving the "enters" family subject-first; this spec
does not, because the cards coordinate it under one subject: *Grimgrin, Corpse-Born*'s "Grimgrin
enters tapped and doesn't untap during your untap step".

A verb phrase is one of three kinds, and what a clause means follows from its head. A deed is
done as the clause resolves. A continuous-effect phrase states what holds: in a static ability's
clause for as long as the ability applies [CR#611.3b], and in a resolving effect for its
duration, or until the game ends when none is printed [CR#611.2a]. An "enters" phrase states a
replacement effect with no "would" [CR#614.1c,614.1d]. The kinds coordinate: *Ent's Fury* prints
"Then that creature gets +1/+1 until end of turn and fights target creature you don't control".

## 2. What a verb phrase is said of

Every verb phrase is said of one thing, supplied by where the phrase stands:

- under `clause`, `does` or `holds`, the subject's noun phrase;
- in a nominal's postmodifier list, as a participle or a subject relative ("a creature dealt
  damage by this creature this turn", "damage that would be dealt to you"), the thing the nominal
  describes;
- as an imperative or a cost, the player who carries it out.

Two words at `NounPhrase` read it from inside a verb phrase. Today one word, `actor`, does both
jobs: `die`, `enter`, `leave`, `becomeStatus`, `phaseIn` and `phaseOut` place it as the thing
their event happens to (`die` is `move(actor, …)`), and `mill`, `scry`, `shuffle`, `discard`,
`fight`, `explore` and `connive` read it as whoever performs them (`mill` moves cards from
`libraryOf(actor)`).

- **`subject`** (NEW; the first reading of today's `actor`) reads what the verb phrase is said
  of, as a participant of its event: the thing a passive promotes (§3), the thing that dies,
  untaps or gets counters (§4), the object a continuous effect changes. It prints no word and has
  no `spelling`, as `which` has none.
- **`actor`** reads the performer of the verb phrase (§4).

Both words rebind wherever a clause inside names another subject, so neither reaches from inside
such a clause to the performer of an enclosing Definition's call. A Definition's body reads that
through its `$actor` (§4).

`subject` is read where it stands, so the term's order is the printed order in every voice, and
`syl.md` §4's exception for a passive agent goes.

## 3. The passive

The passive is two words on the verb-phrase side, the participle and its auxiliary:

    # NEW. "countered", "dealt to this creature", "blocked by it"
    # Its wording is the passive of $phrase's frame, with $by as its agent (below).
    primitive done(phrase: VerbPhrase, by: Option(NounPhrase) = none): VerbPhrase = Done
    # NEW. "is countered"; over an active phrase, the progressive "is blocking"
    primitive be(phrase: VerbPhrase): VerbPhrase = Be {
      spelling = "be $phrase"
    }

Inside `done`, the complement the passive promotes holds `subject`. The printed agent is `done`'s
`by`, and nothing else carries one.

| Card | Oracle | Term |
|---|---|---|
| *Abrupt Decay* | "This spell can't be countered." | `static(clause(this(spell), cant(be(done(counter(subject))))))` |
| *Kaya's Ghostform* | "When enchanted permanent dies or is put into exile, …" | `does(enchanted(permanent), or([die, be(done(putInto(subject, exileZone)))]))` |
| *Tide of War* | "each blocking creature is sacrificed by its controller" | `clause(…, be(done(sacrifice(subject), by = controllerOf(it))))` |

Three cases need the two words apart, and need the agent on `done`:

- **One "be" over two participles.** *Cephalid Illusionist* prints "Prevent all combat damage that
  would be dealt to and dealt by target creature you control this turn." There is one "be", and
  only the second participle has an agent. The relative's verb phrase is (`that` SKETCH; `shared`
  and `_` are §7's; "this turn" is the prevention's duration, a parameter, §8):

      that(would(be(shared(
        and([done(deal(subject, to = _)), done(deal(subject), by = _)]),
        target(…)))))

- **A participle with no "be".** *Abu Ja'far* prints "destroy all creatures blocking or blocked by
  it": an active and a passive participle coordinated as one postmodifier, the active one printed
  in its gerund-participle form.

      destroy(all(nominal(creature, post = [
        shared(or([block(attacker = _), done(block(attacker = subject), by = _)]), it)])))

  In the passive arm `subject` is the creature described and `_` the shared "it": the promoted
  complement and the shared phrase are two different things in one arm, so they are two words.
- **A verb with two passives.** `deal` prints both: *Phytohydra*'s "If damage would be dealt to
  this creature" promotes the damage, and *Grollub*'s "Whenever this creature is dealt damage" the
  recipient. The grammar reads the first as the passive of the frame with a *to* phrase and the
  second as the passive of the ditransitive frame (`TransitiveWithPP(NP,PP[prep:to])/passive`,
  `Ditransitive(NP,NP)/passive`). Each frame is a declaration, and the slot that holds `subject`
  says which is printed:

      # deal(damage, to) is §6's. NEW Template for the ditransitive frame.
      template dealIndirect(recipient: NounPhrase, damage: NounPhrase): VerbPhrase {
        spelling = "deal $recipient $damage",
        body = deal($damage, to = $recipient)
      }

      would(be(done(deal(subject, to = this(creature)))))          # Phytohydra
      does(this(creature), be(done(dealIndirect(subject, bare(damage)))))   # Grollub

  A Template adds no constructor, so both are the one `deal` event, and expansion carries
  `subject` into `to`. No line in the corpus prints the ditransitive in the active ("deals it 3
  damage": 0 lines), so `dealIndirect` is written only under `done`.

`done`'s wording is not its phrase with "by" set after it. The English layer derives a frame's
passive, and the passive with an agent is a variant of that frame whose agent is one of its
complements (`spelling.md` §1). The cards print the agent in three places:

- **Last among the participle's complements.** *Sengir Vampire*'s "a creature dealt damage by
  this creature this turn" has its window after the agent: the Reading is a temporal adjunct
  over the passive with its agent, the order `did`'s wording gives.
- **After a dependent that stands between the complements and the agent.** *Suffocation*'s "you
  were dealt damage this turn by a red instant or sorcery spell" has the duration inside the
  passive: one frame variant holds "dealt damage", "this turn" and the agent, and no verb phrase
  in the Reading spans "dealt damage this turn" alone. 51 units on 50 cards have such a variant
  (`…/passive+by/interleaved…`).
- **Before another complement.** *Impact Resonance*'s "damage dealt by a source to a permanent or
  player" puts the agent ahead of the *to* phrase. One unit has that variant
  (`…/passive+by/internalised-before1`).

The second and third have no wording until §13's follow-up; *Impact Resonance* is a Template.

`subject` under `done` reads what §2 says the enclosing phrase is said of: the clause's subject
under `be`, the described thing in a postmodifier.

## 4. Who performs

The performer of a verb phrase is what its `actor` word reads, what a printed "by" names and what
a trigger on a deed matches. It follows from the verb phrase:

1. **A deed in the active is performed by what it is said of**: the clause's subject, or, for an
   imperative, the controller of the spell or ability [CR#608.2c]. *Tome Scour*'s "Target player
   mills five cards" mills from that player's library.
2. **A `done` phrase is performed by its `by`**, and by nobody the term names when there is none.
   What it is said of is the promoted participant.
3. **A Template or Definition whose body reads `subject` states something that happens to what it
   is said of, and names no performer.** These are the verbs English prints in the active though
   their subject performs nothing:
   - *dies*, defined as "is put into a graveyard from the battlefield" [CR#700.4]; *enters*,
     *leaves*, *phases out*;
   - *becomes tapped* and the other "becomes" events [CR#603.2e]; *becomes blocked*, which an
     effect can cause with no blocker [CR#509.1h,509.3c];
   - *untaps* in "doesn't untap during its controller's untap step": the active player untaps
     their permanents [CR#502.3]; *transforms* (*Cult of the Waxing Moon*, *Immerwolf*), where
     transforming is something done to a permanent [CR#701.27a]; *regenerates* (*Matopi Golem*);
   - *gets counters*: the controller of a source with infect gives a player the poison counters
     its damage causes [CR#120.3b], and *Solemnity*'s "Players can't get counters" stops them (its
     ruling: "Players don't get poison counters and they don't lose life"). So the player who
     gets a counter is not the one who puts it.
4. **A continuous-effect phrase has no performer.** A creature that gets +3/+3 performs nothing.

An unnamed performer is not a wildcard word in the term. In an event pattern (a trigger, a
`would`, a lookback, a prohibition) it matches whoever performs the event. In an instruction it
is left to lowering: *Hardened Scales*'s "that many plus one +1/+1 counters are put on it
instead" states the modified event that occurs in place of the replaced one [CR#614.6], and names
nobody.

What English verb a phrase prints does not decide this; its declaration does. "You get an emblem"
means that player puts the emblem into the command zone [CR#114.2], so `getEmblem` is a deed its
subject performs, while `getCounters` is not:

    # NEW. "get one or more counters"
    template getCounters(counters: NounPhrase): VerbPhrase {
      spelling = "get $counters",
      body = putCounters($counters, subject)
    }

**A verb with a causative and an inchoative use is one Definition and one Template.** *Mana Vault*
prints both: "This artifact doesn't untap during your untap step" and "untap this artifact". They
are one keyword action with one Deed (`surface.md` §6.1): the Definition `untap` stays as it is,
and the inchoative is a Template whose body calls it with the word `subject` as its argument. The
same holds for `transform`. The Template cannot share its Definition's name at `VerbPhrase`
(`syl.md` §3), so the two are `untapSelf` and `transformSelf`. `die`, `enter`, `leave`,
`becomeStatus`, `phaseIn` and `phaseOut` have no transitive twin and keep their names.

*Regenerate* is no such pair. The Definition `regenerate` only sets up the shield and names no
Deed, and `regeneration`, the shield's application, carries it: creating a shield is not
regenerating a permanent [CR#701.19a,701.19c]. So *Matopi Golem*'s "When it regenerates this way"
is `regeneration` said of its subject. How the two are written is `syl-explicit-deeds`'s question.

**"Becomes blocked" is one word with an optional agent**, `becomeBlocked(by = none)`, "become
blocked< by $by>". Its two forms are two events, and an absent `by` is not "by anything": a
pattern watches what it writes (`semantics.md` §3.2).

- Without `by`, the Blocked status begins: once each combat, and an effect can cause it with no
  blocker [CR#509.1h,509.3c].
- With `by`, a creature blocks it: once for each blocker, a creature put onto the battlefield
  blocking it among them [CR#509.3d].

It is not `done(block(…), by = …)`: a "blocks a creature" trigger does not trigger for a creature
put onto the battlefield blocking [CR#509.3b]. The grammar agrees, reading "becomes blocked by a
creature" as "becomes" over an adjective phrase, not as a passive.

**A Definition's body is clauses, and reads its call's performer through `$actor`.** An
`action` Definition stands at `VerbPhrase`, and its body says who does what: "that permanent's
controller reveals the top card of their library. … Otherwise, that player puts a +1/+1 counter
on the exploring permanent and may put the revealed card into their graveyard" [CR#701.44a]. The
body is a `Clause`, or a word that holds clauses. A Definition's body is not substituted where
its call stands, as a Template's is; expansion wraps it in the Definition's scope
(`syl.md` §4), and that scope is the verb phrase. `$actor` is an implicit parameter of every
`action` Definition, bound once at the call to the call's performer (rules 1 and 2 above): what
an active call is said of, a `done` call's `by`, and nobody the term names where `by` is absent.
In *Tide of War*'s `done(sacrifice(subject), by = controllerOf(it))` it is the controller, not
the creature sacrificed.

    action explore {
      body = sequentially([
        clause(controllerOf($actor), reveal(…)),
        doIf(…,
          clause(that(player), putInto(…)),
          otherwise = clause(that(player), and([putCounters(…, $actor), may(putInto(…))])))])
    }

`mill`'s body is `clause($actor, move(…libraryOf($actor)…))`. A body needs no word that hands
a deed to another performer, so `instruct` goes: eight `action` Definitions hold it today
(`fight`, `fightEachOther`, `connive`, `explore`, `endure`, `airbend`, `regeneration`,
`theRingTemptsYou`). `explore`, `connive` and `endure` write the bare word `actor` for the
permanent that gets the counters, inside a Handoff to its controller, where the word reads the
controller; `$actor` does not rebind.

## 5. Deontic words are verb-phrase operators

`cant`, `can`, `must`, `gatedBy` and `doesnt` take a verb phrase and give one. Their modifiers
stay their own parameters:

    cant(phrase: VerbPhrase, timing = none, asThough = none, except = none,
         causedBy = none, unless = none): VerbPhrase

So one subject over a continuous-effect phrase and a prohibition is an ordinary coordination, with
no binder and no placeholder:

| Card | Oracle | Term |
|---|---|---|
| *Aether Tunnel* | "Enchanted creature gets +1/+0 and can't be blocked." | `static(clause(enchanted(creature), and([getsPt(up(1), up(0)), cant(be(done(block(attacker = subject))))])))` |
| *Solemnity* | "Counters can't be put on artifacts, creatures, enchantments, or lands." | `static(clause(bare(counter), cant(be(done(putCounters(subject, bare(or([…]))))))))` |
| *Solemnity* | "Players can't get counters." | `static(clause(bare(player), cant(getCounters(bare(counter)))))` |

The grammar reads *Aether Tunnel*'s line as one subject over a coordination of two verb phrases
(`SubjectPredicate` over `Coordination:VerbPhrase`). 18 lines print "gets … and can't be blocked",
and 15 more coordinate a power/toughness change with another verb under one subject ("gets +2/+2
and attacks each combat if able").

A prohibition over a coordination forbids each arm (`surface.md` §7.6), and it passes through
`be`: *Lava Burst*'s "that damage can't be prevented or dealt instead to another permanent or
player" is one "be" over an `or` of two `done` phrases.

**Where the box opens.** `semantics.md` §2.2 item 3 puts a deontic rule's Subject inside the
rule's box. With the subject outside the operator:

- A clause's subject is read once, before its verb phrase and outside every operator in it.
- Each deontic operator opens its box over its own verb phrase and its Unless Condition. The
  hypothetical participants of the phrase (the "legendary instant or sorcery spell" of
  [CR#205.4e]) are in that box and close with it.
- The Unless Condition is inside the clause, so it reaches the subject. *Pithing Needle*'s
  "Activated abilities of sources with the chosen name can't be activated unless they're mana
  abilities" reads "they" as the subject. Over a subject that ranges, it reads one member at a
  time (`semantics.md` §2.4 item 2): "that player" in [CR#205.4e].
- **An indefinite subject takes its force from what its clause states, not from the modal.**
  Where the clause states what holds (a static ability's clause, a rule's, or a resolving
  effect's prohibition or continuous-effect phrase), it is generic: the clause holds of each
  thing the description fits. "A player can't cast a legendary instant or sorcery spell unless …"
  forbids each player [CR#205.4e]; *Damping Engine*'s "A player who controls more permanents than
  each other player can't play lands …" is the same; *Incinerate*'s "A creature dealt damage this
  way can't be regenerated this turn" is generic inside a resolving spell; and *Yurlok of Scorch
  Thrash*'s "A player losing unspent mana causes that player to lose that much life" is generic
  with no modal at all. Where the clause states a deed done as a spell or ability resolves, an
  indefinite subject is a choice made then (`semantics.md` §2.2 item 1): *Wake to Slaughter*'s
  "An opponent chooses one of them."

- **A subject that counts or restricts is read inside the operator's box**, as `semantics.md`
  §2.2 item 3 closes counted participants there: *Silent Arbiter*'s "No more than one creature
  can attack each combat", "Only creatures in the chosen piles can block this turn".

So a subject shared by a modal and a non-modal arm needs no special reading: it is read outside
both. No discourse reading exists to run any of this.

## 6. Object phrases

What a verb deals, puts, removes, creates, prevents, gains or loses is a noun phrase: the
ordinary determiners over a `Nominal` (`syl-nominal-descriptions`) whose head is `damage`,
`life`, a counter kind, `counter` or `token`. The cards print these phrases with everything a
noun phrase has:

| Card | Oracle |
|---|---|
| *Phytohydra*, *Stormwild Capridor* | "If damage would be dealt …", "If noncombat damage would be dealt to this creature, prevent that damage" |
| *Hardened Scales* | "that many plus one +1/+1 counters are put on it instead" |
| *Winding Constrictor* | "that many plus one of each of those kinds of counters are put on that permanent instead" |
| *Primal Vigor*, *Divine Visitation* | "twice that many of those tokens are created instead"; "that many 4/4 white Angel creature tokens with flying and vigilance are created instead" |
| *Cephalid Illusionist*, *Glacial Chasm* | "all combat damage that would be dealt to and dealt by …"; "Prevent all damage that would be dealt to you." |
| *Impact Resonance* | "X damage divided as you choose among any number of target creatures"; "the greatest amount of damage dealt by a source to a permanent or player this turn" |
| *Grollub* | "each opponent gains that much life" |

They stand as a clause's subject, take a relative clause, a participle, a premodifier and a
demonstrative, and are read back by anaphors. A scalar amount beside a fixed noun in the verb's
wording cannot do any of that. "Damage that would be dealt" alone is on 333 lines, and a passive
clause as a replacement's "instead" on about 80.

**Their sort is `NounPhrase`.** A clause's subject is a `NounPhrase`, and so is the phrase
`shared` takes (§7), so a sort of their own would need a coercion into `NounPhrase`. Every
determiner word (`a`, `all`, `counted`, `that`, `those`) would then stand at two sorts of
Reaches(`NounPhrase`), which `syl.md` §3 refuses. There is no coercion from `Nominal` to
`NounPhrase`: "damage" with no determiner is `bare(damage)`.

The declarations whose object is spread over scalar parameters and a fixed word take it as one
phrase:

| Today | Calls (canon, builtin) | Takes |
|---|---|---|
| `dealDamage(amount, recipient, kind)` | 186, 27 | `deal(damage: NounPhrase, to: NounPhrase = any)` (NEW name) |
| `putCounters(amount, kind, on)` | 93, 46 | `putCounters(counters: NounPhrase, on)` |
| `removeCounters(quantity, kind, from)` | 16, 9 | `removeCounters(counters: NounPhrase, from)` |
| `createObject(count, spec, under)` | 7, 4 | a tokens phrase, as `syl-token-records-to-templates` writes a token's description |
| `preventDamage(cut, damage, also)` | 30, 6 | `prevent(damage: NounPhrase)` (NEW name); `also` below |
| `gainLife(amount)`, `loseLife(amount)` | 54, 4; 25, 9 | `gainLife(life: NounPhrase)`, `loseLife(life: NounPhrase)` |

The damage's kind is a premodifier ("combat damage", "noncombat damage"). A counter kind stands
at `Nominal` as its compound noun ("+1/+1 counter"), by a proposed coercion from the counter-kind
sort; the bare noun `counter` is the head when no kind is printed (*Solemnity*).

**What such a phrase means is read by the verb that takes it.** Damage is no object
[CR#109.1], and counters "are not objects and have no characteristics" [CR#122.1], so these heads
do not select things the way "creature" does:

- The determiner's count is the amount ("3 damage", "that many", "one or more"), a premodifier is
  the kind, and a demonstrative re-mentions the earlier phrase ("that damage", "those counters",
  "those tokens").
- A damage nominal with a `would` relative describes damage that would be dealt; the relative's
  verb phrase is the event pattern. `prevent` over such a phrase is a prevention effect
  [CR#615.1a]: it covers all such damage under "all", and is a shield of N under "the next N"
  [CR#615.7].
- A verb that cannot read the phrase it is given refuses it. Nothing in the sort system does;
  semantics refuses it (`docs/design/semantics.md` §4), and until semantics exists nothing does.

**The numeral's notation follows the head.** Oracle prints "3 damage" and "3 life" but "three
+1/+1 counters" and "two cards". A Hole's notation is stated by its own wording
(`spelling.md` §7), and with the number inside a determiner the wording that prints it no longer
names the noun. So the measuring determiner of a mass head is its own NEW Template, printing
`$n@Arabic`, beside `counted`, which prints `$n@Counting`: `measured`.

**An amount printed after its head is a postmodifier of the nominal**: "damage equal to its
power". 406 lines print "damage equal to …", 280 "life equal to …" and 75 "a number of … counters
… equal to …". A scalar parameter beside the phrase would be a second place for the amount. Where
the phrase prints away from its noun ("deals damage to any target equal to …") is the Tail
question `spelling.md` §3.2 has.

**Not placed by the rules above:**

- ***Sulfuric Vapors*'s "that much damage plus 1"**, with the amount on both sides of the head.
- **`preventDamage`'s `also`**, the additional effect a prevention effect may include
  [CR#615.5].
- **`redirectDamage`** (12 canon calls), which §8's second form of a replacement bears on.
- **Whether an object phrase introduces a Referent** that "that damage" and "that many" read.

The same treatment applies where a passive or a relative needs another verb's object as a
phrase: *Explosion of Riches*'s "Whenever a card is drawn this way" (`draw` takes an `Amount`
today) and *Animal Attendant*'s "If that mana is spent to cast …". Those verbs are reshaped by
the tickets that write such cards, not here.

## 7. Shared phrases

A noun phrase that coordinated verb phrases share is written once, after them, and each arm
holds a placeholder where the phrase belongs:

    # NEW. "tap or untap target creature"
    primitive shared(arms: VerbPhrase, rhs: NounPhrase): VerbPhrase = Shared {
      spelling = "$arms $rhs"
    }
    # NEW. The place in an arm where the shared phrase belongs. Prints no word.
    primitive _: NounPhrase = SharedGap

*Bounding Krasis* prints "you may tap or untap target creature":
`clause(you, may(shared(or([tap(_), untap(_)]), target(creature))))`. The target is introduced
once, outside the `or`, and each `_` reads it. It is printed after the arms, so `_` reads
forward, which `semantics.md` §1 item 2 allows.

Both are ordinary primitives, each declared at one sort, so no word stands at two sorts and no
resolver-known form or sort variable is added. `cargo xtask dsl check` refuses a `shared` whose
`arms` is not a coordination, an arm that does not hold exactly one `_`, and a `_` anywhere else.
The first refusal matters: `shared(tap(_), target(creature))` would be a second term for "tap
target creature".

**Only verb phrases sharing a noun phrase need the mechanism.** The grammar declares 63
`Coordination:…/shared…` Productions and Vintage-legal text uses ten:

| Production | Units | Wordings before the shared phrase |
|---|---|---|
| `Coordination:VerbPhrase/shared` | 247 | 49: "tap or untap", "dealt to and dealt by", "cast or copy", … |
| `Coordination:Nominal/shared-measure` | 205 | 1: "base power and toughness 4/4" |
| `Coordination:AdjectivePhrase/shared` | 112 | 4: "less than or equal to" and three more |
| `Coordination:Nominal/shared-pp` | 49 | 6: "top or bottom of", "power and toughness of", … |
| `Coordination:Nominal/shared-measure-amount` | 13 | |
| `Coordination:VerbPhrase/shared-keyword` | 5 | 2: "have or gain", "can't have or gain" |
| `Coordination:Clause/shared` | 5 | 5 single units |
| `Coordination:NounPhrase/shared-pp`, `…/shared-delimited`, `VerbPhrase/shared-bareinf` | 4, 1, 1 | |

The wordings are each unit's words before the shared phrase, grouped by a script, so their counts
are approximate. In every unit of the first row the coordinated verb phrases carry a noun-phrase
Gap. Every other row is a short list of fixed wordings, and each is a plain Template with an
ordinary parameter: `lessThanOrEqualTo(x)`, `basePowerAndToughness(m)`, `topOrBottomOf(x)`,
`haveOrGain(ability)` (names SKETCH). A second sort that turns out to share an open class of
phrases gets its own declaration when a card needs it. The five clause-level units have not been
read to see whether the shared Reading is the one each card means.

*Dream Fighter*'s "blocks or becomes blocked by a creature" is
`shared(or([block(attacker = _), becomeBlocked(by = _)]), a(creature))`. 42 units on 41 cards
print the wording, and each has only a Reading with "blocks" intransitive until
`english-shared-complement-through-predicative` adds the shared one. The cards mean the shared
one: *Engulfing Slagwurm*'s ruling has its ability trigger once for each creature it blocks or
is blocked by. No Template is declared for the wording.

*Balthor the Defiled*'s "all black and all red creature cards", which
`english-shared-nominal-determiner-coordination` gives a Reading, shares a nominal between two
determiners. It is one card and is a Template or a second declaration, not this `shared`.

## 8. Replacements, lookbacks and the event gate

Each stays its own structure (`semantics.md` §1 item 5(a)).

**A replacement is still `replace(event, replacement)`.** `would` is a verb-phrase operator, like
the deontic words, and `replace`'s event is a `Clause` whose verb phrase is `would(…)`:

    # Phytohydra: "If damage would be dealt to this creature, put that many +1/+1 counters on
    # it instead."
    replace(
      clause(bare(damage), would(be(done(deal(subject, to = this(creature)))))),
      instead(imperative(putCounters(counted(thatMany, p1p1Counter), it))))

`would` is an operator because cards print it inside a relative on a damage phrase (333 lines)
and coordinate it under one subject with another verb phrase (*Illusionary Mask*'s "has not been
turned face up and would assign or deal damage, be dealt damage, or become tapped", one
`Coordination:VerbPhrase` in its Reading). `cargo xtask dsl check` refuses a `replace` whose
event's verb phrase is not a `would`, looking through the words that wrap an event
(`nthOccurrence`, `causedBy`). The sort `WouldClause` goes, since nothing builds one: the words
that took it take a `Clause`. `instead` takes a `Clause`, so a passive effect clause is written
as a clause: *Hardened Scales*'s replacement is
`instead(clause(…, be(done(putCounters(subject, it)))))`, though where "instead" prints inside
that clause is not designed (§11). A conditional and a replacement stay different constructors
with different sorts: only `replace` takes a `Replacement`.

**A clause whose subject carries a `would` relative is a replacement too**, its verb phrase
stating what happens in the event's place. *Carom* prints "The next 1 damage that would be dealt
to target creature this turn is dealt to another target creature instead" [CR#614.9]. The shape
is wider than redirection: "damage that would be dealt … is dealt to …" is on 33 lines; *Worship*
prints "damage that would reduce your life total to less than 1 reduces it to 1 instead"; and
*Trinisphere*'s "each spell that would cost less than three mana to cast costs three mana to
cast" has no "instead". This is a second printed form of a replacement, read off the subject as
`prevent` reads its object (§6).

**A lookback is the operator `did`, and keeps its window.**

    # "attacked this turn"
    did(phrase: VerbPhrase, within: Option(LookbackSpan) = none): VerbPhrase

The window is optional, since some lookbacks print none (*Deadly Cover-Up*'s "If evidence was
collected"), and `did` keeps the `ordinal` a description takes today ("the second spell you cast
each turn").

- As a condition it stands under `holds`: "if you attacked this turn" is
  `holds(you, did(attack, within = thisTurn))`.
- As a description it is a verb phrase said of the described thing (§2), in the nominal's
  postmodifier list: "creatures that attacked this turn" is `did(attack, within = thisTurn)`, and
  *Phyrexian Rebirth*'s "creatures destroyed this way" is `did` over a `done` phrase, whose
  participle takes no form label (§1). The printed "that" is a Template's.
- A relativized object keeps `which`: *Alabaster Leech*'s "White spells you cast" is a clause
  with `which` in the object's place.

One past word serves the condition and the description, so `was` goes.

**A window or a duration is a parameter of the word it limits**, where it is today: `did`'s
`within`, a deontic operator's `timing`, an effect's `Duration`. This spec adds no adverbial word
that wraps a verb phrase. Where such a phrase prints among the verb's other dependents is §13's
follow-up.

**The event gate reads through the new words.** A phrase under `does`, `did` or `would` must
name an event: an event-capable head or a keyword action's Definition, a `done` over one, `be`
over that, a Template of §4 whose body is one, a `shared` over these, or an `and` or `or` of
these. `syl-shared-phrase-written-once` already brings `semantics.md` §3.2 and `surface.md` §7.5 to
admit an `or` of event-capable heads under one subject.

**An unprinted pattern writes `any` as its subject.** Rule bodies that no card prints, such as
`infect`'s `would(any, loseLife(anyAmount))`, become `clause(any, would(loseLife(…)))`, with
`any` the wildcard word at `NounPhrase` (`surface.md` §7.3).

## 9. What is refused, and where

`cargo xtask dsl check` refuses, before any spelling code exists:

- `subject` in a card's term outside a `done` phrase, and a `done` phrase that does not hold
  exactly one `subject`. A Template's body may place `subject` anywhere a noun phrase stands
  (§4).
- `$actor` anywhere but an `action` Definition's body;
- the three `shared` and `_` cases of §7;
- a `replace` whose event is not a `would` clause, and the event gate (§8).

The spelling boundary refuses the rest, as it refuses any term with no English
(`architecture.md`, design principle 4). There is no table of verb forms. `clause(x,
done(counter(subject)))`, a finite clause over a bare participle, has no Reading; neither has
`cant(done(…))` without `be`, nor `done(block(attacker = it, …))` with `subject` in a slot the
verb's passive does not promote. A wording is checked with its silent words silent: `done`'s
phrase with `subject` printing nothing must parse as a passive of one of the verb's frames, and
`shared`'s arms as verb phrases with a Gap.

Some wordings leave more than one well-sorted term, and no rule here picks: "is tapped" has a
passive term and a copular one, since the grammar gives both Readings, and a trailing modifier
over a coordination has a wide and a narrow term (*Broken Wings*'s "target artifact, enchantment,
or creature with flying"; *Troublemaker Ouphe*'s "target artifact or enchantment an opponent
controls"). The contexts separate many of them: `does`, `did` and `would` admit only events, and
a condition wants a state. Choosing among the rest is a pragmatics step of its own
(`docs/design/pragmatics.md`), outside this spec.

## 10. What this replaces

In the design docs, a follow-up edit for the tickets that build this:

| Where | Sentence | Becomes |
|---|---|---|
| `semantics.md` §2.1 | "A Subject is a referent; an Actor is not." through "…in a Subject, an agent or a Handoff, is an ordinary referent." | §2 and §4 here |
| `semantics.md` §2.2 item 3 | "Under `cant`, a hypothetical indefinite has universal force" | of a subject, §5's rule; of the phrase's own participants, unchanged |
| `surface.md` §7.1 | the table of eight contexts | §1's table |
| `surface.md` §7.2 | the whole section ("Voice belongs to the Subject, not to the Action"; "'Becomes blocked by <B>' is `by(B)` over `block`") | §3 and §4 |
| `surface.md` §7.4 | "A transitive Action always keeps its patient inside it. The Subject supplies only the performer." | the slot stays in the verb phrase and holds `subject` under `done` |
| `surface.md` §7.5 | "Each arm is a whole event with its own subject … An `or` over Actions would be a compound action, which the gate refuses." | §8's gate |
| `surface.md` §7.6 | "A deontic rule is a Subject, an Action and optional modifiers"; "`doesnt(subj, action)`" | §5 |
| `syl.md` §2 | "There are three" coercions | the coercions that exist, §1's and §6's among them |
| `syl.md` §4 | a Definition's body and its parameters | an `action` Definition's body is clauses and reads `$actor` (§4) |
| `semantics.md` §2.3 | "the `actor` word still reads the innermost Actor" | a Definition's body reads `$actor` |
| `semantics.md` §3.4 | "A lookback is `did(subj, action, within)`" | the operator `did(phrase, within)` under `holds` or in a postmodifier (§8) |
| `syl.md` §4 | "a passive `by(np)` reads the patient before `np`" | removed |
| `spelling.md` §1 | "its Subject selects the voice" | `done` selects the passive |
| `spelling.md` §2, §4, §6, §7, §9, §10 | passive subject promotion, its notation and "Which passive a term prints" | removed; §3 |
| `spelling.md` §3.3, §10 | Factoring and its placeholder | §7; *Aether Tunnel* is §5's coordination |
| `architecture.md` | "a passive prints the complement its Action's frame promotes as its context's subject"; "Factoring of what a coordination's arms share" | §3, §7 |

Glossary entries (`docs/contexts/game-model/CONTEXT.md`):

- **Subject**: rewritten. The noun phrase a clause prints as its subject, and what a verb phrase
  is said of (§2). The word `subject` reads it as a participant.
- **Actor**: rewritten to §4's rule; "unless a Handoff names another" goes.
- **Patient**: rewritten: it stays inside the verb phrase, and holds `subject` when a passive
  promotes it.
- **Handoff**: retired. A clause whose subject performs its verb phrase needs no name of its
  own, and a printed "have" is `have`.
- **Action**: kept for the verb phrase one verb's declaration builds, which is what the event
  gate admits. The sort is `VerbPhrase`.
- **Coercion**: its list gains the coercions of §1 and §6.
- New: **Clause**, **Verb Phrase**, **Shared Phrase** (§7), and a name for the phrases of §6
  (§13).
- In `docs/contexts/oracle-english/CONTEXT.md`, **Factoring** is retired.

Tickets this meets, each to be revised against it before it is claimed:

- `syl-continuous-effect-subject`: its subject-giving declaration is `clause`, its subjectless
  phrase a `VerbPhrase`, the word that reads the subject `subject`. Its closing remark that
  *Aether Tunnel* still needs Factoring no longer holds, and the "enters" family gives up its
  subject with the rest (§1).
- `syl-misnamed-declarations`: the `actor(np)` to `subject(np)` rename is not made; the wrapper
  is deleted at the same sites (about 502 canon and 170 builtin calls). Its rename of the
  `NounPhrase` word `actor` to `subject` applies only to the bodies that read a participant (§2).
- Whichever ticket rewrites the `any` and `by` sites reads each promoted slot from the card's
  Oracle line, for `deal` line by line, and corrects in the same pass the sites whose voice is
  wrong today: *Captain Marvel, Apex Avenger*, *Divine Intervention* and *Moonlit Meditation*
  write `by(you)` for a printed active ("When you remove the last intervention counter …").
- The eight Definitions that hold a Handoff are rewritten as clauses over `$actor` (§4), which
  corrects `explore`, `connive` and `endure`: the controller "puts a +1/+1 counter on the
  exploring permanent" [CR#701.44a], and today all three put their counters on the controller.
- `syl-explicit-deeds`: its open question on `regenerate` and `regeneration` decides how
  "regenerates" is written (§4).
- `syl-shared-phrase-written-once`: its placeholder is `_`, and the only coordination that takes
  a shared phrase is §7's. Its different-roles cases for a shared subject are ordinary
  coordinations, "blocks or becomes blocked by" goes through `shared`, and `becomeBlocked` takes
  an optional `by` (§4).
- `english-shared-complement-through-predicative` and `english-have-past-participial-complement`:
  the Readings §7 and §11 wait on.
- `syl-printed-read-order`: the `preventDamage` and `removeCounters` reorders are absorbed by
  §6.
- `syl-token-records-to-templates` and `syl-nominal-descriptions`: a tokens phrase and the heads
  of §6 are nominals; a verb phrase stands in a nominal's `post` as a participle.

## 11. Not covered

- **Negation over `be` and the perfect.** About 100 lines print "isn't", "aren't" or "wasn't"
  before a participle (37 of them "attacks and isn't blocked" [CR#509.3g]), and 48 print a form
  of "have" before "been", 15 of them "has been" or "have been" (*Bloodcrazed Goblin*'s "has been
  dealt damage this turn"). Both are verb-phrase operators over `be`, not designed here. `doesnt`
  is do-support only.
- **A cost that prints its own subject.** *Wall of Shards*'s "Cumulative upkeep—An opponent
  gains 1 life." and *Roar of Jukai*'s "Splice onto Arcane—An opponent gains 5 life." are
  clauses where §1 makes a cost a verb phrase. `syl-clause-and-imperative` decides how they are
  written.
- **Contractions** ("it's", "that's") and where "instead" stands in a passive clause: each is a
  Template per placement (`spelling.md` §2).
- **"Have" with a participle** (*Melira's Keepers*'s "can't have counters put on it"), which
  waits on `english-have-past-participial-complement`.
- **"The target of"** (*Gaea's Revenge*'s "can't be the target of nongreen spells …"): a copula
  over a noun, not a passive; a verb-phrase Template over the targeting event.
- **The tallies and records** that hold a subject and an action (`DidClause`, `tally`,
  `eventCount`, `eventSum`, `eventTally`): their subject becomes a noun phrase with the rest; the
  wording of the tallies is not designed here.
- **The copula.** `is(subject, predicate)` keeps its subject here; `syl-nominal-descriptions`
  owns its complement.
- ***Illusionary Mask*'s third sentence**, which coordinates a perfect with a `would` in a
  replacement's event.
- **An object relative under `would`**: *Harm's Way*'s "The next 2 damage that a source of your
  choice would deal to you …".
- **What a deontic operator admits beyond events**, since "can't have or gain flying" forbids a
  state [CR#113.11]; what `and` means in an event pattern, a union in "dealt to and dealt by" and
  a conjunction in "attacks and isn't blocked" [CR#509.3g]; and whether `doesnt` is gated.
- **A participle that describes a state.** *Abu Ja'far*'s "creatures blocking or blocked by it"
  names creatures in a combat state [CR#509.1g], and §3 writes it with `block`, an event (§4).
- **A later sentence that reads a generic subject** (*Damping Engine*'s "That player may
  sacrifice a permanent …"): the discourse design's.
- **The relative-clause words**, a `VerbPhrase` standing as a postmodifier, and the duration
  words: `syl-nominal-descriptions` and `syl-continuous-effect-subject`.

## 12. New words

- Sorts: `Clause`; `VerbPhrase`, the new name of `Action`.
- Primitives: `clause`, `imperative`, `holds`, `have`, `done`, `be`, `subject`, `shared`, `_`.
  A `holds` exists at `AsThough` already ("as though $cond"); no coercion joins that sort to
  `Condition`, so the two do not collide (`syl.md` §3).
- The implicit parameter `$actor` of an `action` Definition.
- Coercions: `Clause` to `StaticSpec`; a counter kind to `Nominal`.
- Templates: `dealIndirect`, `getCounters`, `untapSelf`, `transformSelf`, `measured`, the idiom
  Templates of §7.
- Retyped: `does`, `did`, `would`, `cant`, `can`, `must`, `gatedBy`, `doesnt`, `asThough`,
  `replace`, `instead`, `may`, `withContinuation`, `becomeBlocked`, the words that hold effects
  (§1), the 60 subject-first declarations at `StaticSpec`, and the object-phrase verbs of §6.
- Renamed: the self-replacement `replace`.
- Removed: the sorts `Subject` and `WouldClause`, `actor(np)`, `any` and `by(np)` at `Subject`,
  `instruct`, `was`.

## 13. Later steps and what is still open

1. **Adverbials: the dependents after a verb, in printed order.** The order of what follows a
   verb is wording, and a nested term states scope, not order:
   - an agent after a dependent (*Suffocation*'s "dealt damage this turn by …"; 51 units) and
     before one (*Sengir Vampire*'s "dealt damage by this creature this turn");
   - an agent before a complement (*Impact Resonance*; one unit);
   - a trailing phrase after a modal, which the grammar attaches at several sites (*Incinerate*'s
     "can't be regenerated this turn": to "can't", to "be" or to "regenerated");
   - "for each" before or after a duration (*Aurochs*, *Ral's Staticaster*), which `spelling.md`
     §3.2's `inner` mark exists for.

   The planned answer is the one descriptions got: the verb phrase's unselected dependents (a
   duration, a window, "this way", "for each", the agent) in one list in printed order, with the
   verb's selected complements staying its own parameters. It is not designed here. It does not
   say which of several stacked operators a phrase belongs to, and *Impact Resonance* stays a
   Template under it. It follows the first spelling code, which can show which orders occur.
   Until then a window or duration is a parameter (§8), and the lines whose agent does not
   print last have no wording.
2. **A coercion from `VerbPhrase` to `Clause`**, so that `imperative(…)` need not be written. It
   needs `syl.md` §3 to let a word stand at a sort and at the from-sort of a coercion into it,
   the direct declaration found first. For a coordinator the form that hides, an imperative over
   coordinated verb phrases, has no Reading; for any other word declared at both sorts the
   hiding is silent. A written `imperative(…)` stays valid if it is adopted.
3. **`StaticSpec` as clauses.** The 10 wrappers and several of the 12 others are sentences in
   English ("Skip your draw step", "You don't lose unspent mana …"). `static` taking a `Clause`
   is the later step.
4. **A name for the phrases of §6.** "Object Phrase" names phrases for damage and counters, which
   are not objects [CR#109.1], beside the glossary's **Object**; "Measure Phrase" stands too
   close to the glossary's **Measure Noun Phrase**, the slash measure of "+2/+2".
5. **This layer is surface, not semantics.** The terms here follow the printed English: a
   clause, its verb phrase, its dependents. What this spec had to work around are the sorts that
   say what a clause means: a coercion to get a clause into `StaticSpec`, `does` and `holds` to
   make a `GameEvent` and a `Condition` of one, `replace` at two sorts. The direction is that the
   `.syl` terms and the Rust structures built from them are a surface layer, named for it
   (`sylex_surface`), and `sylex_semantics` reads those structures, decides what each means
   (which clause is an event, a condition, a static statement, a replacement) and is the home of
   reference resolution. A clause then stands bare after "when" and "if" and in a static
   ability, and the event gate is a check semantics makes, not a sort. The direction is written in
   `architecture.md` ("The layers"), `semantics.md` §4 to §6 and `surface.md`. Until
   `sylex_semantics` exists, this spec's `does`, `holds`, coercion into `StaticSpec` and event
   gate in `cargo xtask dsl check` are built as written: they are the scaffolding that keeps the
   corpus checked, and they come out when semantics makes the same refusals. Item 3 is part of
   it.
6. **"If you do" as a sentence of its own.** §1 holds an optional clause and its follow-up
   sentences in one word, `withContinuation`. On a surface layer the follow-up is an ordinary
   conditional sentence, linked to the choice by reference resolution: *Apathy*'s "If the player
   does, untap that creature." as a conditional over "the player does" (SKETCH). Reopened with
   item 5.
7. **How sentences join.** This spec puts the words that hold effects at `Clause` and declares
   the coordinators and sequencing words at both sorts, and designs none of them: a sequence of
   sentences against a coordination of clauses ("Draw two cards, then discard two cards";
   "… . Then …"), a dependent in a run's first sentence that governs the run (`spelling.md` §2),
   "Otherwise", and a trailing sentence over what precedes it ("Activate only as a sorcery.").
   `syl-sequentially-wordings` owns part of it.

