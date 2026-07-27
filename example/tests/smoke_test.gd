extends SceneTree

# Headless regression test for the example project, run via
# `godot_smoke.rs`. Confirms:
#   1. The generated RustRuntime compiles, registers Clock, and dispatches.
#   2. RpcRuntime.gd (the sole autoload) correctly routes a call to it —
#      the same path any cross-language RpcClient (Rust or Kotlin) uses.
#   3. If this process was launched with a JVM-enabled Godot build (a
#      `godot-kotlin` binary), check the same for KotlinRuntime/GameService.
#
# Godot4 cannot load Kotlin-compiled classes, so the Kotlin check
# above is conditional on RpcRuntime actually having a GameService route.
# Kotlin-compiled classes aren't ClassDB-native, so we use
# `RpcRuntime.has_service()` to check if `GameService` has been loaded.


# To avoid dependency on a GDScript protobuf library, we hand-encode a
# `PingRequest { message }` (one string field) in protobuf wire
# bytes: tag 0x0A (field 1, wire type 2 / length-delimited) + varint length +
# UTF-8 bytes.
func _encode_ping_request(message: String) -> PackedByteArray:
	var message_bytes: PackedByteArray = message.to_utf8_buffer()
	var bytes := PackedByteArray()
	bytes.append(0x0A)
	bytes.append(message_bytes.size())
	bytes.append_array(message_bytes)
	return bytes

func _test_rust_service(rpc_runtime: Node) -> void:
	if not ClassDB.class_exists("RustRuntime"):
		push_error("RustRuntime class not found — did the GDExtension load?")
		quit(1)
		return

	var current_time_req := PackedByteArray()
	var current_time_resp: PackedByteArray = rpc_runtime.invoke("Clock", "CurrentTime", current_time_req)
	print("RpcRuntime.invoke('Clock', 'CurrentTime', ...) -> ", current_time_resp)

	if current_time_resp.is_empty():
		push_error("Clock.CurrentTime returned an empty response via RpcRuntime.gd")
		quit(1)
		return

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to RustRuntime's Clock.")


func _test_kotlin_service(rpc_runtime: Node) -> void:
	if not rpc_runtime.has_service("GameService"):
		print("smoke_test: no GameService route registered — skipping Kotlin integration check (needs the godot-kotlin binary; see ../../scripts/run_godot_tests.sh).")
		return

	var ping_req := _encode_ping_request("ping")
	var ping_resp: PackedByteArray = rpc_runtime.invoke("GameService", "Ping", ping_req)
	print("RpcRuntime.invoke('GameService', 'Ping', ...) -> ", ping_resp)

	if ping_resp.is_empty():
		push_error("GameService.Ping returned an empty response via RpcRuntime.gd")
		quit(1)
		return

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to KotlinRuntime's GameService.")


func _initialize() -> void:
	# Wait a frame so the RpcRuntime autoload's _ready() has bootstrapped
	# every language runtime it found.
	await process_frame

	var rpc_runtime: Node = root.get_node_or_null("RpcRuntime")
	if rpc_runtime == null:
		push_error("RpcRuntime autoload not found — is it registered in project.godot?")
		quit(1)
		return

	_test_rust_service(rpc_runtime)
	_test_kotlin_service(rpc_runtime)
	quit(0)
