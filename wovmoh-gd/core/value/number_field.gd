@tool
class_name NumberField extends PanelContainer

@export var field_name: String = ""
@export var min_value: float = 0.0
@export var max_value: float = 1.0
@export var value: float = 0.0:
	set(new):
		value = clamp(new, min_value, max_value)
		_update_value()
@export var absolute_drag: bool = true
@export var drag_scale: float = 1.0
@export var value_format: String = "%.2f"
@export var value_unit: String = ""

var dragging = false
var index: int
var connected: bool = false:
	set(new):
		connected = new
		_update_connected()

func _update_connected():
	if connected:
		%RightHBoxContainer.hide()
		%ProgressBar.hide()
	else:
		%RightHBoxContainer.show()
		%ProgressBar.show()

func _update_value():
	if not is_node_ready(): return
	%ProgressBar.value = remap(value, min_value, max_value, 0, 1) if min_value != -INF and max_value != INF else 0
	%FieldValue.text = value_format % value
	%FieldUnit.text = value_unit
	%LineEdit.text = value_format % value + value_unit
	
	%FieldName.text = field_name
	
	Backend.set_node_field_value(get_parent().get_parent().get_parent().graph_id, get_parent().get_parent().node_id, index, Data.get_data_type_from_field_type(Data.NUMBER), value)

func _try_parse(new: String) -> float:
	var regex = RegEx.new()
	regex.compile("[^.\\d]+") # Matches any characters that aren't digits or a period (.)
	var number_string = regex.sub(new, "", true)
	print(number_string)
	if (number_string.is_valid_float()):
		return float(number_string)
	else:
		return value
	
func _ready() -> void:
	_update_value()

func _show_lineedit():
	$Main.hide()
	$Lineedit.show()
	%LineEdit.grab_focus()
	%LineEdit.edit()

func _hide_lineedit():
	$Lineedit.hide()
	$Main.show()

func _on_line_edit_text_submitted(new_text: String) -> void:
	value = _try_parse(new_text)
	_hide_lineedit()

func _begin_drag():
	#print("Begin Drag")
	dragging = true
	get_parent().get_parent().draggable = false

func _end_drag():
	#print("End Drag")
	dragging = false
	get_parent().get_parent().draggable = true

func _drag(screen_relative: Vector2, position: Vector2):
	if absolute_drag:
		value = remap(position.x, 0, size.x, min_value, max_value)
	else:
		value += screen_relative.x * drag_scale * 0.01

func _on_gui_input(event: InputEvent) -> void:
	#if event.is_action_pressed("select"):
		#print("SELECTR")
	if event.is_action_released("select"):
		if dragging: _end_drag()
		else: _show_lineedit()
	if event is InputEventMouseMotion:
		if event.button_mask == MouseButtonMask.MOUSE_BUTTON_MASK_LEFT:
			if !dragging: _begin_drag()
			else: _drag(event.screen_relative, event.position)
