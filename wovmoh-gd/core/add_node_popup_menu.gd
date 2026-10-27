class_name AddNodePopupMenu extends PopupMenu

var kinds: Dictionary[String, String]

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var descriptors = await Backend.get_node_descriptors()
	var categories: Array[String] = [];
	for descriptor in descriptors:
		kinds[descriptor["name"]] = descriptor["kind"]
		
		if !categories.has(descriptor["category"]):
			categories.push_back(descriptor["category"])
	
	for category in categories:
		var submenu: PopupMenu = PopupMenu.new()
		add_child(submenu)
		submenu.name = category
		add_submenu_node_item(category, submenu)
		submenu.index_pressed.connect(submenu_index_pressed.bind(submenu))
	
	for descriptor in descriptors:
		var submenu: PopupMenu = get_node(descriptor["category"])
		submenu.add_item(descriptor["name"])

func submenu_index_pressed(index: int, submenu: PopupMenu):
	var item = submenu.get_item_text(index)
	_on_button_pressed(kinds[item])

func _on_button_pressed(node_kind: String):
	print(node_kind)
	await Backend.add_node(get_parent().graph_id, node_kind, position)
	get_parent()._refresh_from_backend()
	queue_free()
