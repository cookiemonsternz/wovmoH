class_name Pin extends Label

var index
var highlight_panel_scene = preload("res://core/value/highlight_panel_container.tscn")

func _ready() -> void:
	add_child(highlight_panel_scene.instantiate(), false, Node.INTERNAL_MODE_FRONT)
	var midi_icon = TextureRect.new()
	midi_icon.texture = load("res://addons/at-icons/node/node_graph_connection.svg")
	add_child(midi_icon, false, Node.INTERNAL_MODE_FRONT)
	midi_icon.visible = false
	midi_icon.set_anchors_preset(Control.PRESET_CENTER_RIGHT)

func setup(data: Variant, i: int):
	index = i
	text = data["name"]

func _on_mouse_entered() -> void:
	match State.current_state:
		State.NORMAL: pass
		State.SELECTING_PIN:
			get_child(0, true).show()


func _on_mouse_exited() -> void:
	match State.current_state:
		State.NORMAL: pass
		State.SELECTING_PIN:
			get_child(0, true).hide()

func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.is_pressed() and event.button_index == MouseButton.MOUSE_BUTTON_LEFT and State.current_state == State.SELECTING_PIN:
		State.pin_selected.emit(self)
		State.current_state = State.NORMAL
		get_child(0, true).hide()
		
		var node: WovmohGraphNode = get_parent()
		node.set_slot_enabled_right(node.get_output_port_slot(index), false)
		
		get_child(1, true).show()
