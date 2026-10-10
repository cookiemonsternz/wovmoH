class_name SubscriptionTableElement extends MarginContainer

var subscription_data: Variant

func setup(data: Variant):
	subscription_data = data
	%MessageType.text = data["message"]["type"]
	
	match data["message"]["type"]:
		"PitchWheel":
			%Note_Controller.text = "Pitch"
		"ControlChange":
			%Note_Controller.text = str(data["message"]["controller"])
		_:
			%Note_Controller.text = str(data["message"]["note"])
	
	%Field.text = "Input: %s | Node: %s" % [str(data["field_index"]), str(data["node_id"])]


func _on_button_pressed() -> void:
	#try_delete.emit(subscription_data["graph_id"], subscription_data["node_id"], subscription_data["field_index"], subscription_data["message"])
	subscription_data["message"]["controller"] = int(subscription_data["message"]["controller"])
	Backend.unsubscribe_midi_input_from_field(int(subscription_data["graph_id"]), int(subscription_data["node_id"]), int(subscription_data["field_index"]), subscription_data["message"])
	get_parent()._on_midi_config_visibility_changed()
