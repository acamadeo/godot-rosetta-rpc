extends Node

# This file is the one autoload that godot-rosetta-rpc requires. It bootstraps
# every language runtime as a child node, and registers which runtime implements
# which services. When a service method is invoked, it delegates the request to the
# proper runtime.

# Fallback search roots for KotlinRuntime.gdj / CSharpRuntime.cs, used only if
# a project's [rosetta_rpc] section (written by install.py's
# --kotlin-gdj-root / --csharp-rpc-root flags — see below) doesn't override
# them.
# 
# For Kotlin, this should match the path defined in `build.gradle.kts`:
# ```
# registrationFilesDirectory.set(projectDir.resolve("kotlin/gdj"))
# ```
# 
# For C#, this should match the directory where CSharpRuntime.cs is
# outputted.
const DEFAULT_KOTLIN_GDJ_SEARCH_ROOT := "res://kotlin/gdj"
const DEFAULT_CSHARP_RPC_SEARCH_ROOT := "res://csharp/generated/rpc"

var _kotlin_gdj_search_root: String = ProjectSettings.get_setting(
	"rosetta_rpc/kotlin_gdj_root", DEFAULT_KOTLIN_GDJ_SEARCH_ROOT
)
var _csharp_rpc_search_root: String = ProjectSettings.get_setting(
	"rosetta_rpc/csharp_rpc_root", DEFAULT_CSHARP_RPC_SEARCH_ROOT
)

# service_id (String) -> Dictionary{node: Node, invoke_method: String}.
# - invoke_method varies per language: snake_case for Rust/Kotlin,
#     PascalCase for C#.
var _service_routes: Dictionary = {}

func _ready() -> void:
	if ClassDB.class_exists("RustRuntime"):
		var rust_runtime: Node = ClassDB.instantiate("RustRuntime")
		add_child(rust_runtime)
		rust_runtime.call("bootstrap")
		_register_routes(rust_runtime)

	# Unlike gdext, godot-kotlin-jvm classes do not register in ClassDB.
	# Instead, find the generated .gdj registration file and manually load it.
	var kotlin_runtime_gdj_path := _recurse_find_file(_kotlin_gdj_search_root, "KotlinRuntime.gdj")
	if kotlin_runtime_gdj_path != "":
		var kotlin_runtime_script: Script = load(kotlin_runtime_gdj_path)
		if kotlin_runtime_script != null:
			var kotlin_runtime: Node = kotlin_runtime_script.new()
			add_child(kotlin_runtime)
			kotlin_runtime.call("bootstrap")
			_register_routes(kotlin_runtime)

	# Unlike gdext, godot-kotlin-jvm classes do not register in ClassDB.
	# Instead, find the generated CSharpRuntime.cs file and manually load it.
	var csharp_runtime_path := _recurse_find_file(_csharp_rpc_search_root, "CSharpRuntime.cs")
	if csharp_runtime_path != "":
		var csharp_runtime_script: Script = load(csharp_runtime_path)
		if csharp_runtime_script != null:
			var csharp_runtime: Node = csharp_runtime_script.new()
			add_child(csharp_runtime)
			csharp_runtime.call("Bootstrap")
			_register_routes(csharp_runtime, "Invoke", "RegisteredServiceIds")

	# Adding another language: instantiate its runtime class the same way —
	# via ClassDB if it registers native classes (like Rust), or via a
	# directory scan for its own equivalent of a KotlinRuntime.gdj or
	# CSharpRuntime.cs file. Then call _register_routes() on it.

# Returns whether the runtime has invoked service_id.
func has_service(service_id: String) -> bool:
	return _service_routes.has(service_id)


# Invokes /<service_id>.<method_id> with the given request and returns its response as a byte
# stream.
# 
# This is the single entry point every language's RpcClient calls, regardless
# of which language actually implements service_id.
func invoke(service_id: String, method_id: String, request_bytes: PackedByteArray) -> PackedByteArray:
	var route = _service_routes.get(service_id)
	assert(route != null, "RpcRuntime: no runtime registered for service '%s'" % service_id)
	return route.node.call(route.invoke_method, service_id, method_id, request_bytes)


# Recursively searches under dir_path for a file named filename, returning
# its res://<full_path> (or "" if not found).
func _recurse_find_file(dir_path: String, filename: String) -> String:
	var dir := DirAccess.open(dir_path)
	if dir == null:
		return ""
	dir.list_dir_begin()
	var entry := dir.get_next()
	while entry != "":
		var entry_path := dir_path.path_join(entry)
		if dir.current_is_dir():
			var found := _recurse_find_file(entry_path, filename)
			if found != "":
				dir.list_dir_end()
				return found
		elif entry == filename:
			dir.list_dir_end()
			return entry_path
		entry = dir.get_next()
	dir.list_dir_end()
	return ""

func _register_routes(
	runtime_node: Node,
	invoke_method: String = "invoke",
	registered_ids_method: String = "registered_service_ids",
) -> void:
	for service_id in runtime_node.call(registered_ids_method):
		if _service_routes.has(service_id):
			push_error("RpcRuntime: service '%s' is already registered by another runtime" % service_id)
			continue
		_service_routes[service_id] = {"node": runtime_node, "invoke_method": invoke_method}
