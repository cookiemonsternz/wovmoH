class_name SubscriptionTableElement extends MarginContainer

func setup(data: Variant):
	%MessageType.text = data["message"]["type"]
	
	match data["message"]["type"]:
		"PitchWheel":
			%Note_Controller.text = "Pitch"
		"ControlChange":
			%Note_Controller.text = data["message"]["controller"]
		_:
			%Note_Controller.text = data["message"]["note"]
	
	%Field.text = "Input: %s | Node: %s" % [data["field_index"], data["node_id"]]
