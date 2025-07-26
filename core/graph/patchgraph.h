#pragma once

#include "node.h"

#include "lib/json.hpp" // Nlohmann json
using json = nlohmann::json;

#include <string>
#include <vector>
#include <memory> // for std::unique_ptr
#include <map>    // for std::map

class PatchGraph
{
public:
    PatchGraph();
    ~PatchGraph() = default;

    // Node management
    Node *addNode(std::unique_ptr<Node> node);       // Take ownership of the node
    void removeNode(const std::string &nodeId);      // Unparent node from patch graph
    Node *findNode(const std::string &nodeId) const; // Find a node by its ID
    const std::map<std::string, Node *> &getAllNodes() const; // Get all nodes

    // Connection management
    void connect(const std::string &sourceNodeId, const std::string &outputPinId, const std::string &destNodeId, const std::string &inputPinId);
    void disconnect(const std::string &destNodeId, const std::string &inputPinId);

    // Processing
    void evaluate(); // Later will do topological sort
    void sort();

    // Serialization
    std::string title = "Untitled";
    std::string description = "No description";
    std::string author = "Unknown";

    json toJson() const;
    void fromJson(const json &j);

private:
    std::vector<std::unique_ptr<Node>> m_nodes; // Owned nodes
    std::map<std::string, Node *> m_nodeMap;    // Quick access, non-owning pointers
    std::vector<Node *> getTopologicallySortedNodes() const;
    std::vector<Node *> m_sorted_nodes;
};

