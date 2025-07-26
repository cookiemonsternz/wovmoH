#include "patchgraph.h"
#include "nodes/corenodes.h" // This is a bad way of doing it
// In the future, for user defined nodes, it will not be easily extendable.

#include <queue>

PatchGraph::PatchGraph() {}

// Node management
// Add Node
Node *PatchGraph::addNode(std::unique_ptr<Node> node)
{
    m_nodes.push_back(std::move(node)); // transfer ownership to m_nodes new pointer

    Node *nodePtr = m_nodes.back().get(); // get  raw pointer to  node
    m_nodeMap[nodePtr->getId()] = nodePtr;

    return nodePtr;
};
// Remove Node
void PatchGraph::removeNode(const std::string &nodeId)
{
    // First delete from map, then vector. If vector is deleted, it deallocates the memory (bc of unique_ptr))
    // and then map has dangling pointer
    auto it = m_nodeMap.find(nodeId);
    if (it != m_nodeMap.end())
    {
        m_nodeMap.erase(it); // remove from map

        auto nodeIt = std::remove_if(m_nodes.begin(), m_nodes.end(),
                                     [&nodeId](const std::unique_ptr<Node> &node)
                                     { return node->getId() == nodeId; });

        if (nodeIt != m_nodes.end())
        {
            m_nodes.erase(nodeIt, m_nodes.end()); // rm from vector
        }
    }
};
// Find Node
Node *PatchGraph::findNode(const std::string &nodeId) const
{
    auto it = m_nodeMap.find(nodeId);
    if (it != m_nodeMap.end())
    {
        return it->second; // return the raw pointer
    }
    return nullptr; // not found
};
// Get all nodes
const std::map<std::string, Node *> &PatchGraph::getAllNodes() const
{
    return m_nodeMap;
}
// Connect
void PatchGraph::connect(const std::string &sourceNodeId, const std::string &outputPinId, const std::string &destNodeId, const std::string &inputPinId)
{
    Node *sourceNode = findNode(sourceNodeId);
    Node *destNode = findNode(destNodeId);

    if (!sourceNode || !destNode)
    {
        // qDebug() << "Error: Source or destination node not found in graph";
        return;
    }

    OutputPin *outputPin = sourceNode->getOutputPin(outputPinId);
    InputPin *inputPin = destNode->getInputPin(inputPinId);
    if (outputPin && inputPin)
    {
        outputPin->connect(inputPin); // Connect the pins
    }
    else
    {
        // qDebug() << "Error: Output or input pin not found";
    }
};
// Disconnect
void PatchGraph::disconnect(const std::string &destNodeId, const std::string &inputPinId)
{
    Node *destNode = findNode(destNodeId);
    if (!destNode)
    {
        // Handle error: node not found
        return;
    }

    InputPin *inputPin = destNode->getInputPin(inputPinId);
    if (inputPin)
    {
        inputPin->disconnect(); // Disconnect the pin
    }
    else
    {
        // Handle error: pin not found
    }
};
// Evaluate
void PatchGraph::evaluate()
{
    for (Node *node : m_sorted_nodes)
    {
        node->process(); // Call process on each node
    }
};
// Sort graph
void PatchGraph::sort()
{
    m_sorted_nodes = getTopologicallySortedNodes();
}
// Topological sort - Khan's algorithm
// see https://www.geeksforgeeks.org/topological-sorting-indegree-based-solution/
// Raw pointers for nodes
std::vector<Node *> PatchGraph::getTopologicallySortedNodes() const
{
    std::map<Node *, int> in_degrees;
    std::queue<Node *> queue;
    std::vector<Node *> sorted_result;

    // Init indegrees
    for (const auto &node_unique_ptr : m_nodes)
    {
        Node *node = node_unique_ptr.get();

        int degree = node->getInDegree();
        in_degrees[node] = degree;

        // Do it now instead of iterating again later
        if (degree == 0)
        {
            queue.push(node);
        }
    }

    while (!queue.empty())
    {
        Node *current_node = queue.front(); // get data
        queue.pop();                        // remove element

        sorted_result.push_back(current_node);

        for (const std::unique_ptr<OutputPin> &out_pin_unique_ptr : current_node->getOutputPins())
        {
            OutputPin *out_pin = out_pin_unique_ptr.get();
            for (InputPin *in_pin : out_pin->getConnectedInputPins())
            {
                Node *connected_node = in_pin->getParentNode();
                if (connected_node)
                {
                    in_degrees[connected_node]--;
                    if (in_degrees[connected_node] == 0)
                    {
                        queue.push(connected_node);
                    }
                } // Connected input parent node exists
            } // foreach input pin
        } // foreach output pin
    } // while queue

    // Check for loops
    if (sorted_result.size() != m_nodes.size())
    {
        // qDebug() << "ERROR - Cycle in graph, returned empty dict, this is probably why your code broke!";
        return {};
    }

    return sorted_result;
}

// JSON serialization
json PatchGraph::toJson() const
{
    std::vector<json> nodesJson;
    std::vector<json> connectionsJson;
    for (const auto &node : m_nodes)
    {
        json nodeJson = {
            {"id", node->getId()},
            {"name", node->getName()},
            {"type", node->getType()},
        };
        nodeJson["parameters"] = node->getParameters();
        nodeJson["properties"] = node->getProperties();
        nodesJson.push_back(nodeJson);
    }
    for (const auto &node : m_nodes)
    {
        for (const auto &outPin : node->getOutputPins())
        {
            for (InputPin *inPin : outPin->getConnectedInputPins())
            {
                json connectionJson = {
                    {"sourceNodeId", node->getId()},
                    {"outputPinId", outPin->getId()},
                    {"destNodeId", inPin->getParentNode()->getId()},
                    {"inputPinId", inPin->getId()}};
                connectionsJson.push_back(connectionJson);
            }
        }
    }
    nlohmann::ordered_json j;
    j = {
        {"metadata", {{"version", 0.1}, {"title", title}, {"description", description}, {"author", author}}},
        {"nodes", nodesJson},
        {"connections", connectionsJson}};
    return j;
}

void PatchGraph::fromJson(const json &j)
{
    // delete all nodes
    m_nodes.clear();
    m_nodeMap.clear();
    m_sorted_nodes.clear();

    // metadata
    if (j.contains("metadata"))
    {
        title = j["metadata"]["title"];
        description = j["metadata"]["description"];
        author = j["metadata"]["author"];
    }

    // nodes
    if (j.contains("nodes"))
    {
        for (const auto &nodeJson : j["nodes"])
        {
            std::string id = nodeJson["id"];
            std::string name = nodeJson["name"];
            std::string type = nodeJson["type"];

            // Create a new node based on the type
            Node *node = nullptr;
            if (type == "ConstantNumberNode")
            {
                node = new ConstantNumberNode(id, name);
            }
            else if (type == "AddNode")
            {
                node = new AddNode(id, name);
            }
            else if (type == "MultiplyNode")
            {
                node = new MultiplyNode(id, name);
            }
            else if (type == "DebugNode")
            {
                node = new DebugNode(id, name);
            }
            else
            {
                // qDebug() << "Unknown node type:" << type;
                continue; // Skip unknown types
            }
            // Set parameters
            if (nodeJson.contains("parameters"))
            {
                // qDebug() << nodeJson["parameters"].dump();
                node->setParameters(nodeJson["parameters"]);
            }

            if (nodeJson.contains("properties"))
            {
                node->setProperties(nodeJson["properties"]);
            }

            // add to patch graph
            addNode(std::unique_ptr<Node>(node));
        }
    }
    // connections
    if (j.contains("connections"))
    {
        for (const auto &connectionJson : j["connections"])
        {
            std::string sourceNodeId = connectionJson["sourceNodeId"];
            std::string outputPinId = connectionJson["outputPinId"];
            std::string destNodeId = connectionJson["destNodeId"];
            std::string inputPinId = connectionJson["inputPinId"];

            connect(sourceNodeId, outputPinId, destNodeId, inputPinId);
        }
    }
}
