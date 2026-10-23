@tool
class_name WovmohGraphNode extends GraphNode

@export var kind: String = ""
@export var kind_title: String = "":
	set(new):
		kind_title = new
		_update_title()
@export var title_override: String = "":
	set(new):
		title_override = new
		_update_title()

var field_scene = preload("res://core/value/field.tscn")
var pin_scene = preload("res://core/pin.tscn")

var node_id: int
var num_slots = 0

func _update_title():
	if title_override != "":
		title = " " + title_override
	else:
		title = " " + kind_title

func add_input(data: Variant, index: int):
	var field: Field = field_scene.instantiate()
	field.setup(data, index)
	add_child(field)
	
	var field_data_type = Data.get_field_type_from_data(data)
	set_slot(num_slots, true, field_data_type, Data.PORT_COLORS[field_data_type], false, 0, Color.BLACK)
	num_slots += 1

func add_output(data: Variant, index: int):
	var pin: Pin = pin_scene.instantiate()
	pin.setup(data, index)
	add_child(pin)
	
	var pin_data_type = Data.get_field_type_from_data(data)
	set_slot(num_slots, false, 0, Color.BLACK, true, pin_data_type, Data.PORT_COLORS[pin_data_type])
	num_slots += 1


func _on_dragged(from: Vector2, to: Vector2) -> void:
	var graph_id = get_parent().graph_id
	Backend.set_node_position(graph_id, node_id, to)
