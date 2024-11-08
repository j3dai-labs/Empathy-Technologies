/*******************************************************************************
        File:           tao.cpp
        Author:         Marion Bohr
        Creation Date:  08/21/2024
        Description:    Determining the shortest route on your learning journey
                        By means of single source algorithm of Dijkstra.
                        Working with priority queue, optimal for sparse graphs.
                        Input:	Graph given as adjacent list in a dot-file and
                        	   	start-node as a source 
                        Output:	distances, predecessors and shortest path from
                        		source to each node in the graph
        Build:          make -f Makefile
        Usage:			tao <dot-file> <start-node>
*******************************************************************************/

#include <iostream>
#include "graph.h"

// Main program
int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <dot-file> <start-node>\n";
        return EXIT_FAILURE;
    }

    std::string filename = argv[1];
    std::string startNodeName = argv[2];

    std::map<std::string, int> nodeIndex;
    std::vector<std::string> indexToNode;
    Graph* graph = readGraphFromDotFile(filename, nodeIndex, indexToNode);
    if (graph == nullptr) {
        return EXIT_FAILURE;
    }

    if (nodeIndex.find(startNodeName) == nodeIndex.end()) {
        std::cerr << "Error: Start node not found in the graph.\n";
        delete graph;
        return EXIT_FAILURE;
    }
    int source = nodeIndex[startNodeName];

    std::cout << "Graph:\n";
    printGraph(graph, indexToNode);

    std::vector<int> distance(graph->vertices, INFINITY);
    std::vector<int> predecessor = dijkstra(graph, source, distance);

    std::cout << "\nDistances from start node " << startNodeName << ":\n";
    for (int i = 0; i < graph->vertices; ++i) {
        if (distance[i] == INFINITY) {
            std::cout << "Vertex: " << indexToNode[i] << ", Distance: unreachable\n";
        } else {
            std::cout << "Vertex: " << indexToNode[i] << ", Distance: " << distance[i] << "\n";
        }
    }

    std::cout << "\nShortest paths from start node " << startNodeName << ":\n";
    for (int i = 0; i < graph->vertices; ++i) {
        std::vector<int> path = createShortestPath(i, predecessor);
        std::cout << "Path to " << indexToNode[i] << ": ";
        if (path.size() == 1 && path[0] == i && distance[i] == INFINITY) {
            std::cout << "no path";
        } else {
            for (int node : path) {
                std::cout << indexToNode[node] << " ";
            }
        }
        std::cout << "\n";
    }

    delete graph;
    return 0;
}

