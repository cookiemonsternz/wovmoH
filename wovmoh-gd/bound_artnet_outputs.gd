extends VBoxContainer

var element_scene = preload("res://artnet_output_subscription_table_element.tscn")

func _on_midi_config_visibility_changed() -> void:
	if not Backend.is_connected_to_host(): return
	
	for child in get_children():
		child.queue_free()
	
	var subscriptions = await Backend.get_artnet_output_subscriptions()
	
	for subscription in subscriptions:
		var element: ArtnetOutputSubscriptionTableElement = element_scene.instantiate()
		element.setup(subscription)
		add_child(element)
