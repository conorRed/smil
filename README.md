The hope is to make a pgm library in Rust. Transformation of variables between nodes should be
equations. Focusing first on representations. How will data be represented in this library?
The goal is to construct a “model of a system about which we would like to reason”.
More generally, just looking at declarative representations of a model so that the reasoning process is
independent of the representation.


## Bayesian Networks

A graph, where the nodes are random variables. Each random variable has a domain and range. The space of output values
of the random variable has an associated probability defined

The goal is representation, inference (setting one of the variables and seeing how the network
reacts) and learning (fitting the model to data).

### Factorisation

A process by which we try to break down the computational expensive object $P(x_1, x_2, ..., x_n)$
into a product of smaller, simpler, objects called factors. This is done through identification of
independence such that $P(x_1, x_2, ..., x_n) = P(x_1)P(x_2)...P(x_n)$ in the case where all these
random variables were independent.

### Parametrisation

#### Example

If you were trying to determine if Alice will go to the gym. If her decision is based on
whether it's raining/not raining and whether there's traffic or not. Say, this person also has a
friend, Bob,  who also is faced with a similar decision. The situation is crudely modelled with

- B: whether Bob went to the gym.
- T: traffic/ no traffic.
- R: Raing/ no rain.
- A: whether tracy goes to the gym.

In this model, the state of the world is describe in 16 ways (2x2x2x2). To define a probability
distribution across these worlds we need to provide a measure for each state.

If we factor this distribution though and put in our knowledge about independence we might have:

- P(A, B, T, R) = P(A | B, T, R)P(B, T, R)
- P(A | B, T, R)P(B, T, R) = P(A | B, T, R)P(B| T, R)P(T, R)
- = P(A | B, T, R)P(B| T, R)P(T | R)P(R)

We can now use the normalisation of probabilities to reduce the amount of measures we have to
specifiy. For P(A | B, T, R) there 8 states for each state of A. But we only need to specify for $n - 1$
states of A where $n$ is the number of states because we just subtract from one to get it's table
for its last state.

The more we factor, the more we can take advantage of normalisation. If whether Alice goes to the
gym is independent of Bobs decision and rain and traffic are independent (although, this might be a
bad assumption). Then

- $P(A, B, T, R) = P(A |T, R)P(B| T, R)P(T)P(R)$
- States to specify a measure for is 4+4+2 which is 10. Rather than 15.

#### Inference

P(T | A): Probability that there was traffic, given we saw Alice at the gym.
- $P(T = 1| A =1) = \frac{P(A = 1 | T = 1, B, R)P(A = 1, T, B, R)}{P(T = 1, A, B, R)}$



## Types

- Bayesian Network is a pair (G, P) where P _factorizes_ over G.
- P is a set of CPD's (Conditional Probability Distributions) associated with the nodes in G.

## Converting To PNG using Dot

`dot -T png -O img/<filename>.dot`
