extends Node

signal field_selected(field)

const NORMAL = 0
const SELECTING_FIELD = 1

var current_state = NORMAL
