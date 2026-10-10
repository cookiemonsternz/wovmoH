extends FoldableContainer

@onready var device_container: VBoxContainer = $MarginContainer/HBoxContainer/Device
@onready var input_container: VBoxContainer = $MarginContainer/HBoxContainer/HBoxContainer/Input
@onready var output_container: VBoxContainer = $MarginContainer/HBoxContainer/HBoxContainer/Output

func _on_folding_changed(is_folded: bool) -> void:
	if !is_folded:
		var inputs: Array = await Backend.get_available_midi_inputs()
		var outputs: Array = await Backend.get_available_midi_outputs()
		
		for child in device_container.get_children().slice(2):
			if child is Label: 
				child.queue_free()
		for child in input_container.get_children():
			if child is CheckBox: 
				child.queue_free()
		for child in output_container.get_children():
			if child is CheckBox: 
				child.queue_free()
		
		for input_device in inputs:
			var output_index = outputs.find_custom(are_same_device.bind(input_device))
			if output_index != -1:
				var device_name_label = Label.new()
				device_name_label.text = input_device["name"]
				if device_name_label.text.contains("MIDI In"): device_name_label.text = device_name_label.text.substr(0, 10)
				device_container.add_child(device_name_label)
				var input_checkbox = CheckBox.new()
				input_checkbox.toggled.connect(connect_input.bind(input_device["id"]))
				input_container.add_child(input_checkbox)
				var output_checkbox = CheckBox.new()
				output_container.add_child(output_checkbox)

func are_same_device(input_device: Variant, output_device: Variant) -> bool:
	var in_name: String = input_device["name"]
	var out_name: String = output_device["name"]
	
	return in_name.similarity(out_name) > 0.75

func connect_input(toggled_on: bool, id: String,):
	if !toggled_on: return
	Backend.connect_midi_input(id.json_escape())
