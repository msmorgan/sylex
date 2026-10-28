import Semantics.Cards.Check.Fixtures
import Semantics.Cards.Check.Shard00
import Semantics.Cards.Check.Shard01
import Semantics.Cards.Check.Shard02
import Semantics.Cards.Check.Shard03
import Semantics.Cards.Check.Shard04
import Semantics.Cards.Check.Shard05
import Semantics.Cards.Check.Shard06
import Semantics.Cards.Check.Shard07
import Semantics.Cards.Check.Shard08
import Semantics.Cards.Check.Shard09
import Semantics.Cards.Check.Shard10
import Semantics.Cards.Check.Shard11
import Semantics.Cards.Check.Shard12
import Semantics.Cards.Check.Shard13
import Semantics.Cards.Check.Shard14
import Semantics.Cards.Check.Shard15

/-!
# Semantics.Cards

The canon card bench: every card in `plugins_v3/canon/cards/`, with its `ok_` theorem. `genCanon`
(`lean/lakefile.lean`) generates the data modules `Semantics.Cards.Data.*` and the check modules
`Semantics.Cards.Check.*` imported here. Proofs read canon cards from
`Semantics.Cards.Data.Fixtures` and never import this module.
-/
