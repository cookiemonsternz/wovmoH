extends PanelContainer

@export var channel_line_edit: LineEdit

@export var main: Main

func _ready() -> void:
	State.artnet_field_selected.connect(_on_field_selected)

func _on_field_selected(field: Field) -> void:
	#var midi_message = JSON.stringify({"type": message_type_button.text, "controller": int(note_controller_line_edit.text)})
	Backend.subscribe_artnet_input_to_field(main.get_current_graph().graph_id, field.get_parent().node_id, field.get_child(0).index, int(channel_line_edit.text))
	%SelectingFieldPopup.hide()


func _on_button_pressed() -> void:
	State.current_state = State.ARTNET_SELECTING_FIELD
	main._on_edit_button_pressed()
	%SelectingFieldPopup.show()
