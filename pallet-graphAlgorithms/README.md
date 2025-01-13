# Finding the shortest path on your Learning Journey 

## Overview

This pallet provides a way to find the shortest path on your learning journey to reach new skills and knowledge by overcoming the shortest gaps based on your already existing skills and knowledge. These are represented by a directed, weighted graph.  

### Input

- Dot-file containing an adjacency list representing the graph. Its node names can be either integer values or strings.
- Start node where the journey should begin. This is a node name existing in the graph.

### Ouput

- A representation of the graph.
- The shortest paths beginning with the start node to each node in the graph. 

## Idea
The learner's skills and knowledge have already been determined in advance of the learning journey. When the learner starts their learning journey, the system suggests the optimal sequence of topics based on their existing skills and knowledge. The stations or modules to be acquired are represented internally in the system by nodes in a directed weighted graph. The optimal path for the learning journey is determined automatically on the basis of Dijkstra's algorithm. After each station reached, the learner receives a reward, e.g. in the form of a badge or a certificate documenting the achievement of this station. However, each station that the learner reaches also changes the starting position for the next station on the learning journey, which affects the weights and the nodes in the graph that have already been completed. If the learner has completed their learning journey in full and thus achieved all the relevant stations and badges, they could also receive a special certificate of completion, for example.

## Execution
This pallet acts as an off-chain worker (OCW) in the Polkadot system. The shortest paths from a given start node are calculated off-chain. The learner decides whether to work out the next (ideal) station determined according to Dijkstra or a self-selected one. The “completed” station is recorded on the blockchain and a badge can now be issued. For each subsequent station, the OCW is executed again based on the new initial situation. The learning journey ends after all the designated stations have been reached.
