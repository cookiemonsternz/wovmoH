class_name WovmohGraphEdit extends GraphEdit

@export var graph_id: int = 0

var node_scene = preload("res://core/wovmoh_graph_node.tscn")

func _ready() -> void:
	Backend.client.connected.connect(_on_connected)
	connection_request.connect(_on_connection_request)

func _on_connected() -> void:
	var data = await Backend.get_graph(graph_id)
	for node_data in data["nodes"]:
		_add_node(node_data)

func _add_node(node_data: Variant):
	var node: WovmohGraphNode = node_scene.instantiate()
	node.node_id = int(node_data["id"])
	node.name = str(node_data.id)
	node.kind = node_data["kind"]
	node.kind_title = node_data["name"]
	node.title_override = node_data["name_override"]
	node.position_offset = Vector2(node_data["position"][0], node_data["position"][1])
	for output_data in node_data["outputs"]:
		node.add_output(output_data)
	
	var i = 0
	for input_data in node_data["inputs"]:
		node.add_input(input_data, i)
		i += 1
	
	add_child(node)

func _on_connection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int):
	var node_from: WovmohGraphNode = get_node(str(from_node))
	var node_to: WovmohGraphNode = get_node(str(to_node))
	var slot_to: int = node_to.get_input_port_slot(to_port)
	var field_to = node_to.get_child(slot_to).get_child(0)
	field_to.connected = true
	Backend.connect_nodes(graph_id, node_from.node_id, from_port, node_to.node_id, to_port)
	connect_node(from_node, from_port, to_node, to_port)
