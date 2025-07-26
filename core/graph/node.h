#pragma once

#include "types/datatypes.h"
#include "pin.h"
#include "lib/json.hpp" // For json
using json = nlohmann::json;

#include <string>
#include <vector>
#include <memory>
#include <algorithm> // for std::find_if

class Node
{
public:
    Node(const std::string id, const std::string name, const std::string type);
    virtual ~Node() = default;

    const std::string& getId() const;
    const std::string& getName() const;
    const std::string& getType() const;
    const VectorData getPosition() const;
    void setPosition(const VectorData &position);

    // Main
    virtual void process() = 0;

    // Pins
    InputPin* getInputPin(const std::string &id) const;
    OutputPin* getOutputPin(const std::string &id) const;
    const std::vector<std::unique_ptr<InputPin>>& getInputPins() const;
    const std::vector<std::unique_ptr<OutputPin>>& getOutputPins() const;
    
    // Sorting
    int getInDegree() const;
    
    // Serialization
    virtual json getParameters() const;
    virtual void setParameters(const json &params);

    virtual json getProperties() const;
    virtual void setProperties(const json &properties);
protected:
    // for derived classes
    void addInputPin(PinDataType pinType, const std::string &id, const std::string &name, DataValue defaultValue);
    void addOutputPin(PinDataType pinType, const std::string &id, const std::string &name);
    
    std::string m_id;   // unique within node
    std::string m_name; // Display name
    std::string m_type; // Node type (e.g "AddNode, "MultiplyNode", etc.)

    VectorData m_position; // Position in the graph

    std::vector<std::unique_ptr<InputPin>> m_inputPins;
    std::vector<std::unique_ptr<OutputPin>> m_outputPins;
};