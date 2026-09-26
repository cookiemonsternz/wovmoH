@tool
class_name Field extends PanelContainer

@export var min_value: float = 0.0
@export var max_value: float = 1.0
@export var step: float = 0.01
@export var value: float = 0.0:
	set(new):
		value = new
		_update_value()

func _update_value():
	if not $ProgressBar: return
	$ProgressBar.value = remap(value, min_value, max_value, 0, 1)


func _on_mouse_entered() -> void:
	$LeftHBoxContainer/LeftButton.show()
	$RightHBoxContainer/RightButton.show()

func _on_mouse_exited() -> void:
	$LeftHBoxContainer/LeftButton.hide()
	$RightHBoxContainer/RightButton.hide()
