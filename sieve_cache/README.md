Let's start from first principles: What is a cache? It's just a way to store data.

Sometimes you have more data than you have memory. That's fine; it's where **cache eviction**
comes in. Cache eviction is just **a way to choose which data to drop when you run out of memory.**

You probably know about LRU (least recently used) cache eviction. If not, it's pretty close
to what it sounds like! You track when each item is read or written. When you run out of memory
and need to make some room, **you kick out the item that has been untouched for the longest.**

LRU is a famous and old strategy, but it's far from the only cache eviction strategy. It's one
that many folks have implemented though, so let's try something different.

Sieve is a relatively new cache eviction strategy. It picks which things to evict based on whether
they have been used recently.

Philosophically different from LRU, a rough comparison would be:
* LRU concerns itself with aggressively retaining useful data.
* Sieve concerns itself with rapidly getting rid of useless data.

Roughly, LRU's failure mode is retaining useless data, while Sieve's failure mode is getting rid
of useful data. LRU pays a full buffer's lifetime to evict anything, while Sieve evicts more eagerly.
This is not enough information to judge which is better, so let's look at what it takes to achieve
either algorithm.

LRU requires something like a queue. A hash map with the values expressing a linked list is a fairly
common method of implementation. Upon a **read**, LRU requires a move or swap of the element's
position into the most recently used position, to allow the least recently read to fall toward the end.

Eviction for LRU is quite handy - you just look at the oldest thing and remove it from the cache map.


