# Legends of Runeterra Deck Tracker

A work in progress Legends of Runeterra deck tracker inspired by [Hearthstone Deck Tracker].

## Implementation details

The deck tracker is currently built on some smaller crates:
* [`runeterra-database`], deserializes the JSON files found in the Legends of Runeterra [Data Dragon] and exposes Rust types with field and string values to be used as data source for the [`runeterra-core`] types.
* [`runeterra-game-api`], a wrapper crate over the Legends of Runeterra [Game Client API].
* [`runeterra-core`], crate containing base types to be used by the main deck tracker application.

[`runeterra-database`]: runeterra-database
[`runeterra-game-api`]: runeterra-game-api
[`runeterra-core`]: runeterra-core

[Hearthstone Deck Tracker]: https://github.com/HearthSim/Hearthstone-Deck-Tracker
[Game Client API]: https://developer.riotgames.com/docs/lor#game-client-api
[Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
