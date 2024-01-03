The hope is to make a pgm library in Rust. Transformation of variables between nodes should be
equations. Focusing first on representations. How will data be represented in this library?
The goal is to construct a “model of a system about which we would like to reason”.
More generally, just looking at declarative representations of a model so that the reasoning process is
independent of the representation.


## Bayesian Networks

A graph, where the nodes are random variables. Each random variable has a domain and range. The space of output values
of the random variable has an associated probability defined 

## Types

- Bayesian Network is a pair (G, P) where P _factorizes_ over G.
- P is a set of CPD's (Conditional Probability Distributions) associated with the nodes in G.

## Converting To PNG using Dot

`dot -T png -O img/<filename>.dot`
