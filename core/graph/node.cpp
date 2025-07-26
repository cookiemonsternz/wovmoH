#include "node.h"
#include "lib/json.hpp" // For json

using json = nlohmann::json;

Node::Node(const std::string id, const std::string name, const std::string type)
    : m_id(id),
      m_name(name),
      m_type(type)
{
}
const std::string &Node::getId() const
{
    return m_id;
};
const std::string &Node::getName() const
{
    return m_name;
};
const std::string &Node::getType() const
{
    return m_type;
};
const VectorData Node::getPosition() const
{
    return m_position;
};
void Node::setPosition(const VectorData &position)
{
    m_position = position;
};
// Pins
// Find input pin by id
InputPin *Node::getInputPin(const std::string &id) const
{
    auto it = std::find_if(
        m_inputPins.begin(),
        m_inputPins.end(),
        [&id](const std::unique_ptr<InputPin> &pin)
        { return pin->getId() == id; });
    if (it != m_inputPins.end())
    {
        return it->get();
    }
    return nullptr;
};
// Find output pin by id
OutputPin *Node::getOutputPin(const std::string &id) const
{
    auto it = std::find_if(
        m_outputPins.begin(),
        m_outputPins.end(),
        [&id](const std::unique_ptr<OutputPin> &pin)
        { return pin->getId() == id; });
    if (it != m_outputPins.end())
    {
        return it->get();
    }
    return nullptr;
};
// Get all input pins
const std::vector<std::unique_ptr<InputPin>> &Node::getInputPins() const
{
    return m_inputPins;
};
// Get all output pins
const std::vector<std::unique_ptr<OutputPin>> &Node::getOutputPins() const
{
    return m_outputPins;
};

int Node::getInDegree() const
{
    int inDegree = 0;
    for (const auto &pin : m_inputPins)
    {
        if (pin->isConnected())
        {
            inDegree++;
        }
    }
    return inDegree;
};

// Serialization
json Node::getParameters() const
{
    json params = json::object();
    for (const auto& in_pin_unique_ptr : m_inputPins)
    {
        InputPin *in_pin = in_pin_unique_ptr.get();
        if (in_pin && !in_pin->isConnected()) {
            DataValue value = in_pin->getValue();
            if (std::holds_alternative<double>(value)) { // Number
                params[in_pin->getId()] = std::get<double>(value);
            } else if (std::holds_alternative<bool>(value)) { // Bool
                params[in_pin->getId()] = std::get<bool>(value);
            } else if (std::holds_alternative<ColorData>(value)) { // Color
                ColorData color = std::get<ColorData>(value);
                params[in_pin->getId()] = {{"r", color.r}, {"g", color.g}, {"b", color.b}, {"a", color.a}};
            } else if (std::holds_alternative<VectorData>(value)) { // Vector
                VectorData vec = std::get<VectorData>(value);
                params[in_pin->getId()] = {{"x", vec.x}, {"y", vec.y}, {"z", vec.z}};
            } else {
                params[in_pin->getId()] = nullptr; // Unknown type
            }
        }
    }
    return params;
}

void Node::setParameters(const json &params)
{
    for (auto it = params.begin(); it != params.end(); ++it)
    {
        InputPin *pin = getInputPin(it.key());
        if (pin && !pin->isConnected())
        {
            if (it.value().is_number())
            {
                pin->setValue(DataValue(it.value().get<double>()));
            }
            else if (it.value().is_boolean())
            {
                pin->setValue(DataValue(it.value().get<bool>()));
            }
            else if (it.value().is_object() && it.value().contains("r") && it.value().contains("g") && it.value().contains("b") && it.value().contains("a"))
            {
                ColorData color;
                color.r = it.value()["r"];
                color.g = it.value()["g"];
                color.b = it.value()["b"];
                color.a = it.value()["a"];
                pin->setValue(DataValue(color));
            }
            else if (it.value().is_object() && it.value().contains("x") && it.value().contains("y") && it.value().contains("z"))
            {
                VectorData vec;
                vec.x = it.value()["x"];
                vec.y = it.value()["y"];
                vec.z = it.value()["z"];
                pin->setValue(DataValue(vec));
            }
            else
            {
                pin->setValue(DataValue(std::monostate())); // Unknown type
            }
        }
    }
}

json Node::getProperties() const
{
    json properties = json::object();
    json position = m_position.to_json();
    properties["position"] = position;

    json inputPinsArray = json::array();
    for (const std::unique_ptr<InputPin>& pin : getInputPins()) {
        json pinJson = json::object();
        pinJson["name"] = pin->getName();
        pinJson["id"] = pin->getId();
        pinJson["type"] = pin->getDataTypeString();
        inputPinsArray.push_back(pinJson);
    }
    properties["inputPins"] = inputPinsArray;

    json outputPinsArray = json::array();
    for (const std::unique_ptr<OutputPin>& pin : getOutputPins()) {
        json pinJson = json::object();
        pinJson["name"] = pin->getName();
        pinJson["id"] = pin->getId();
        pinJson["type"] = pin->getDataTypeString();
        outputPinsArray.push_back(pinJson);
    }
    properties["outputPins"] = outputPinsArray;

    return properties;
}

void Node::setProperties(const json &properties) 
{
    if (!properties.contains("position")) { return; }
    json pos = properties["position"];
    
    setPosition(
        VectorData(
            pos["x"],
            pos["y"],
            pos["z"]
        )
    );
}

// Protected methods (for derived classes)
void Node::addInputPin(PinDataType pinType, const std::string &id, const std::string &name, DataValue defaultValue = std::monostate()) {
    m_inputPins.push_back(std::make_unique<InputPin>(this, pinType, id, name, defaultValue));
};
void Node::addOutputPin(PinDataType pinType, const std::string &id, const std::string &name) {
    m_outputPins.push_back(std::make_unique<OutputPin>(this, pinType, id, name));
};