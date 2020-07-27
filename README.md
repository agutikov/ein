# ein-bot
Bot solving Zebra Puzzles


Initial version of representation of problem as graph.
![Clustered with two-direction arcs](https://github.com/agutikov/ein-bot/blob/master/clustered.svg?raw=true)



Grouping by type with relations "is instance of" and "is type of".
![Link type example](https://github.com/agutikov/ein-bot/blob/master/link_type.svg?raw=true)

Simplify view by reducing bi-directional links.
![Link type example simple](https://github.com/agutikov/ein-bot/blob/master/link_type_2.svg?raw=true)

Arrow is "labled" by type of source and destination (this case - only destination).
![Link label example](https://github.com/agutikov/ein-bot/blob/master/link_label.svg?raw=true)

Multiple types of arrows between two objects.
House_2 is to the right of House_1.
And both are next to another one.
![Multiple link types](https://github.com/agutikov/ein-bot/blob/master/multiple_link_types.svg?raw=true)


Apriori relations.
![Apriori relations](https://github.com/agutikov/ein-bot/blob/master/apriori_relations.svg?raw=true)


Grouping by links to "type" nodes and simplified bi-directional links.
![Linked with type nodes](https://github.com/agutikov/ein-bot/blob/master/linked.svg?raw=true)


Constraint of number of attributes of the same type.
For any Object instance of Type_1 for any Type != Type_1 there can be only one Path from Object to Type.
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/single_attribute_constraint.svg?raw=true)




Inference.

Triangle or transitive inference rule.
![House color constraint](https://github.com/agutikov/ein-bot/blob/master/inference_triangle.svg?raw=true)



![House color constraint](https://github.com/agutikov/ein-bot/blob/master/inference_square.svg?raw=true)





# Links


https://en.wikipedia.org/wiki/Zebra_Puzzle

http://rosettacode.org/wiki/Zebra_puzzle

https://wiki.opencog.org/w/Einstein's_Puzzle







