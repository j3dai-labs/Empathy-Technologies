/*******************************************************************************
        File:           graph.h
        Author:         Marion Bohr
        Creation Date:  08/21/2024
        Description:    Library Header
        		For determining the shortest route on your learning journey
                        By means of single source algorithm of Dijkstra
        Build:          make -f Makefile
*******************************************************************************/
#ifndef GRAPH_H
#define GRAPH_H

#include <iostream>
#include <vector>
#include <queue>
#include <map>
#include <string>
#include <limits>

// Define INFINITY as the maximum integer value
const int INFINITY = std::numeric_limits<int>::max();

// Structure for a node in the adjacency list
struct AdjListNode {
    int dest;          // Destination node (as index in the graph)
    int weight;        // Weight of the edge
    AdjListNode* next; // Pointer to the next node
};

// Structure for the adjacency list
struct AdjList {
    AdjListNode* head; // Head of the adjacency list
};

// Structure for the graph
struct Graph {
    int vertices;                  // Number of nodes
    std::vector<AdjList> array;    // Array of adjacency lists
};

// Function declarations for graph operations
Graph* createGraph(int vertices);
AdjListNode* newAdjListNode(int dest, int weight);
void addEdge(Graph* graph, int src, int dest, int weight);
Graph* readGraphFromDotFile(const std::string& filename, std::map<std::string, int>& nodeIndex, std::vector<std::string>& indexToNode);
void printGraph(Graph* graph, const std::vector<std::string>& indexToNode);
std::vector<int> dijkstra(Graph* graph, int src, std::vector<int>& distance);
std::vector<int> createShortestPath(int targetNode, const std::vector<int>& predecessor);

#endif // GRAPH_H

