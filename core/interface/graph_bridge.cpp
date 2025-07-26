#include "graph_bridge.h"
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/classes/file_access.hpp>

using namespace godot;

void GraphBridge::_bind_methods() {
    // Bind your methods to make them callable from GDScript
    ClassDB::bind_method(D_METHOD("export_graph_json"), &GraphBridge::export_graph_json);
    ClassDB::bind_method(D_METHOD("import_graph_json", "graph_json"), &GraphBridge::import_graph_json);
    ClassDB::bind_method(D_METHOD("add_node", "id", "name", "type"), &GraphBridge::add_node);
    ClassDB::bind_method(D_METHOD("remove_node", "id"), &GraphBridge::remove_node);
    ClassDB::bind_method(D_METHOD("get_node_data", "id"), &GraphBridge::get_node_data);
    ClassDB::bind_method(D_METHOD("update_node_parameters", "id", "json"), &GraphBridge::update_node_parameters);
    ClassDB::bind_method(D_METHOD("update_node_properties", "id", "json"), &GraphBridge::update_node_properties);
    ClassDB::bind_method(D_METHOD("test_load"), &GraphBridge::test_load);
    // Note: connect/disconnect methods use std::string, so they won't be bindable to GDScript directly
}

GraphBridge::GraphBridge() {
    m_graph = std::make_unique<PatchGraph>();
}

GraphBridge::~GraphBridge() {
}

// You'll also need to implement all the other methods declared in your header:
String GraphBridge::export_graph_json() const {
    json data = m_graph->toJson();
    return String(data.dump().c_str());
}

void GraphBridge::import_graph_json(const String& graph_json) {
    m_graph->fromJson(graph_json.utf8().get_data());
}

void GraphBridge::add_node(const String& id, const String& name, const String& type) {
    
}

void GraphBridge::remove_node(const String& id) {
}

String GraphBridge::get_node_data(const String& id) {
    return String("Hello this is backend");
}

void GraphBridge::update_node_parameters(const String& id, const String& json) {
}

void GraphBridge::update_node_properties(const String& id, const String& json) {
}

void GraphBridge::connect(const std::string &sourceNodeId, const std::string &outputPinId, const std::string &destNodeId, const std::string &inputPinId) {
}

void GraphBridge::disconnect(const std::string &destNodeId, const std::string &inputPinId) {
}

void GraphBridge::_process(double delta) {
}

void GraphBridge::test_load() {
    Ref<FileAccess> file = FileAccess::open("res://data/graph.json", FileAccess::READ);

    if (file.is_valid()) {
        String content = file->get_as_text();
        file->close();
        // UtilityFunctions::print("contents: ", content);

        json j = json::parse(content.utf8().get_data());

        m_graph->fromJson(j);

        m_graph->sort();
        m_graph->evaluate();

        UtilityFunctions::print("Successfully read and executed");

    } else {
        UtilityFunctions::print("fail file read");
    }
}