/*******************************************************************************
        File:           graph.cpp
        Author:         Marion Bohr
        Creation Date:  08/21/2024
        Description:    Library 
        		For determining the shortest route on your learning journey
                        By means of single source algorithm of Dijkstra
        Build:          make -f Makefile
*******************************************************************************/
#include "graph.h"
#include <fstream>
#include <sstream>
#include <set>
#include <algorithm>

// Helper function to create a new adjacency list node
AdjListNode* newAdjListNode(int dest, int weight) {
    AdjListNode* newNode = new AdjListNode;
    newNode->dest = dest;
    newNode->weight = weight;
    newNode->next = nullptr;
    return newNode;
}

// Helper function to create a graph
Graph* createGraph(int vertices) {
    Graph* graph = new Graph;
    graph->vertices = vertices;
    graph->array.resize(vertices);
    for (int i = 0; i < vertices; ++i) {
        graph->array[i].head = nullptr;
    }
    return graph;
}

// Helper function to add an edge to the graph
void addEdge(Graph* graph, int src, int dest, int weight) {
    AdjListNode* newNode = newAdjListNode(dest, weight);
    newNode->next = graph->array[src].head;
    graph->array[src].head = newNode;
}

// Function to read a graph from a .dot file
Graph* readGraphFromDotFile(const std::string& filename, std::map<std::string, int>& nodeIndex, std::vector<std::string>& indexToNode) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        std::cerr << "Error opening file.\n";
        return nullptr;
    }

    std::set<std::string> nodes;
    std::string line;
    Graph* graph = nullptr;

    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::string src, arrow, dest, label;
        int weight;

        if (iss >> src >> arrow >> dest >> label) {
            if (label.find("label=") != std::string::npos) {
                size_t pos = label.find("=");
                if (pos != std::string::npos) {
                    weight = std::stoi(label.substr(pos + 1));
                } else {
                    continue;
                }

                src.erase(std::remove(src.begin(), src.end(), ';'), src.end());
                dest.erase(std::remove(dest.begin(), dest.end(), ';'), dest.end());
                src.erase(std::remove(src.begin(), src.end(), '"'), src.end());
                dest.erase(std::remove(dest.begin(), dest.end(), '"'), dest.end());

                nodes.insert(src);
                nodes.insert(dest);
            }
        }
    }

    int vertices = nodes.size();
    graph = createGraph(vertices);

    int index = 0;
    for (const std::string& node : nodes) {
        nodeIndex[node] = index;
        indexToNode.push_back(node);
        index++;
    }

    file.clear();
    file.seekg(0, std::ios::beg);

    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::string src, arrow, dest, label;
        int weight;

        if (iss >> src >> arrow >> dest >> label) {
            if (label.find("label=") != std::string::npos) {
                size_t pos = label.find("=");
                if (pos != std::string::npos) {
                    weight = std::stoi(label.substr(pos + 1));
                } else {
                    continue;
                }

                src.erase(std::remove(src.begin(), src.end(), ';'), src.end());
                dest.erase(std::remove(dest.begin(), dest.end(), ';'), dest.end());
                src.erase(std::remove(src.begin(), src.end(), '"'), src.end());
                dest.erase(std::remove(dest.begin(), dest.end(), '"'), dest.end());

                addEdge(graph, nodeIndex[src], nodeIndex[dest], weight);
            }
        }
    }

    file.close();
    return graph;
}

// Function to print the graph (for testing)
void printGraph(Graph* graph, const std::vector<std::string>& indexToNode) {
    for (int v = 0; v < graph->vertices; ++v) {
        AdjListNode* focus = graph->array[v].head;
        std::cout << "\nAdjacency list of vertex " << indexToNode[v] << "\n head ";
        if (!focus) {
            std::cout << "-> [no outgoing edges]";
        }
        while (focus) {
            std::cout << "-> " << indexToNode[focus->dest] << " (weight=" << focus->weight << ")";
            focus = focus->next;
        }
        std::cout << "\n";
    }
}

// Dijkstra's algorithm
std::vector<int> dijkstra(Graph* graph, int src, std::vector<int>& distance) {
    int vertices = graph->vertices;
    std::vector<int> predecessor(vertices, -1);
    std::vector<bool> done(vertices, false);

    distance[src] = 0;
    using pii = std::pair<int, int>;
    std::priority_queue<pii, std::vector<pii>, std::greater<pii>> pq;
    pq.push({0, src});

    while (!pq.empty()) {
        int u = pq.top().second;
        pq.pop();

        if (done[u]) continue;
        done[u] = true;

        AdjListNode* focus = graph->array[u].head;
        while (focus) {
            int v = focus->dest;
            int weight = focus->weight;

            if (!done[v] && distance[u] + weight < distance[v]) {
                distance[v] = distance[u] + weight;
                predecessor[v] = u;
                pq.push({distance[v], v});
            }
            focus = focus->next;
        }
    }
    return predecessor;
}

// Function to create the shortest path
std::vector<int> createShortestPath(int targetNode, const std::vector<int>& predecessor) {
    std::vector<int> path;
    int u = targetNode;
    while (u != -1) {
        path.insert(path.begin(), u);
        u = predecessor[u];
    }
    return path;
}
