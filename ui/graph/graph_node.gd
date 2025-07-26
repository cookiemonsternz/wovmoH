class_name WovmohGraphNode extends GraphNode

var pins = {}

func set_node(json):
	name = json.id
	title = json.name
	#id_label.text = json.id
	#type_label.text = json.type
	
	var x = json.properties.position.x
	var y = json.properties.position.y
	var z = json.properties.position.z
	
	position_offset = Vector2(x, y);
	var i = 0;
	for outputPinJson in json.properties.outputPins:
		var pin_object = OutputGraphPin.new()
		add_child(pin_object)
		
		pin_object.set_pin(outputPinJson)
		
		set_slot(i, false, 0, Color.TRANSPARENT, true, 0, Color.RED)
		
		pins[outputPinJson.id] = pin_object
		i += 1;
	
	# Don't reset i here so that it's offset by the number of pins already there
	for inputPinJson in json.properties.inputPins:
		var pin_object = InputGraphPin.new()
		add_child(pin_object)
		
		pin_object.set_pin(inputPinJson)
		
		set_slot(i, true, 0, Color.RED, false, 0, Color.TRANSPARENT)
		
		pins[inputPinJson.id] = pin_object
		i += 1;

func get_pin(pin_name: StringName):
	var children = get_children()
	var filteredChildren = []
	for child in children:
		if child is InputGraphPin || child is OutputGraphPin:
			filteredChildren.append(child)
	
	
	for i in range(len(filteredChildren)):
		if filteredChildren[i].name == pin_name:
			print(pin_name, " : ", filteredChildren, " : ",  i)
			return i
