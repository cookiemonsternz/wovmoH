extends Node

const NUMBER = 0
const BOOLEAN = 1
const COLOR = 2
const VECTOR3 = 3
const POINT3 = 4

const PORT_COLORS = {
	NUMBER: Color.RED,
	BOOLEAN: Color.GREEN,
	COLOR: Color.BLUE,
	VECTOR3: Color.YELLOW,
	POINT3: Color.PINK,
}

func get_field_type_from_data(field_data: Variant) -> int:
	match field_data["data_type"]:
		"Number": return NUMBER
		"Boolean": return BOOLEAN
		"Color": return COLOR
		"Vector3": return VECTOR3
		"Point3": return POINT3
	return -1

func get_data_type_from_field_type(type: int) -> String:
	match type:
		NUMBER: return "Number"
		BOOLEAN: return "Boolean"
		COLOR: return "Color"
		VECTOR3: return "Vector3"
		POINT3: return "Point3"
	return ""
