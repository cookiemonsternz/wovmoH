extends Control

@export var graph_id: int = 0

func _ready() -> void:
	Backend.client.connected.connect(_on_connected)

func _on_connected() -> void:
	var data = await Backend.get_graph(graph_id)
	for node_data in data["nodes"]:
		_add_node(node_data)

func _add_node(node_data: Variant):
	var node = WovmohGraphNode.new()
	node.name = str(node_data.id)
	node.kind = node_data["kind"]
	node.kind_title = node_data["name"]
	node.title_override = node_data["name_override"]
	node.position_offset = Vector2(node_data["position"][0], node_data["position"][1])
	for input_data in node_data["inputs"]:
		node.add_input(input_data)
	for output_data in node_data["outputs"]:
		node.add_output(output_data)
	
	$GraphEdit.add_child(node)
