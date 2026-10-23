extends PanelContainer

@export var midi_output_button: OptionButton
@export var message_type_button: OptionButton
@export var note_controller_line_edit: LineEdit

@export var main: Main

func _ready() -> void:
	State.pin_selected.connect(_on_pin_selected)

func _on_pin_selected(pin: Pin) -> void:
	var midi_message = JSON.stringify({"type": message_type_button.text, "controller": int(note_controller_line_edit.text)})
	var midi_output =  (await Backend.get_connected_midi_outputs())[midi_output_button.selected]["id"]
	Backend.subscribe_midi_output_to_pin(main.get_current_graph().graph_id, pin.get_parent().node_id, pin.index, midi_output.json_escape(), midi_message)
	%SelectingPinPopup.hide()


func _on_button_pressed() -> void:
	State.current_state = State.SELECTING_PIN
	main._on_edit_button_pressed()
	%SelectingPinPopup.show()
