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
This means you need a _synchronized head reference_ for LRU. Of course there are clever tricks you can
do to minimize this cost, but intrinsically, there's a point of synchronized contention for LRU updates
on **read**.

Eviction for LRU is quite handy - you just look at the oldest thing and remove it from the cache map.

For Sieve, contention is met on **write**, when the cache is full. The "hand" reference is synchronized,
but importantly you can still read without blocking. Sieve needs a list of nodes through which it can
iterate, not unlike LRU. However, the order is unimportant with Sieve.

While read is non-blocking, write in a full Sieve cache has a little more work to do than write in a
full LRU cache. The Sieve hand must walk until it finds a node that is marked unused, and evict it. As
it walks past them, the Sieve cache marks used nodes unused.

# State flow for Sieve cache
It's probably easiest to understand this visually.

Let's consider a cache with some keys, and the used/unused bit named `read` with 0 or 1 to indicate
false and true.

**Initial state:**
```
cache: a  b  c  d  e  f  g
read:  0  0  0  0  0  0  0
hand:  ^
```

**get(b):**
```
cache: a  b  c  d  e  f  g
read:  0  1  0  0  0  0  0
hand:  ^
```

**set(h):**
```
- consider the hand element: a, read=0. It's unread so let's evict it and leave the hand index as-is.
- now there's room, insert h
cache: b  c  d  e  f  g  h
read:  1  0  0  0  0  0  0
hand:  ^
```

**set(i):**
```
- consider the hand element: b, read=1. It's read so let's set it unread and move forward.
cache: b  c  d  e  f  g  h
read:  0  0  0  0  0  0  0
hand:     ^

- consider the hand element: c, read=0. It's unread so let's evict it and leave the hand index as-is.
- now there's room, insert i
cache: b  c  d  e  f  g  h  i
read:  0  0  0  0  0  0  0  0
hand:     ^
```

So for any key you read, you just set the `read` flag to 1. This can be accomplished with an atomic boolean.
For any key you seek to evict, you look at the `hand` position and atomically swap 0.
If you swapped out 1, you move the hand index. If you swapped out 0, you evict the hand element.
