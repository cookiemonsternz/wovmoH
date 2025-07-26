#pragma once

#include "lib/json.hpp"
using json = nlohmann::json;

#include <godot_cpp/core/class_db.hpp>

namespace godot {
	class Node {
		Node(const String id, const String name, const String type);
		virtual ~Node() = default;

		const String& getId() const;
		const String& getName() const;
		const String& getType() const;

		virtual void process() = 0;

		int getInDegree() const;

		// Serialization
		virtual json getParameters() const;
		virtual void setParameters(const json &params);

		virtual json getProperties() const;
		virtual void setProperties(const json &properties);
	};
}
