#pragma once

#include "../core/graph/patchgraph.h"
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/utility_functions.hpp>

/*
Handles transferring data between cpp backend and godot with a json format.
Also exposes methods for directly editing graph state, so json commands are not
required from godot.
*/

namespace godot {
	class GraphBridge : public Node {
		GDCLASS(GraphBridge, Node)
	private:
		std::unique_ptr<PatchGraph> m_graph;

	protected:
		static void _bind_methods();

	public:
		GraphBridge();
		~GraphBridge();

        String export_graph_json() const;
        void import_graph_json(const String& graph_json);

        // Node operations
        void add_node(const String& id, const String& name, const String& type);
        void remove_node(const String& id);
        String get_node_data(const String& id);
        void update_node_parameters(const String& id, const String& json);
        void update_node_properties(const String& id, const String& json);

        // Connectiongs
        void connect(const std::string &sourceNodeId, const std::string &outputPinId, const std::string &destNodeId, const std::string &inputPinId);
        void disconnect(const std::string &destNodeId, const std::string &inputPinId);

        // processing
		void _process(double delta) override;

        void test_load();
	};
}