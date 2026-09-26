extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$TcpClient.connect_to_host("127.0.0.1", 7878)

func send_Some_data():
	$TcpClient.send("Hello from godot".to_utf8_buffer())

func _on_tcp_client_connected() -> void:
	print("CONNECTED")


func _on_tcp_client_data_recieved(data: PackedByteArray) -> void:
	print("DATA RECIEVED: ", data)


func _on_tcp_client_disconnected() -> void:
	print("DISCONNECTED")


func _on_tcp_client_error() -> void:
	print("ERROR")
