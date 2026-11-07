---
needs: [lean-v3-monstrosity-adapt-name-the-source, lean-retirement]
---

# Events for the verbs keyword abilities define

**Why.** Several keyword abilities define a verb for one of their own steps, used only to say what
happened: a creature "evolves" [CR#702.100b], "trains" [CR#702.149c], "exploits a creature"
[CR#702.110b], "mentors" another [CR#702.134c], "devoured" [CR#702.82b], is "championed"
[CR#702.72c], "enlists" another [CR#702.154c]. Only devour's is modeled, as a separate
`action devour { ofAbility = devour, … }` in `keyword_action/devour.syl`; the others have a lexical
verb and no Deed, so "whenever this creature evolves" cannot be written. *Biomancer's Familiar*
also uses "<creature> adapts", which the CR never defines: [CR#701.46a] defines only the
instruction "adapt N".

**Done when.**
- Each keyword ability's own declaration in `definitions/keyword_ability/` states the step its verb
  names, and nothing else in `.syl` uses `ofAbility`; `keyword_action/devour.syl` is gone and its
  readers ("creatures it devoured", "each creature sacrificed this way") still resolve.
- Each event happens when its rule says: once counters are actually put (evolve, training), when
  the ability resolves (mentor), when the sacrifice or exile happens (exploit, devour, champion),
  when the cost is paid by tapping the other creature (enlist).
- "<creature> adapts" is a standalone event, defined the way [CR#702.100b] defines "evolves": an
  adapt whose Source is that creature resolves. Its doc comment says it is a project-supplied rule
  filling a CR gap, not an inflection of the `adapt` keyword action.
- Each event says which argument is the Source and which the patient, since the subject is not
  always the Source: "Whenever equipped creature mentors a creature" (Source, patient), "When a
  Faerie is championed with this creature" (patient, passive).
- The subject's kind belongs to the event: "players can't adapt" is refused, because adapt's event
  subject is a creature, and a test pins the refusal.
- None of these events is usable as an instruction.
- These Vintage-legal cards are in `plugins/canon/cards/` and pass the `.syl` checks: *Watchful
  Radstag* and *Renegade Krasis* (evolves), *Savior of Ollenbock* (trains), *Aegis of the Legion*
  (mentors), *Mistbind Clique* (championed), *Guardian of New Benalia* (enlists), and a card with
  "When this creature exploits a creature".

**Not in scope.** *Biomancer's Familiar*'s "as though it had no +1/+1 counters"
(`lean-v3-single-use-as-though`).

**Pointers.** `plugins/builtin/definitions/keyword_ability/{evolve,training,exploit,mentor,devour,champion,enlist}.syl`,
`plugins/builtin/definitions/keyword_action/{devour,adapt}.syl`, `docs/decisions/semantics.md`
§3 and §3.1 (Deeds of keyword actions and abilities).
