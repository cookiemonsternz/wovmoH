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

var id: int

func _update_title():
	if title_override != "":
		title = title_override
	else:
		title = kind_title

func add_input(data: Variant):
	var input_label = Label.new()
	input_label.text = data["name"]
	add_child(input_label)

func add_output(data: Variant):
	pass
