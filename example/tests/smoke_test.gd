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


# To avoid dependency on a GDScript protobuf library, we hand-encode/decode
# any message shaped like `{ string field = 1; }` in protobuf wire bytes:
# tag 0x0A (field 1, wire type 2 / length-delimited) + varint length + UTF-8
# bytes.
func _encode_single_string_field(value: String) -> PackedByteArray:
	var value_bytes: PackedByteArray = value.to_utf8_buffer()
	var bytes := PackedByteArray()
	bytes.append(0x0A)
	bytes.append(value_bytes.size())
	bytes.append_array(value_bytes)
	return bytes


# Decodes a *successful envelope* (status byte 0x00) wrapping a message
# a serialized single-string protobuf message.
# Returns empty string if the field is absent or if the envelope isn't
# an OK response.
func _decode_enveloped_single_string_field(response: PackedByteArray) -> String:
	if response.size() < 1 or response[0] != 0x00:
		return ""
	var payload := response.slice(1)
	if payload.size() < 2 or payload[0] != 0x0A:
		return ""
	var length: int = payload[1]
	return payload.slice(2, 2 + length).get_string_from_utf8()


# Wire-level envelope error codes — must match `RpcErrorCode`.
const _ERROR_CODE_UNKNOWN_SERVICE := 0
const _ERROR_CODE_APPLICATION := 3

# Decodes an error envelope's code (status byte 0x01 + code:i32 LE + a
# length-prefixed UTF-8 message).
# Returns -1 if `response` isn't a well-formed error envelope (e.g. it's a
# successful response instead).
func _decode_error_envelope_code(response: PackedByteArray) -> int:
	if response.size() < 9 or response[0] != 0x01:
		return -1
	return response[1] | (response[2] << 8) | (response[3] << 16) | (response[4] << 24)


func _encode_ping_request(message: String) -> PackedByteArray:
	return _encode_single_string_field(message)

func _encode_unlock_achievement_request(achievement_id: String) -> PackedByteArray:
	return _encode_single_string_field(achievement_id)

func _encode_probe_broken_service_request(target: String) -> PackedByteArray:
	return _encode_single_string_field(target)

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


# Confirms RpcRuntime.gd's `invoke()` reports an unregistered service_id as a
# decodable error envelope (status byte 0x01, code 0 == UnknownService)
# instead of crashing on a null dereference — regression check for the
# `assert()`-is-a-no-op-in-release gap this framework used to have.
func _test_unknown_service(rpc_runtime: Node) -> bool:
	var request := PackedByteArray()
	var response: PackedByteArray = rpc_runtime.invoke("NoSuchService", "NoSuchMethod", request)
	print("RpcRuntime.invoke('NoSuchService', 'NoSuchMethod', ...) -> ", response)

	var code := _decode_error_envelope_code(response)
	if code != _ERROR_CODE_UNKNOWN_SERVICE:
		push_error("expected UnknownService error envelope (code %d) from an unregistered service, got %s" % [_ERROR_CODE_UNKNOWN_SERVICE, response])
		return false

	print("smoke_test OK: RpcRuntime.gd reports an unknown service as a decodable error envelope.")
	return true


# Confirms that calling a Broken* service's Fail method surfaces as an
# Application-coded error envelope at the RpcRuntime.gd level.
func _test_broken_service_envelope(rpc_runtime: Node, service_id: String) -> bool:
	if not rpc_runtime.has_service(service_id):
		print("smoke_test: no service %s registered — skipping broken service test" % [service_id])
		return true

	var request := PackedByteArray()
	var response: PackedByteArray = rpc_runtime.invoke(service_id, "Fail", request)
	print("RpcRuntime.invoke('%s', 'Fail', ...) -> %s" % [service_id, response])

	var code := _decode_error_envelope_code(response)
	if code != _ERROR_CODE_APPLICATION:
		push_error("expected Application error envelope (code %d) from '%s'.Fail, got %s" % [_ERROR_CODE_APPLICATION, service_id, response])
		return false

	print("smoke_test OK: RpcRuntime.gd reports %s.Fail as a decodable Application-coded error envelope." % service_id)
	return true


# Calls `prober_service_id.ProbeBrokenService(target)`. This dispatches
# a call to BrokenXXX service's `target` method, which should always fail
# with a message that includes "$target".
#
# In every language, ProbeBrokenService should catch the error thrown
# by the callee and return it in the response, instead of crashing.
func _test_probe_broken_service(rpc_runtime: Node, prober_service_id: String, target: String) -> bool:
	var request := _encode_probe_broken_service_request(target)
	var response: PackedByteArray = rpc_runtime.invoke(prober_service_id, "ProbeBrokenService", request)
	print("RpcRuntime.invoke('%s', 'ProbeBrokenService', target=%s) -> %s" % [prober_service_id, target, response])

	var error_message := _decode_enveloped_single_string_field(response)
	if not error_message.contains(target):
		push_error("expected %s.ProbeBrokenService(%s)'s error_message to mention %s, got: '%s' (raw: %s)" % [prober_service_id, target, target, error_message, response])
		return false

	print("smoke_test OK: %s's error crossed into %s's caller as: %s" % [target, prober_service_id, error_message])
	return true


# Rust's Profiler is compatible with Kotlin and C#.
func _test_probe_broken_services_from_rust(rpc_runtime: Node) -> bool:
	if not ClassDB.class_exists("RustRuntime"):
		push_error("RustRuntime class not found — did the GDExtension load?")
		return false

	var passed = true
	for target in ["BrokenKotlin", "BrokenCSharp"]:
		if rpc_runtime.has_service(target):
			passed = _test_probe_broken_service(rpc_runtime, "Profiler", target) and passed
		else:
			print("smoke_test: no %s route registered — skipping Rust cross-language error propagation check for %s." % [target, target])
	return passed


# Kotlin's GameService is compatible with Rust.
func _test_probe_broken_services_from_kotlin(rpc_runtime: Node) -> bool:
	if not rpc_runtime.has_service("GameService"):
		print("smoke_test: no GameService route registered — skipping Kotlin cross-language error propagation check.")
		return true

	var passed = true
	for target in ["BrokenRust"]:
		passed = _test_probe_broken_service(rpc_runtime, "GameService", target) and passed
	return passed


# C#'s Achievements is compatible with Rust.
func _test_probe_broken_services_from_csharp(rpc_runtime: Node) -> bool:
	if not rpc_runtime.has_service("Achievements"):
		print("smoke_test: no Achievements route registered — skipping C# cross-language error propagation check.")
		return true

	var passed = true
	for target in ["BrokenRust"]:
		passed = _test_probe_broken_service(rpc_runtime, "Achievements", target) and passed
	return passed


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
	passed = _test_unknown_service(rpc_runtime) and passed

	# Confirm that broken services return the error properly through the encoded
	# envelope.
	passed = _test_broken_service_envelope(rpc_runtime, "BrokenRust") and passed
	passed = _test_broken_service_envelope(rpc_runtime, "BrokenKotlin") and passed
	passed = _test_broken_service_envelope(rpc_runtime, "BrokenCSharp") and passed

	# Confirm that application errors propagate across language boundaries.
	passed = _test_probe_broken_services_from_rust(rpc_runtime) and passed
	passed = _test_probe_broken_services_from_kotlin(rpc_runtime) and passed
	passed = _test_probe_broken_services_from_csharp(rpc_runtime) and passed

	quit(0 if passed else 1)
