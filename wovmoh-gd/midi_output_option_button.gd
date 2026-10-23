extends OptionButton


func _pressed() -> void:
	self.clear()
	
	var outputs = await Backend.get_connected_midi_outputs()
	for output in outputs:
		self.add_item(output["name"])
