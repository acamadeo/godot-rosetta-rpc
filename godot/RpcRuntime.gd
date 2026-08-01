extends Node

# This file is the one autoload that godot-rosetta-rpc requires. It bootstraps
# every language runtime as a child node, and registers which runtime implements
# which services. When a service method is invoked, it delegates the request to the
# proper runtime.

# Where to search for `KotlinRuntime.gdj`. This assumes the Kotlin project uses the
# following path defined in `build.gradle.kts`.
# ```
# registrationFilesDirectory.set(projectDir.resolve("kotlin/gdj"))
# ```
const KOTLIN_GDJ_SEARCH_ROOT := "res://kotlin/gdj"

var _service_routes: Dictionary = {} # service_id (String) -> Node (owning runtime)

func _ready() -> void:
	if ClassDB.class_exists("RustRuntime"):
		var rust_runtime: Node = ClassDB.instantiate("RustRuntime")
		add_child(rust_runtime)
		rust_runtime.call("bootstrap")
		_register_routes(rust_runtime)

	# Unlike gdext, godot-kotlin-jvm classes do not register in ClassDB.
	# Instead, find the generated .gdj registration file and manually load it.
	var kotlin_runtime_gdj_path := _recurse_find_file(KOTLIN_GDJ_SEARCH_ROOT, "KotlinRuntime.gdj")
	if kotlin_runtime_gdj_path != "":
		var kotlin_runtime_script: Script = load(kotlin_runtime_gdj_path)
		var kotlin_runtime: Node = kotlin_runtime_script.new()
		add_child(kotlin_runtime)
		kotlin_runtime.call("bootstrap")
		_register_routes(kotlin_runtime)

	# Adding another language: instantiate its runtime class the same way —
	# via ClassDB if it registers native classes (like Rust), or via a
	# directory scan for its own equivalent of a .gdj file. Then call
	#  _register_routes() on it.

# Returns whether the runtime has invoked service_id.
func has_service(service_id: String) -> bool:
	return _service_routes.has(service_id)


# Invokes /<service_id>.<method_id> with the given request and returns its response as a byte
# stream.
# 
# This is the single entry point every language's RpcClient calls, regardless
# of which language actually implements service_id.
func invoke(service_id: String, method_id: String, request_bytes: PackedByteArray) -> PackedByteArray:
	var runtime_node: Node = _service_routes.get(service_id)
	assert(runtime_node != null, "RpcRuntime: no runtime registered for service '%s'" % service_id)
	return runtime_node.call("invoke", service_id, method_id, request_bytes)


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

func _register_routes(runtime_node: Node) -> void:
	for service_id in runtime_node.call("registered_service_ids"):
		if _service_routes.has(service_id):
			push_error("RpcRuntime: service '%s' is already registered by another runtime" % service_id)
			continue
		_service_routes[service_id] = runtime_node
