# Changes to Rubin

Tracks changes between each release.

Any changes referring to `Net` are related to the client/server stuff
Any changes referring to `Non-Net` are related to the `MemStore` and `PersistentStores`

## v0.4.0

Added support for two new features: INCR and DECR.

A new counter store was added which keeps track of counters associated with given values (i.e keys).
INCR increments a given value by one and DECR decrements a given value by one.
Not _that_ big of an update but still, support for more Redis-like things!

* Added new INCR and DECR commands to increment and decrement values in Rubin (Net / Non-Net)
    * New store was added to hold these values
    * Each value is incremented / decremented by one on each call
* Minor fixes / improvements

## v0.3.2

This updated fixed an issue with passing multiple values when adding to the string store.

## v0.3.1

This update adds some logging into the server portion of the `net` module.

* Added tracing logging to the server (Net)
* Updated internals of error handling for the server (Net)
* New dump command to dump the `MemStore` out to disk (Net, Non-net)

## v0.3.0

This update reworks the internals of how the inner storage in the `MemStore` works.
Each new store that would be added would add a load of unnecessary duplication so they've been combined into a generic `InnerStore` type.
This should allow for less work going forward but there is probably a better way to do it.
As the `MemStore` is the core feature of this library, this _shouldn't_ affect the `Net` stuff and `PersistentStore` as they just wrap around the `MemStore` calls.

* Overhauled how the inner store works (Net, Non-net)
    * There is now **ONE** generic store that will cover all types (e.g. `String`, `Vec<T>`, etc.)
    * This means there is now a blanket implentation for each store going forward.
    * Specifics for a given type can be handled in the wrapper implementation (i.e. adding an item to a specific Vec)
* Reworked the API for the store to use the new `InnerStore` type (Non-net)
* Documentation updates (Net, Non-net)

## v0.2.0

* Updated the ability to name your own storefile when using a `PersistentStore` (Non-net)
* Added removal a string from the string store (Net, Non-Net)
* Added clearing the string store (Net / Non-net)
* Added ability to get shared ref to string store (Non-net)
* Minor edits and tweaks
