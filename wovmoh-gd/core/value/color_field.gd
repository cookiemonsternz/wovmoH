@tool
class_name ColorField extends PanelContainer

@export var field_name: String = ""
@export var value: Color:
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
		%ColorPickerButton.hide()
	else:
		%ColorPickerButton.show()

func _update_value():
	if not is_node_ready(): return
	%ColorPickerButton.color = value
	
	%FieldName.text = field_name
	
	var value_json = '{ "e": [%s, %s, %s, %s]}' % [value.r, value.g, value.b, value.a]
	Backend.set_node_field_value(get_parent().get_parent().get_parent().graph_id, get_parent().get_parent().node_id, index, Data.get_data_type_from_field_type(Data.COLOR), value_json)

func _ready() -> void:
	_update_value()

func _on_color_picker_button_color_changed(color: Color) -> void:
	value = color
