#include "corenodes.h"
#include "graph/pin.h"

#include <godot_cpp/variant/utility_functions.hpp>

// ConstantNumberNode Implementation
ConstantNumberNode::ConstantNumberNode(std::string id, std::string name, double value)
    : Node(id, name, "ConstantNumberNode"),
      m_value(value)
{
    addOutputPin(PinDataType::NUMBER, "out", "Value");
    if (auto outputPin = getOutputPin("out"))
    {
        outputPin->setValue(DataValue(m_value));
    }
}

void ConstantNumberNode::process()
{
    // no process
}

json ConstantNumberNode::getParameters() const
{
    json params;
    params["value"] = m_value;
    return params;
}

void ConstantNumberNode::setParameters(const json &params)
{
    if (params.contains("value"))
    {
        m_value = params["value"];
        if (auto outputPin = getOutputPin("out"))
        {
            outputPin->setValue(DataValue(m_value));
        }
    }
}

// AddNode Implementation
AddNode::AddNode(std::string id, std::string name)
    : Node(id, name, "AddNode")
{
    addInputPin(PinDataType::NUMBER, "in1", "Input A", 0.0);
    addInputPin(PinDataType::NUMBER, "in2", "Input B", 0.0);
    addOutputPin(PinDataType::NUMBER, "out", "Result");
}

void AddNode::process()
{
    // Get input pins
    InputPin *inputPinA = getInputPin("in1");
    InputPin *inputPinB = getInputPin("in2");
    OutputPin *outputPin = getOutputPin("out");

    if (!outputPin) { // Early exit if output pin is missing
        // Optionally log an error here
        return;
    }

    if (inputPinA && inputPinB)
    {
        if (inputPinA->isConnected() && inputPinB->isConnected())
        {
            DataValue valueA = inputPinA->getValue();
            DataValue valueB = inputPinB->getValue();
            // Check type, datavalue should be Number
            if (std::holds_alternative<double>(valueA) && std::holds_alternative<double>(valueB))
            {
                double num1 = std::get<double>(valueA);
                double num2 = std::get<double>(valueB);
                double result = num1 + num2;
                outputPin->setValue(DataValue(result));
            }
            else
            { // Handle error: type mismatch
                outputPin->setValue(std::monostate());
            }
        }
        else
        { // Handle error: not all inputs are connected
            outputPin->setValue(std::monostate());
        }
    }
    else
    { // Handle error: pin not found
        outputPin->setValue(std::monostate());
    }
}

// MultiplyNode Implementation
MultiplyNode::MultiplyNode(std::string id, std::string name)
    : Node(id, name, "MultiplyNode")
{
    addInputPin(PinDataType::NUMBER, "in1", "Input A", 0.0);
    addInputPin(PinDataType::NUMBER, "in2", "Input B", 0.0);
    addOutputPin(PinDataType::NUMBER, "out", "Result");
}

void MultiplyNode::process()
{
    // Get input pins
    InputPin *inputPinA = getInputPin("in1");
    InputPin *inputPinB = getInputPin("in2");
    OutputPin *outputPin = getOutputPin("out");

    if (!outputPin) { // Early exit if output pin is missing
        // Optionally log an error here
        return;
    }

    if (inputPinA && inputPinB)
    {
        if (inputPinA->isConnected() && inputPinB->isConnected())
        {
            DataValue valueA = inputPinA->getValue();
            DataValue valueB = inputPinB->getValue();
            // Check type, datavalue should be Number
            if (std::holds_alternative<double>(valueA) && std::holds_alternative<double>(valueB))
            {
                double num1 = std::get<double>(valueA);
                double num2 = std::get<double>(valueB);
                double result = num1 * num2;
                outputPin->setValue(DataValue(result));
            }
            else
            { // Handle error: type mismatch
                outputPin->setValue(std::monostate());
            }
        }
        else
        { // Handle error: not all inputs are connected
            outputPin->setValue(std::monostate());
        }
    }
    else
    { // Handle error: pin not found
        outputPin->setValue(std::monostate());
    }
}

// DebugNode Implementation
DebugNode::DebugNode(std::string id, std::string name)
    : Node(id, name, "DebugNode")
{
    addInputPin(PinDataType::NUMBER, "inNum", "Number Input", 0.0);
    // addInputPin(PinDataType::BOOLEAN, "inBool", "Bool Input");
    // addInputPin(PinDataType::COLOR, "inColor", "Color Input");
    // addInputPin(PinDataType::VECTOR, "inVec", "Vector Input");
}

void DebugNode::process()
{
    // Get input pin
    InputPin *inputPinNum = getInputPin("inNum");
    if (inputPinNum)
    {
        if (inputPinNum->isConnected())
        {
            DataValue value = inputPinNum->getValue();
            if (std::holds_alternative<double>(value))
            {
                godot::UtilityFunctions::print("DebugNode: Number Input:", std::get<double>(value));
            }
            else
            {
                godot::UtilityFunctions::print("DebugNode: Number Input has incorrect data type or is monostate.");
            }
        }
        else
        {
            godot::UtilityFunctions::print("DebugNode: Number Input not connected");
        }
    }
    else
    {
        godot::UtilityFunctions::print("DebugNode: Input pin 'inNum' not found.");
    }
}