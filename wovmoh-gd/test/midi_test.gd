extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	Backend.client.connected.connect(on_connected)

func on_connected():
	print("--- INPUTS ---")
	print(await Backend.get_available_midi_inputs())
	print("--- OUTPUTS ---")
	print(await Backend.get_available_midi_outputs())


func _on_connect_button_pressed() -> void:
	if await Backend.connect_midi_input(%ConnectLineEdit.text):
		print("Connection Successful")


func _on_get_connected_inputs_button_pressed() -> void:
	var inputs = await Backend.get_connected_midi_inputs()
	print("--- CONNECTED INPUTS ---")
	print(inputs)
