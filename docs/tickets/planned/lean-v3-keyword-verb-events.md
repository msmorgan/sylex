---
needs: [lean-v3-monstrosity-adapt-name-the-source, lean-retirement]
---
# Events for the verbs keyword abilities define

Several keyword abilities define a verb for their own step, used only to describe what happened:

- "evolves": counters are put on it as a result of its evolve ability resolving [CR#702.100b]
- "trains": a resolving training ability puts counters on it [CR#702.149c]
- "exploits a creature": the controller of the exploit ability sacrifices a creature as that
  ability resolves [CR#702.110b]
- "mentors": a mentor ability whose source is the first creature and whose target is the second
  resolves [CR#702.134c]
- "devoured": sacrificed as a result of its devour ability as it entered [CR#702.82b]
- "championed": exiled as the direct result of a champion ability [CR#702.72c]
- "enlists": you pay its enlist cost by tapping the other creature [CR#702.154c]

Add "adapts" to them. [CR#701.46] defines no "<creature> adapts", yet *Biomancer's Familiar* uses
it; define it the way [CR#702.100b] defines "evolves": an adapt whose Source is that creature
resolves. It is a project-supplied rule filling the CR's gap, not an inflection of the adapt
keyword action.

None of these verbs is ever an instruction, and none has an Actor: the controller does the steps.
Today only devour's is modelled, as a separate `action … { ofAbility }` in
`keyword_action/devour.syl`, and evolve's lexical verb has no Deed, so "whenever this creature
evolves" cannot be written.

## Change

- The ability's own declaration in `keyword_ability/` states the step its verb names; retire the
  `ofAbility` pattern and `keyword_action/devour.syl`. Adapt's event is a standalone definition.
- Each event states when it happens, as its rule does: once counters are actually put (evolve,
  training), when the ability resolves (mentor), when the sacrifice or exile happens (exploit,
  devour, champion), when the cost is paid (enlist), when the adapt resolves (adapt).
- Each event states which argument is the Source and which the patient. The subject is not always
  the Source: "Whenever equipped creature mentors a creature" (Source, patient), "When a Faerie is
  championed with this creature" (patient, passive).
- The subject kind belongs to the event: "players can't adapt" is refused because adapt's event
  subject is a creature. This takes over the adapt half of `badPlayersCantExploreOrAdapt` (a Lean pin, carried to a
  Rust test by `semantics-rust-rewrite`).

## Cards

*Watchful Radstag* and *Renegade Krasis* ("whenever this creature evolves"), *Savior of
Ollenbock*, the exploit triggers ("When this creature exploits a creature"), *Aegis of the Legion*,
*Mistbind Clique*, *Guardian of New Benalia*, and *Biomancer's Familiar*, which also needs
`lean-v3-single-use-as-though`.
