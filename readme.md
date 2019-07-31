# KD Optimization

This is a blackbox optimization algorithm that partition the space using a [kd-tree](https://en.wikipedia.org/wiki/K-d_tree) and performs the search using [Monte-Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search).

## Algorithm

### Implementation

The search space is splitted using a kd-tree. At each step we go down the tree in order to decide on the hypercube in which to sample the next point.

When we get into an empty hypercube, we sample from it.

When we get into a leaf containning a single value, we split it in the middle of its longest axis (producing a balanced tree).
We put the value into the corresponding children (making it a singleton) and sample from the other children (which was empty).

When we get to an alreaddy splitted node, we use the [ucb-tuned algorithm](http://imagine.enpc.fr/~audibert/ucbtuned0.5.pdf) in order to decide the children that we will explore during this exploration.

### Strengths and weaknesses

This rely on very few hypothesis (close-by points have a similar distribution of reward) and lets us deal with the exploration/exploitation compromise in a very direct way which makes this approach very resilient to noisy target function.

However, this algorihm cuts the space into cubes and studies the distribution of their rewards : this coarse grained approach does not extract as much information from the alreaddy computed points as something like bayesian optimisation. Which mean that this algorithm would probably not be recommended if the noise is low enough.

Additionnaly, dimensions with low impact on the target function can significantly degrade the search-speed (approach such as [Random EMbedding Bayesian Optimization](https://ml.informatik.uni-freiburg.de/papers/16-JAIR-REMBO.pdf) might be an interesting solution to this problem).

Problems with several global minimums (and not just small local minimums) tend to lead to divide the search into several points of the search space degrading the efficiency of the algorithm.

Thus, **the ideal target function has a single minimum, no useless input dimenssions and is very noisy**.

## TODO

- Map input to unit hypercube in order to avoid giving too much weight to dimenssions that are large in the original search space.

- We could add a function that initialise the search from an existing set of points.

- Add proper examples to readme and doc (in the meantime, see `main.rs`).

## Reference

I currently do not know wether this algorithm is new (unlikely) or alreaddy exists somewhere in the literature.
