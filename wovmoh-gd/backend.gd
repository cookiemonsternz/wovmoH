extends Node

var client: TcpClient

var _request_id: int = 0

func _ready() -> void:
	client = TcpClient.new()
	add_child(client)
	
	client.connect_to_host("127.0.0.1", 7878)

func is_connected_to_host() -> bool:
	if !client: return false
	if !client.is_connected_to_host(): return false
	return true

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("ui_right"):
		print(await add_example_graph(0))
	if Input.is_action_just_pressed("ui_left"):
		print(await add_graph())
	if Input.is_action_just_pressed("ui_up"):
		print(await get_graph(0))

func _try_get_response(id: int) -> Variant:
	while true:
		var response = await client.response
		if int(response["id"]) == id:
			return response
	return

func _parse_response(response: Variant) -> Variant:
	if not response["data"].has("Ok"): 
		push_error("Response error: ", response)
		return null
	else: return response["data"]["Ok"]

func _send(message: String):
	client.send((message % _request_id).to_utf8_buffer())
	_request_id += 1
	
	return _parse_response(await _try_get_response(_request_id - 1))

func poll() -> bool:
	var response = await _send('{"id":%s,"command":{"type":"Poll"}}')
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func add_example_graph(id: int) -> bool:
	var response = await _send('{"id":%s,"command":{"type":"AddExampleGraph", "id":' + str(id) + '}}')
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func add_graph() -> int:
	var response = await _send('{"id":%s,"command":{"type":"AddGraph"}}')
	if not response or response["type"] != "GraphCreated":
		return -1
	
	return int(response["id"])

func get_graph(id: int) -> Variant:
	var response = await _send('{"id":%s,"command":{"type":"GetGraph", "id":' + str(id) + '}}')
	if not response or response["type"] != "GraphData":
		return null
	
	return response["graph"]

func get_node_descriptors() -> Array:
	var response = await _send('{"id":%s,"command":{"type":"GetNodeDescriptors"}}')
	if not response or response["type"] != "NodeDescriptors":
		return []
	
	return response["descriptors"]

func set_node_field_value(graph_id: int, node_id: int, field_index: int, type: String, value: Variant) -> bool:
	var command = '"command":{"type":"SetInputFieldValue", "graph_id":%s, "node_id":%s, "field_index":%s,  "value":{"type":"%s", "value":%s}}' % [graph_id, node_id, field_index, type, value]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func connect_nodes(graph_id: int, node_from: int, pin_from: int, node_to: int, field_to: int) -> bool:
	var command = '"command":{"type":"ConnectNodes", "graph_id":%s, "node_from":%s, "pin_from":%s, "node_to":%s, "field_to":%s}' % [graph_id, node_from, pin_from, node_to, field_to]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func disconnect_nodes(graph_id: int, node_from: int, pin_from: int, node_to: int, field_to: int) -> bool:
	var command = '"command":{"type":"DisconnectNodes", "graph_id":%s, "node_from":%s, "pin_from":%s, "node_to":%s, "field_to":%s}' % [graph_id, node_from, pin_from, node_to, field_to]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func add_node(graph_id: int, kind: String, position: Vector2) -> bool:
	var command = '"command":{"type":"AddNode", "graph_id":%s, "kind":"%s", "position": %s}' % [graph_id, kind, [position.x, position.y]]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func set_node_position(graph_id: int, node_id: int, position: Vector2) -> bool:
	var command = '"command":{"type":"SetNodePosition", "graph_id":%s, "node_id":%s, "position": %s}' % [graph_id, node_id, [position.x, position.y]]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func delete_node(graph_id: int, node_id: int) -> bool:
	var command = '"command":{"type":"DeleteNode", "graph_id":%s, "node_id":%s}' % [graph_id, node_id]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_available_midi_inputs() -> Variant:
	var response = await _send('{"id":%s,"command":{"type":"GetAvailableMidiInputs"}}')
	if not response or response["type"] != "AvailableMidiInputs":
		return []
	
	return response["inputs"]

func get_available_midi_outputs() -> Variant:
	var response = await _send('{"id":%s,"command":{"type":"GetAvailableMidiOutputs"}}')
	if not response or response["type"] != "AvailableMidiOutputs":
		return []
	
	return response["outputs"]

func connect_midi_input(id: String) -> bool:
	var command = '"command":{"type":"ConnectMidiInput", "id":"%s"}' % id
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func connect_midi_output(id: String) -> bool:
	var command = '"command":{"type":"ConnectMidiOutput", "id":"%s"}' % id
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_connected_midi_inputs() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetConnectedMidiInputs"}}')
	if not response or response["type"] != "ConnectedMidiInputs":
		return []
	
	return response["inputs"]

func get_connected_midi_outputs() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetConnectedMidiOutputs"}}')
	if not response or response["type"] != "ConnectedMidiOutputs":
		return []
	
	return response["outputs"]

func subscribe_midi_input_to_field(graph_id: int, node_id: int, field_index: int, message: Variant) -> bool:
	var command = '"command":{"type":"SubscribeMidiInputToField", "graph_id":%s, "node_id":%s, "field_index":%s, "message":%s}' % [graph_id, node_id, field_index, message]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_midi_input_subscriptions() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetMidiInputSubscriptions"}}')
	if not response or response["type"] != "MidiInputSubscriptions":
		return []
	
	return response["subscriptions"]

func unsubscribe_midi_input_from_field(graph_id: int, node_id: int, field_index: int, message: Variant):
	var command = '"command":{"type":"UnsubscribeMidiInputFromField", "graph_id":%s, "node_id":%s, "field_index":%s, "message":%s}' % [graph_id, node_id, field_index, message]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func subscribe_midi_output_to_pin(graph_id: int, node_id: int, pin_index: int, midi_output: String, message: Variant) -> bool:
	var command = '"command":{"type":"SubscribeMidiOutputToPin", "graph_id":%s, "node_id":%s, "pin_index":%s, "midi_output": "%s", "message":%s}' % [graph_id, node_id, pin_index, midi_output, message]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_midi_output_subscriptions() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetMidiOutputSubscriptions"}}')
	if not response or response["type"] != "MidiOutputSubscriptions":
		return []
	
	return response["subscriptions"]

func unsubscribe_midi_output_from_pin(graph_id: int, node_id: int, pin_index: int, midi_output: String, message: Variant) -> bool:
	var command = '"command":{"type":"UnsubscribeMidiOutputFromPin", "graph_id":%s, "node_id":%s, "pin_index":%s, "midi_output": "%s", "message":%s}' % [graph_id, node_id, pin_index, midi_output, message]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_midi_input_name_from_id(id: String) -> String:
	var command = '"command":{"type":"GetMidiInputNameFromId", "id":"%s"}' % [id]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "MidiInputName":
		return ""
	
	return response["name"]

func get_midi_output_name_from_id(id: String) -> String:
	var command = '"command":{"type":"GetMidiOutputNameFromId", "id":"%s"}' % [id]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "MidiOutputName":
		return ""
	
	return response["name"]

func get_artnet_input_subscriptions() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetArtnetInputSubscriptions"}}')
	if not response or response["type"] != "ArtnetInputSubscriptions":
		return []
	
	return response["subscriptions"]

func subscribe_artnet_input_to_field(graph_id: int, node_id: int, field_index: int, channel: int) -> bool:
	var command = '"command":{"type":"SubscribeArtnetInputToField", "graph_id":%s, "node_id":%s, "field_index":%s, "channel":%s}' % [graph_id, node_id, field_index, channel]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func unsubscribe_artnet_input_from_field(graph_id: int, node_id: int, field_index: int, channel: int):
	var command = '"command":{"type":"UnsubscribeArtnetInputFromField", "graph_id":%s, "node_id":%s, "field_index":%s, "channel":%s}' % [graph_id, node_id, field_index, channel]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func get_artnet_output_subscriptions() -> Array[Variant]:
	var response = await _send('{"id":%s,"command":{"type":"GetArtnetOutputSubscriptions"}}')
	if not response or response["type"] != "ArtnetOutputSubscriptions":
		return []
	
	return response["subscriptions"]

func subscribe_artnet_output_to_pin(graph_id: int, node_id: int, pin_index: int, channel: int) -> bool:
	var command = '"command":{"type":"SubscribeArtnetOutputToPin", "graph_id":%s, "node_id":%s, "pin_index":%s, "channel":%s}' % [graph_id, node_id, pin_index, channel]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true

func unsubscribe_artnet_output_from_pin(graph_id: int, node_id: int, pin_index: int, channel: int):
	var command = '"command":{"type":"UnsubscribeArtnetOutputFromPin", "graph_id":%s, "node_id":%s, "pin_index":%s, "channel":%s}' % [graph_id, node_id, pin_index, channel]
	var response = await _send('{"id":%s,' + command + "}")
	if not response or response["type"] != "Acknowledge":
		return false
	
	return true
