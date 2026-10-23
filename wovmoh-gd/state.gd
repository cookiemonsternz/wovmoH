extends Node

signal field_selected(field: Field)
signal pin_selected(pin: Pin)
signal artnet_field_selected(field: Field)
signal artnet_pin_selected(pin: Pin)

const NORMAL = 0
const SELECTING_FIELD = 1
const SELECTING_PIN = 2
const ARTNET_SELECTING_FIELD = 3
const ARTNET_SELECTING_PIN = 4

var current_state: int = NORMAL
