class_name ArtnetInputSubscriptionTableElement extends MarginContainer

var subscription_data: Variant

func setup(data: Variant):
	subscription_data = data
	%Channel.text = str(data["channel"])
	
	%Field.text = "Input: %s | Node: %s" % [str(data["field_index"]), str(data["node_id"])]


func _on_button_pressed() -> void:
	pass
	#try_delete.emit(subscription_data["graph_id"], subscription_data["node_id"], subscription_data["field_index"], subscription_data["message"])
	#subscription_data["message"]["controller"] = int(subscription_data["message"]["controller"])
	Backend.unsubscribe_artnet_input_from_field(int(subscription_data["graph_id"]), int(subscription_data["node_id"]), int(subscription_data["field_index"]), int(subscription_data["channel"]))
	get_parent()._on_midi_config_visibility_changed()
