class_name Field extends Control

@export var field_name: String

var number_field_scene = preload("res://core/value/number_field.tscn")
var boolean_field_scene = preload("res://core/value/boolean_field.tscn")
var color_field_scene = preload("res://core/value/color_field.tscn")

func setup(field_data: Variant, index: int):
	match field_data["data_type"]:
		"Number":
			var number_field: NumberField = number_field_scene.instantiate()
			number_field.field_name = field_data["name"]
			number_field.value = field_data["value"]["value"]
			number_field.min_value = -INF if field_data["constraints"]["value"]["min"] == null else field_data["constraints"]["value"]["min"]
			number_field.max_value = INF if field_data["constraints"]["value"]["max"] == null else field_data["constraints"]["value"]["max"]
			if number_field.min_value == -INF or number_field.max_value == INF:
				number_field.absolute_drag = false
			number_field.index = index
			add_child(number_field)
		"Boolean":
			var boolean_field: BooleanField = boolean_field_scene.instantiate()
			boolean_field.field_name = field_data["name"]
			boolean_field.value = field_data["value"]["value"]
			boolean_field.index = index
			add_child(boolean_field)
		"Color":
			var color_field: ColorField = color_field_scene.instantiate()
			color_field.field_name = field_data["name"]
			var color_arr = field_data["value"]["value"]["e"]
			color_field.value = Color(color_arr[0], color_arr[1], color_arr[2], color_arr[3])
			color_field.index = index
			add_child(color_field)
