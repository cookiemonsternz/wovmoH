class_name Main extends Control

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	Backend.api_error.connect(_show_api_error)
	GodotxToast.default_origin = GodotxToastEnums.ToastOrigin.BOTTOM_RIGHT

func _show_api_error(message: String) -> void:
	GodotxToast.error(message)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if Input.is_action_just_pressed("save"):
		var file_dialog = FileDialog.new()
		file_dialog.access = FileDialog.ACCESS_FILESYSTEM
		add_child(file_dialog)
		file_dialog.file_selected.connect(func(path: String):
			Backend.save_file(path)
		)
		file_dialog.popup_centered_clamped()
	if Input.is_action_just_pressed("open"):
		var file_dialog = FileDialog.new()
		file_dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
		file_dialog.access = FileDialog.ACCESS_FILESYSTEM
		add_child(file_dialog)
		file_dialog.file_selected.connect(func(path: String):
			Backend.load_file(path)
		)
		file_dialog.popup_centered_clamped()

func get_current_graph() -> WovmohGraphEdit:
	return $VBoxContainer/GraphView/HSplitContainer/WovmohGraphEdit


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
