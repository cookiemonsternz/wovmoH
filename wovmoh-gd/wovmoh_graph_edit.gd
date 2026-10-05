class_name WovmohGraphEdit extends GraphEdit

@export var graph_id: int = 0

var node_scene = preload("res://core/wovmoh_graph_node.tscn")
var add_node_popup_menu_scene = preload("res://core/add_node_popup_menu.tscn")

var nodes: Dictionary[int, WovmohGraphNode]
var copy_buffer = []

func _ready() -> void:
	Backend.client.connected.connect(_on_connected)
	connection_request.connect(_on_connection_request)
	disconnection_request.connect(_on_disconnection_request)
	popup_request.connect(_on_popup_request)
	delete_nodes_request.connect(_on_delete_nodes_request)
	copy_nodes_request.connect(_on_copy_nodes_request)
	paste_nodes_request.connect(_on_paste_nodes_request)

func _on_connected() -> void:
	var data = await Backend.get_graph(graph_id)
	print(data)
	for node_data in data["nodes"]:
		_add_node(node_data)
	for connection_data in data["connections"]:
		var from_node = nodes[connection_data["from_node"]]
		var to_node = nodes[connection_data["to_node"]]
		var to_slot: int = to_node.get_input_port_slot(connection_data["to_index"])
		var to_field = to_node.get_child(to_slot).get_child(0)
		to_field.connected = true
		connect_node(from_node.name, int(connection_data["from_index"]), to_node.name, int(connection_data["to_index"]))

func _refresh_from_backend():
	for child in get_children():
		if child is WovmohGraphNode:
			child.queue_free()
	for connection in connections:
		disconnect_node(connection.from_node, connection.from_port, connection.to_node, connection.to_port)
	
	var data = await Backend.get_graph(graph_id)
	#print(data)
	for node_data in data["nodes"]:
		_add_node(node_data)
	for connection_data in data["connections"]:
		var from_node = nodes[connection_data["from_node"]]
		var to_node = nodes[connection_data["to_node"]]
		var to_slot: int = to_node.get_input_port_slot(connection_data["to_index"])
		var to_field = to_node.get_child(to_slot).get_child(0)
		to_field.connected = true
		connect_node(from_node.name, int(connection_data["from_index"]), to_node.name, int(connection_data["to_index"]))

func _add_node(node_data: Variant) -> WovmohGraphNode:
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
	
	nodes[node.node_id] = node
	
	return node

func _on_connection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int):
	var node_from: WovmohGraphNode = get_node(str(from_node))
	var node_to: WovmohGraphNode = get_node(str(to_node))
	var slot_to: int = node_to.get_input_port_slot(to_port)
	var field_to = node_to.get_child(slot_to).get_child(0)
	field_to.connected = true
	Backend.connect_nodes(graph_id, node_from.node_id, from_port, node_to.node_id, to_port)
	connect_node(from_node, from_port, to_node, to_port)

func _on_disconnection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int):
	print("DISCONNECTING")
	var node_from: WovmohGraphNode = get_node(str(from_node))
	var node_to: WovmohGraphNode = get_node(str(to_node))
	var slot_to: int = node_to.get_input_port_slot(to_port)
	var field_to = node_to.get_child(slot_to).get_child(0)
	field_to.connected = false
	Backend.disconnect_nodes(graph_id, node_from.node_id, from_port, node_to.node_id, to_port)
	disconnect_node(from_node, from_port, to_node, to_port)

func _on_popup_request(pos: Vector2):
	for child in get_children():
		if child is AddNodePopupMenu:
			child.queue_free()
	var popup_scene: Control = add_node_popup_menu_scene.instantiate()
	popup_scene.position = pos
	add_child(popup_scene)

func _on_delete_nodes_request(nodes: Array[StringName]):
	for node in nodes:
		print(get_node(str(node)).node_id, " : ", get_node(str(node)).kind)
		Backend.delete_node(graph_id, get_node(str(node)).node_id)
	_refresh_from_backend()

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.is_pressed() and event.button_index == MouseButton.MOUSE_BUTTON_LEFT:
		for child in get_children():
			if child is AddNodePopupMenu:
				child.queue_free()

func _get_selected_nodes() -> Array:
	var selection = []
	for child in get_children():
		if child is WovmohGraphNode and child.selected:
			selection.append(child)
	return selection

func _on_copy_nodes_request() -> void:
	copy_buffer = _get_selected_nodes()

func _on_cut_nodes_request() -> void:
	copy_buffer = _get_selected_nodes()

func _on_paste_nodes_request() -> void:
	for node: WovmohGraphNode in copy_buffer:
		Backend.add_node(graph_id, node.kind, node.position_offset + get_local_mouse_position())
	_refresh_from_backend()
