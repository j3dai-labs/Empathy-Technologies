Off-chain worker that can process dot files with integers or character strings as node names and a start node, but without triggering transactions on the blockchain:

	- Process dot files dynamically (node names as integers or strings).
	- Call the Dijkstra algorithm to calculate the shortest paths.

To be noted/to be added:
	- Dynamic input: The off-chain worker receives the dot file and the start node either via RPC or another method (still to be set up). 
	- Error handling: ensure that all error sources (such as invalid files, invalid node names or incorrect formats) are handled correctly to prevent unforeseen cancellations.
	- Optimisation if necessary
 
