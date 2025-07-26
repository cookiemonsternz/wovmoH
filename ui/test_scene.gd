extends Node2D

@export var graph_bridge: GraphBridge

@export var graph: WovmohGraphEdit

func _ready():
	graph_bridge.test_load()
	
	var json_string = graph_bridge.export_graph_json()
	var json = JSON.parse_string(json_string)
	print(json)
	
	graph.instantiate(json)
