#pragma once

#include "graph/node.h"
#include "graph/pin.h"
#include <string>
#include <variant> // For std::holds_alternative, std::get, std::monostate

// Constant value node
class ConstantNumberNode : public Node
{
public:
    ConstantNumberNode(std::string id, std::string name, double value = 0);
    void process() override;

    // Serialization
    json getParameters() const override;
    void setParameters(const json &params) override;
private:
    double m_value;
};

// Add node
class AddNode : public Node
{
public:
    AddNode(std::string id, std::string name);
    void process() override;
};

// Multiply node
class MultiplyNode : public Node
{
public:
    MultiplyNode(std::string id, std::string name);
    void process() override;
};

// Debug print Node
class DebugNode : public Node
{
public:
    DebugNode(std::string id, std::string name);
    void process() override;
};
