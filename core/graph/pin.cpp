#include "pin.h"

/********************************************************/
/*                         BASE PIN                     */
/********************************************************/
BasePin::BasePin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name)
    : m_parentNode(parentNode),
      m_pinDataType(pinType),
      m_id(id),
      m_name(name)
{
    // Pass
}

// GETTERS
// Get pin data type
PinDataType BasePin::getPinDataType() const
{
    return m_pinDataType;
}
// Get pin Id
const std::string &BasePin::getId() const
{
    return m_id;
}
// Get pin name
const std::string &BasePin::getName() const
{
    return m_name;
}
// Get pin parent node
Node *BasePin::getParentNode() const
{
    return m_parentNode;
}
// Get pin datatype as a string
std::string BasePin::getDataTypeString() const
{
    switch (m_pinDataType)
    {
    case PinDataType::INVALID:
        return std::string("Invalid");
        break;
    case PinDataType::NUMBER:
        return std::string("Number");
        break;
    case PinDataType::BOOLEAN:
        return std::string("Boolean");
        break;
    case PinDataType::COLOR:
        return std::string("Color");
        break;
    case PinDataType::VECTOR:
        return std::string("Vector");
        break;
    default:
        return std::string("Unknown");
        break;
    }
}

/********************************************************/
/*                         INPUT PIN                    */
/********************************************************/
InputPin::InputPin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name, DataValue defaultValue)
    : BasePin(parentNode, pinType, id, name)
{
    m_value = defaultValue; // Set default value
    // m_connectedOutputPin already nullptr bc default initializer
}
// Get if connected
bool InputPin::isConnected() const
{
    if (m_connectedOutputPin == nullptr)
    {
        return false;
    }
    else
    {
        return true;
    }
}
// Get connected output
OutputPin *InputPin::getConnectedOutputPin() const
{
    return m_connectedOutputPin;
}
// Get Value - Value only functions when not connected to an output
DataValue InputPin::getValue() const
{
    if (m_connectedOutputPin == nullptr)
    {
        return m_value;
    }
    else
    {
        return m_connectedOutputPin->getValue();
    }
}
// Set Value - Value only functions when not connected to an output
void InputPin::setValue(const DataValue &value)
{
    if (m_connectedOutputPin == nullptr)
    {
        m_value = value;
    }
}
// Connect to output node
void InputPin::connect(OutputPin *outputPin)
{
    m_connectedOutputPin = outputPin;
}
// Disconnect from output node
void InputPin::disconnect()
{
    m_connectedOutputPin = nullptr;
}

/********************************************************/
/*                         OUTPUT PIN                   */
/********************************************************/
OutputPin::OutputPin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name)
    : BasePin(parentNode, pinType, id, name)
{
    // m_connectedOutputPin already nullptr bc default initializer
}
// Get Value
const DataValue &OutputPin::getValue() const
{
    return m_value;
}
// Set Value
void OutputPin::setValue(const DataValue &value)
{
    m_value = value;
}
// Get connected Inputs
std::vector<InputPin *> OutputPin::getConnectedInputPins() const
{
    return m_connectedInputPins;
};
// Connect to input
void OutputPin::connect(InputPin *inputPin)
{
    inputPin->connect(this);
    m_connectedInputPins.push_back(inputPin);
};
// Disconnect from all input
void OutputPin::disconnect()
{
    for (InputPin *inputPin : m_connectedInputPins)
    {
        inputPin->disconnect();
    }
    m_connectedInputPins.clear();
};
// Disconnect from specific input
void OutputPin::disconnect(InputPin *inputPin)
{
    return; // remove this when fully implemented
    for (InputPin *inPin : m_connectedInputPins)
    {
        if (inputPin == inPin)
        {
            inputPin->disconnect();
            // TODO - Erase from m_connectedInputPins
        }
    }
}