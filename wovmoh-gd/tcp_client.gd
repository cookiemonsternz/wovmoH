class_name TcpClient extends Node

signal connected
signal response(response_data: Variant)
signal disconnected
signal error

var _status: int = 0
var _stream: StreamPeerTCP = StreamPeerTCP.new()
var _receive_buffer: PackedByteArray = PackedByteArray()

func _ready() -> void:
	_stream.poll()
	_status = _stream.get_status()

@warning_ignore("unused_parameter")
func _process(delta: float) -> void:
	_stream.poll()
	var new_status: int = _stream.get_status()
	if new_status != _status:
		_status = new_status
		match _status:
			_stream.STATUS_NONE:
				print("Disconnected from host.")
				emit_signal("disconnected")
			_stream.STATUS_CONNECTING:
				print("Connecting to host.")
			_stream.STATUS_CONNECTED:
				print("Connected to host.")
				emit_signal("connected")
			_stream.STATUS_ERROR:
				print("Error with socket stream.")
				emit_signal("error")

	if _status == _stream.STATUS_CONNECTED:
		var available_bytes: int = _stream.get_available_bytes()
		if available_bytes <= 0: return

		var result := _stream.get_partial_data(available_bytes)
		# Check for read error.
		if result[0] != OK:
			print("Error getting data from stream: ", result[0])
			error.emit()
			return
		
		_receive_buffer.append_array(result[1])
		_process_receive_buffer()

func _process_receive_buffer() -> void:
	while true:
		var newline_index := _receive_buffer.find(10) # 10 = \n
		if newline_index == -1: return
		
		# get message from buffer
		var message_bytes := _receive_buffer.slice(0, newline_index)
		# Remove message from buffer
		_receive_buffer = _receive_buffer.slice(newline_index + 1)
		
		if message_bytes.is_empty(): continue
		
		var message_text := message_bytes.get_string_from_utf8()
		var json := JSON.new()
		var parse_error := json.parse(message_text)
		if parse_error != OK:
			push_warning("Invalid JSON: ", message_text)
			continue
		
		var response_data: Variant = json.data
		response.emit(response_data)

func connect_to_host(host: String, port: int) -> void:
	print("Connecting to %s:%d" % [host, port])
	# Reset status so we can tell if it changes to error again.
	_status = _stream.STATUS_NONE
	if _stream.connect_to_host(host, port) != OK:
		print("Error connecting to host.")
		emit_signal("error")
	
	_stream.set_no_delay(true)

@warning_ignore("shadowed_variable")
func send(data: PackedByteArray) -> bool:
	if _status != _stream.STATUS_CONNECTED:
		print("Error: Stream is not currently connected.")
		return false
	
	# Add newline to terminate request
	data.append_array("\n".to_utf8_buffer())
	
	@warning_ignore("shadowed_variable")
	var error: int = _stream.put_data(data)
	#print("Sent data: ", data, " err: ", error)
	if error != OK:
		print("Error writing to stream: ", error)
		return false
	return true
