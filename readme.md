## KD Optimization

This is a blackbox optimization algorithm that partition the space using a [kd-tree](https://en.wikipedia.org/wiki/K-d_tree) and performs the search using [Monte-Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search).

This rely on very few hypothesis (close-by points have a similar distribution of reward), lets us deal with the exploration/exploitation compromise in a very direct way and should be very resilient to a noisy target function.

However, this algorihm cuts the space into cubes and studies the distribution of their rewards : this coarse grained approach does not extract as much information from the alreaddy computed points as something like bayesian optimisation.
Additionnaly, dimensions with low impact on the target function can significantly degrade the search-speed (approach such as [Random EMbedding Bayesian Optimization](https://ml.informatik.uni-freiburg.de/papers/16-JAIR-REMBO.pdf) might be an interesting solution to this problem).

# TODO

**Work in progress!**

# Reference

It is currently unknown wether this is a spin-off an existing idea or fully new (unlikely).
