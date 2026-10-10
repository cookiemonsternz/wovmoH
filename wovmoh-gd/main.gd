class_name Main extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func get_current_graph() -> WovmohGraphEdit:
	return $VBoxContainer/GraphView/WovmohGraphEdit


func _on_edit_button_pressed() -> void:
	$VBoxContainer/GraphView.show()
	$VBoxContainer/ManageMenu.hide()
	
	#$VBoxContainer/TopBar/CenterItems/ManageButton.disabled = true
	#$VBoxContainer/TopBar/CenterItems/EditButton.disabled = false
	#$VBoxContainer/TopBar/CenterItems/PerformButton.disabled = true

func _on_manage_button_pressed() -> void:
	$VBoxContainer/GraphView.hide()
	$VBoxContainer/ManageMenu.show()
	
	#$VBoxContainer/TopBar/CenterItems/ManageButton.disabled = false
	#$VBoxContainer/TopBar/CenterItems/EditButton.disabled = true
	#$VBoxContainer/TopBar/CenterItems/PerformButton.disabled = true


func _on_perform_button_pressed() -> void:
	pass # Replace with function body.
