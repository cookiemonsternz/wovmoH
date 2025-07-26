#pragma once

#include "types/datatypes.h"
#include <string>
#include <vector>

class Node;
class InputPin;
class OutputPin;

enum class PinDirection
{
    INPUT,
    OUTPUT
};

/********************************************************/
/*                         BASE PIN                     */
/********************************************************/
class BasePin
{
public:
    BasePin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name);
    virtual ~BasePin() = default;

    // Getters
    Node *getParentNode() const;
    PinDataType getPinDataType() const;
    const std::string &getId() const;
    const std::string &getName() const;

    virtual PinDirection getDirection() const = 0; // Pure virtual function - must be implemented by derived classes and means that this class can't be instantiated

    std::string getDataTypeString() const;
protected:
    Node *m_parentNode;
    PinDataType m_pinDataType;
    std::string m_id;   // unique within node
    std::string m_name; // Display name
};

/********************************************************/
/*                         INPUT PIN                    */
/********************************************************/
class InputPin : public BasePin
{
public:
    InputPin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name, DataValue defaultValue);

    // Getters
    PinDirection getDirection() const override { return PinDirection::INPUT; }

    // Value
    DataValue getValue() const;
    void setValue(const DataValue &value); // only if not connected

    // Connections
    bool isConnected() const;
    OutputPin *getConnectedOutputPin() const;

    void connect(OutputPin *outputPin);
    void disconnect();
private:
    OutputPin *m_connectedOutputPin = nullptr;
    DataValue m_value;
};

/********************************************************/
/*                         OUTPUT PIN                   */
/********************************************************/
class OutputPin : public BasePin
{
public:
    OutputPin(Node *parentNode, PinDataType pinType, const std::string &id, const std::string &name);

    // Getters
    PinDirection getDirection() const override { return PinDirection::OUTPUT; }

    // Value
    const DataValue &getValue() const;
    void setValue(const DataValue &value);

    // Connections
    void connect(InputPin *inputPin);
    void disconnect();
    void disconnect(InputPin *inputPin); // NOT IMPLEMENTED

    std::vector<InputPin *> getConnectedInputPins() const;
private:
    DataValue m_value;
    std::vector<InputPin *> m_connectedInputPins;
};
