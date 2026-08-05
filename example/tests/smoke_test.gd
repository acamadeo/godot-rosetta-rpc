extends SceneTree

# Headless regression test for the example project, run via
# `godot_smoke.rs`. Confirms:
#   1. The generated RustRuntime compiles, registers Clock, and dispatches.
#   2. RpcRuntime.gd (the sole autoload) correctly routes a call to it —
#      the same path any cross-language RpcClient (Rust, Kotlin, or C#) uses.
#   3. If this process was launched with a JVM-enabled Godot build (a
#      `godot-kotlin` binary), check the same for KotlinRuntime/GameService.
#   4. If this process was launched with a .NET-enabled Godot build (a
#      `godot-mono` binary), check the same for CSharpRuntime/Achievements.
#
# A vanilla godot4 binary can load neither Kotlin- nor C#-compiled classes,
# so both checks above are conditional on RpcRuntime actually having the
# corresponding route. Neither Kotlin- nor C#-compiled global classes are
# ClassDB-native, so we use `RpcRuntime.has_service()` to check if
# `GameService`/`Achievements` has been loaded, rather than `ClassDB.class_exists()`.


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


# Same one-string-field wire encoding as _encode_ping_request, for
# `UnlockAchievementRequest { achievement_id }`.
func _encode_unlock_achievement_request(achievement_id: String) -> PackedByteArray:
	var id_bytes: PackedByteArray = achievement_id.to_utf8_buffer()
	var bytes := PackedByteArray()
	bytes.append(0x0A)
	bytes.append(id_bytes.size())
	bytes.append_array(id_bytes)
	return bytes

func _test_clock_service(rpc_runtime: Node) -> bool:
	if not ClassDB.class_exists("RustRuntime"):
		push_error("RustRuntime class not found — did the GDExtension load?")
		return false

	var request := PackedByteArray()
	var response: PackedByteArray = rpc_runtime.invoke("Clock", "CurrentTime", request)
	print("RpcRuntime.invoke('Clock', 'CurrentTime', ...) -> ", response)

	if response.is_empty():
		push_error("Clock.CurrentTime returned an empty response via RpcRuntime.gd")
		return false

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to RustRuntime's Clock.")
	return true


func _test_profiler_service(rpc_runtime: Node) -> bool:
	if not ClassDB.class_exists("RustRuntime"):
		push_error("RustRuntime class not found — did the GDExtension load?")
		return false

	var request := PackedByteArray()
	var response: PackedByteArray = rpc_runtime.invoke("Profiler", "Profile", request)
	print("RpcRuntime.invoke('Profiler', 'Profile', ...) -> ", response)

	if response.is_empty():
		push_error("Profiler.Profile returned an empty response via RpcRuntime.gd")
		return false

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to RustRuntime's Profiler.")
	return true


func _test_game_service(rpc_runtime: Node) -> bool:
	if not rpc_runtime.has_service("GameService"):
		print("smoke_test: no GameService route registered — skipping Kotlin integration check (needs the godot-kotlin binary).")
		return true

	var request := _encode_ping_request("ping")
	var response: PackedByteArray = rpc_runtime.invoke("GameService", "Ping", request)
	print("RpcRuntime.invoke('GameService', 'Ping', ...) -> ", response)

	if response.is_empty():
		push_error("GameService.Ping returned an empty response via RpcRuntime.gd")
		return false

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to KotlinRuntime's GameService.")
	return true


func _test_achievements_service(rpc_runtime: Node) -> bool:
	if not rpc_runtime.has_service("Achievements"):
		print("smoke_test: no Achievements route registered — skipping C# integration check (needs a .NET-enabled Godot binary).")
		return true

	var request := _encode_unlock_achievement_request("first_blood")
	var response: PackedByteArray = rpc_runtime.invoke("Achievements", "UnlockAchievement", request)
	print("RpcRuntime.invoke('Achievements', 'UnlockAchievement', ...) -> ", response)

	if response.is_empty():
		push_error("Achievements.UnlockAchievement returned an empty response via RpcRuntime.gd")
		return false

	print("smoke_test OK: RpcRuntime.gd correctly routed a call to CSharpRuntime's Achievements.")
	return true


func _initialize() -> void:
	# Wait a frame so the RpcRuntime autoload's _ready() has bootstrapped
	# every language runtime it found.
	await process_frame

	var rpc_runtime: Node = root.get_node_or_null("RpcRuntime")
	if rpc_runtime == null:
		push_error("RpcRuntime autoload not found — is it registered in project.godot?")
		quit(1)
		return

	var passed = true
	passed = _test_clock_service(rpc_runtime) and passed
	passed = _test_profiler_service(rpc_runtime) and passed
	passed = _test_game_service(rpc_runtime) and passed
	passed = _test_achievements_service(rpc_runtime) and passed
	quit(0 if passed else 1)
