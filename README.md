# ein-bot
Bot solving Zebra Puzzles


Initial version of representation of problem as graph.
![Clustered with two-direction arcs](https://github.com/agutikov/ein-bot/blob/master/files/clustered.svg?raw=true)



Grouping by type with relations "is instance of" and "is type of".
![Link type example](https://github.com/agutikov/ein-bot/blob/master/files/link_type.svg?raw=true)

Simplify view by reducing bi-directional links.
![Link type example simple](https://github.com/agutikov/ein-bot/blob/master/files/link_type_2.svg?raw=true)

Arrow is "labled" by type of source and destination (this case - only destination).
![Link label example](https://github.com/agutikov/ein-bot/blob/master/files/link_label.svg?raw=true)

Multiple types of arrows between two objects.
House_2 is to the right of House_1.
And both are next to another one.
![Multiple link types](https://github.com/agutikov/ein-bot/blob/master/files/multiple_link_types.svg?raw=true)


Apriori relations.
![Apriori relations](https://github.com/agutikov/ein-bot/blob/master/files/apriori_relations.svg?raw=true)


Grouping by links to "type" nodes and simplified bi-directional links.
![Linked with type nodes](https://github.com/agutikov/ein-bot/blob/master/files/linked.svg?raw=true)

## Constraints.

### Single type constraint
Each object node has only one link with type node.

### Number of attributes constraint
Constraint of number of attributes of the same type.
For any Object instance of Type_1 for any Type != Type_1 there can be only one Path from Object to Type.
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/single_attribute_constraint.svg?raw=true)

This case - both constraints are equal - every node can have only one link of every possible type.


## Inference.

### Triangle inference rule.
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/inference_triangle.svg?raw=true)


### Square inference rule.
Applicable (in this case) for spatial relations: right and next.
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/inference_square.svg?raw=true)
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/inference_square_2.svg?raw=true)
This kind of relations are possible between objects of the same type.

## Hypothesis.

Hypothesis - is a copy of a graph as a hypothetical state of relations.
It contains state on previous step and an added link that satisfies constraints and all inferred links.


### Generation of hypothesis.

1. Select object node of some Type_1.
2. Select another Type_2.
3. Generate a set of links from given node to all objects of Type_2.
4. Apply constraints - if object of Type_2 already has a link with object of Type_1 - then drop this hypothetic link.
Set of links are set of hypothesis.

![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/hgen.svg?raw=true)



### Testing of hypothesis.

Testing of hypothesis is application of contraints after infrence.
1. Apply inference - infer all possible links.
2. Apply constraints after each inferred link.
If constraint is not satisfied - then test failed - then hypothesis is wrong.


![House color constraint](https://github.com/agutikov/ein-bot/blob/master/files/htest_1.svg?raw=true)


How to define constraint of existance of Ivory house to the left of Green one?




### Verification of hypothesis.

Only one hypothesis from a set is true - other are wrong.
So if there is more than one hypothesis that pass testing - then there is ambiguity.

### Multilevel hypothesis.

Ambiguity can be solved by multilevel hypothesis.
For each true hypothesis - generate another set of hypothesis and verify them.
- If no hypothesis of next level is true - then initial hypothesis failed.
- If only one hypothesis of next level is true - then initial hypothesis is true.
- If there is ambiguity on next level (more than one hypothesis passed testing) - then go one level deeper.

Objects that has new links after previous step of hypothesis generation and inference
are good starting objects for next level hypothesis generation. 


### Ambiguity.

Finally - if no more hypothesis can be generated - then problem conditions are ambiguous and there is more than one solution.

Btw - it's normal for problems with multiple solutions.


## Final Algorythm

```
S - relations state.
Constraint verification:    verify : S -> bool;
Inference:                  infer : S -> S;
Hypothesis generation:      hgen : S -> [S];

s = infer(s)
while (not solved(s)):
    h = hgen(s)
    while (len(h) > 1):
        for s in h:
            if verify(s):
                s = infer(s)
            else:
                del s

        for s in h:
            # replace existing hypothesis with muptiple next-level hypothesis
            h += hgen(s)
            del s

    # only one hypothesis alive
    s = h[0]
```





# Links


https://en.wikipedia.org/wiki/Zebra_Puzzle

http://rosettacode.org/wiki/Zebra_puzzle

https://wiki.opencog.org/w/Einstein's_Puzzle







