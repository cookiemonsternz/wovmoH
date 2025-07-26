class_name WovmohGraphEdit extends GraphEdit

var nodes = {}

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func instantiate(json):
	for node in json.nodes:
		#print(node)
		var node_object = WovmohGraphNode.new()
		add_child(node_object)
		node_object.set_node(node)
		
		nodes[node.id] = node_object
		#print("///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")
	#print(nodes)
	
	for connection in json.connections:
		print(connection)
		print(connection.sourceNodeId, nodes[connection.sourceNodeId].get_pin(connection.outputPinId), connection.destNodeId, nodes[connection.destNodeId].get_pin(connection.inputPinId))
		connect_node(connection.sourceNodeId, nodes[connection.sourceNodeId].get_pin(connection.outputPinId), connection.destNodeId, nodes[connection.destNodeId].get_pin(connection.inputPinId) - nodes[connection.destNodeId].get_output_port_count())

func _on_connection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int) -> void:
	connect_node(from_node, from_port, to_node, to_port)

func _on_disconnection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int) -> void:
	disconnect_node(from_node, from_port, to_node, to_port)
