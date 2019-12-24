# Legends of Runeterra Deck Tracker

A work in progress Legends of Runeterra deck tracker inspired by [Hearthstone Deck Tracker].

## Implementation details

The deck tracker is currently built on some smaller crates:
* [`runeterra-database`], deserializes the JSON files found in the Legends of Runeterra [Data Dragon] and exposes nice Rust types to be used further in code.
* [`runeterra-game-api`], a wrapper crate over the Legends of Runeterra [Game Client API].

[`runeterra-database`]: runeterra-database
[`runeterra-game-api`]: runeterra-game-api

[Hearthstone Deck Tracker]: https://github.com/HearthSim/Hearthstone-Deck-Tracker
[Game Client API]: https://developer.riotgames.com/docs/lor#game-client-api
[Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
