@tool
class_name BooleanField extends PanelContainer

@export var field_name: String = ""
@export var value: bool:
	set(new):
		value = new
		_update_value()

var index: int
var connected: bool = false:
	set(new):
		connected = new
		_update_connected()

func _update_connected():
	if connected:
		%FieldCheckButton.hide()
	else:
		%FieldCheckButton.show()


func _update_value():
	if not is_node_ready(): return
	%FieldCheckButton.button_pressed = value
	
	%FieldName.text = field_name
	
	Backend.set_node_field_value(get_parent().get_parent().get_parent().graph_id, get_parent().get_parent().node_id, index, Data.get_data_type_from_field_type(Data.BOOLEAN), value)

func _ready() -> void:
	_update_value()


func _on_field_check_button_pressed() -> void:
	value = %FieldCheckButton.button_pressed
