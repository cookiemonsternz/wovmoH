extends HBoxContainer

enum DataType {
	NUMBER, 
	BOOLEAN,
	COLOR, 
	VECTOR3, 
	POINT3
}

@export_enum("Number", "Boolean", "Color", "Vector3", "Point3") var data_type:
	set(new):
		data_type = new
		_update_type()

func _update_type():
	match data_type:
		DataType.NUMBER:
			
