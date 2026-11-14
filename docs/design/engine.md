# Engine

The contract of `sylex_engine`, the engine layer of
[Architecture](architecture.md#the-layers)'s layers, which holds game state and
executes [core](core.md).

## Laws

1. **The environment is an activation record**: one register vector per region
   entry, addressed by id.
2. **The engine enumerates; consumers decide**: every legal Decision is exposed,
   and policy belongs to consumers.
3. **An instruction reads only its operands.**
4. **Conferrals are positive and current**: default-deny grants derived from
   current layered characteristics.
5. **State-based actions that need a choice are engine code today**, and the
   check repeats only after a batch changed state.
6. **Broken state fails loudly; what the rules make impossible is ignored.**
7. **Members of one simultaneous batch are unordered.**

## Detail

### 1. The environment is an activation record

Each region entry gets one activation: a register vector indexed by definition,
held in a table on the game state. A work item carries the activation's id,
never a copy of its values, and an Execution Frame is that id plus payment
state. A loop element or a predicate's candidate is a parameter of its own
region, not an entry in a second, frame-local environment with its own read
rule.

### 2. The engine enumerates; consumers decide

At every Decision Point the engine exposes every legal Decision, including a
choice with one option, and it accepts concession at every one [CR#104.3a].
Automation, auto-payment, retry policy, redaction and convenience filtering
belong to the consumers: runners, strategies, the terminal client and the
matchup suite. Why: a choice that looks forced can carry rules-relevant
identity, and a complete legal set serves tests, strategies and clients alike.
A helper that applies a policy, such as auto-tapping for mana, may live in the
engine crate, but legality never calls it.

### 3. An instruction reads only its operands

An action reads its literal arguments and the registers it names. It never
infers meaning from its parent, its siblings, the number of targets, or how the
card renders. Why: independent instructions compose across ability shapes and
target counts. A branch that reaches outside an instruction to repair it
signals a missing structural form or binding, and the fix belongs there.

### 4. Conferrals are positive and current

A Card Type or Subtype confers what it enables, such as playing a land, the
Combatant role, or being attached, as a positive, default-deny grant that the
engine's subsystems read. A grant comes from the object's current layered
characteristics [CR#613.1], so gaining or losing a type gains or loses what it
confers, and nothing is cached across the change. A prohibition subtracts from
a grant; it never stands in for one that is absent. Why: a grant says what a
characteristic enables without listing its complement as prohibitions, and
deriving it from current characteristics leaves nothing stale.

### 5. State-based actions that need a choice are engine code today

The rules rows are data ([Semantics](semantics.md) §1, item 4); this is
the engine's half. Data rows are evaluated synchronously, one object at a time,
into game events. The legend rule [CR#704.5j] groups objects across a
controller and pauses at a Decision Point, so it is engine code. That
single-object, non-pausing evaluation is how the interpreter happens to be
built today, not a design decision: core instructions can decide
(`DefId` and `Expr` in `crates/core`). The `engine-legend-rule-sba-order` ticket owns
it.

The check repeats only when the batch it performed changed state [CR#704.3].
If every event in the batch was replaced or prohibited, nothing was performed,
and repeating would loop forever on, for example, an indestructible creature
with lethal damage.

### 6. Broken state fails loudly; what the rules make impossible is ignored

Malformed core is refused at load ([core](core.md) law 5). A run-time read
that still finds an impossible state, such as a missing register, a value of
the wrong kind or an unknown verb, is a lowering or engine bug and fails
loudly, naming the defect. Everything the rules make impossible is ignored:
"any part of an instruction that's impossible to perform is ignored"
[CR#101.3], and that part does nothing. A departed object is one case: an
illegal target is not affected, and a spell or ability whose every target is
illegal doesn't resolve [CR#608.2b].

### 7. Members of one simultaneous batch are unordered

Members of one simultaneous batch are unordered for rules purposes: the event
log's commit order within a batch is representational only, and a rule that
reads it is an encoding defect. A batch is one simultaneous event
[CR#704.3,510.2], which can contain several occurrences [CR#603.2c]. Every
consumer that is sensitive to order is settled by its own rule, never by
position inside the batch: timestamps [CR#613.7m], the order of triggered
abilities [CR#603.3b], and last known information [CR#704.8]. Where the rules
make order matter, they say how: individual card draws are sequential events
[CR#121.2], and the owner arranges cards put into one graveyard at once
[CR#404.3], or into one position in a library at once [CR#401.4]. An
`Occurrence::Batch` is applied and matched as one occurrence, and every fact it
records in the history log shares one `HistEntry.batch` id; the log still stores
those facts one after another, so each also has a log position, and that
position means nothing between members of the same batch.
