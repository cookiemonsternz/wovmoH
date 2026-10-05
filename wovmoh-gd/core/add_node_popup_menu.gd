class_name AddNodePopupMenu extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var node_kinds = await Backend.get_node_kinds()
	for node_kind in node_kinds:
		var button = Button.new()
		button.text = node_kind
		button.pressed.connect(_on_button_pressed.bind(node_kind))
		$PanelContainer/VBoxContainer.add_child(button)

func _on_button_pressed(node_kind: String):
	print(node_kind)
	await Backend.add_node(get_parent().graph_id, node_kind, position)
	get_parent()._refresh_from_backend()
	queue_free()
