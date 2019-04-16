const __exports = {};

let wasm;

/**
* @returns {void}
*/
export function init_app() {
    return wasm.init_app();
}

__exports.init_app = init_app;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

function __widl_instanceof_CanvasRenderingContext2D(idx) { return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0; }

__exports.__widl_instanceof_CanvasRenderingContext2D = __widl_instanceof_CanvasRenderingContext2D;

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, exnptr) {
    try {
        getObject(arg0).drawImage(getObject(arg1), arg2, arg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D = __widl_f_draw_image_with_html_image_element_CanvasRenderingContext2D;

function __widl_f_begin_path_CanvasRenderingContext2D(arg0) {
    getObject(arg0).beginPath();
}

__exports.__widl_f_begin_path_CanvasRenderingContext2D = __widl_f_begin_path_CanvasRenderingContext2D;

function __widl_f_stroke_CanvasRenderingContext2D(arg0) {
    getObject(arg0).stroke();
}

__exports.__widl_f_stroke_CanvasRenderingContext2D = __widl_f_stroke_CanvasRenderingContext2D;

function __widl_f_set_stroke_style_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).strokeStyle = getObject(arg1);
}

__exports.__widl_f_set_stroke_style_CanvasRenderingContext2D = __widl_f_set_stroke_style_CanvasRenderingContext2D;

function __widl_f_set_line_width_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).lineWidth = arg1;
}

__exports.__widl_f_set_line_width_CanvasRenderingContext2D = __widl_f_set_line_width_CanvasRenderingContext2D;

function __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    getObject(arg0).lineTo(arg1, arg2);
}

__exports.__widl_f_line_to_CanvasRenderingContext2D = __widl_f_line_to_CanvasRenderingContext2D;

function __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
}

__exports.__widl_f_move_to_CanvasRenderingContext2D = __widl_f_move_to_CanvasRenderingContext2D;

function __widl_f_clear_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
}

__exports.__widl_f_clear_rect_CanvasRenderingContext2D = __widl_f_clear_rect_CanvasRenderingContext2D;

function __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
}

__exports.__widl_f_fill_rect_CanvasRenderingContext2D = __widl_f_fill_rect_CanvasRenderingContext2D;

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __widl_f_create_element_Document(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(getObject(arg0).createElement(varg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_create_element_Document = __widl_f_create_element_Document;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function __widl_f_body_Document(arg0) {

    const val = getObject(arg0).body;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_body_Document = __widl_f_body_Document;

function __widl_instanceof_Element(idx) { return getObject(idx) instanceof Element ? 1 : 0; }

__exports.__widl_instanceof_Element = __widl_instanceof_Element;

function __widl_f_set_attribute_Element(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setAttribute(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_set_attribute_Element = __widl_f_set_attribute_Element;

function __widl_f_client_width_Element(arg0) {
    return getObject(arg0).clientWidth;
}

__exports.__widl_f_client_width_Element = __widl_f_client_width_Element;

function __widl_f_client_height_Element(arg0) {
    return getObject(arg0).clientHeight;
}

__exports.__widl_f_client_height_Element = __widl_f_client_height_Element;

function __widl_f_set_inner_html_Element(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).innerHTML = varg1;
}

__exports.__widl_f_set_inner_html_Element = __widl_f_set_inner_html_Element;

function __widl_f_add_event_listener_with_callback_EventTarget(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).addEventListener(varg1, getObject(arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_add_event_listener_with_callback_EventTarget = __widl_f_add_event_listener_with_callback_EventTarget;

function __widl_instanceof_HTMLCanvasElement(idx) { return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0; }

__exports.__widl_instanceof_HTMLCanvasElement = __widl_instanceof_HTMLCanvasElement;

function __widl_f_get_context_HTMLCanvasElement(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_get_context_HTMLCanvasElement = __widl_f_get_context_HTMLCanvasElement;

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            arg = arg.substring(read);
            writeOffset += written;
            if (arg.length === 0) {
                break;
            }
            ptr = wasm.__wbindgen_realloc(ptr, size, size * 2);
            size *= 2;
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}

function __widl_f_to_data_url_HTMLCanvasElement(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).toDataURL());
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_to_data_url_HTMLCanvasElement = __widl_f_to_data_url_HTMLCanvasElement;

function __widl_f_set_width_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).width = arg1;
}

__exports.__widl_f_set_width_HTMLCanvasElement = __widl_f_set_width_HTMLCanvasElement;

function __widl_f_set_height_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).height = arg1;
}

__exports.__widl_f_set_height_HTMLCanvasElement = __widl_f_set_height_HTMLCanvasElement;

function __widl_instanceof_HTMLElement(idx) { return getObject(idx) instanceof HTMLElement ? 1 : 0; }

__exports.__widl_instanceof_HTMLElement = __widl_instanceof_HTMLElement;

function __widl_f_set_onload_HTMLElement(arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
}

__exports.__widl_f_set_onload_HTMLElement = __widl_f_set_onload_HTMLElement;

function __widl_f_new_Image(exnptr) {
    try {
        return addHeapObject(new Image());
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_new_Image = __widl_f_new_Image;

function __widl_f_set_src_HTMLImageElement(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).src = varg1;
}

__exports.__widl_f_set_src_HTMLImageElement = __widl_f_set_src_HTMLImageElement;

function __widl_f_offset_x_MouseEvent(arg0) {
    return getObject(arg0).offsetX;
}

__exports.__widl_f_offset_x_MouseEvent = __widl_f_offset_x_MouseEvent;

function __widl_f_offset_y_MouseEvent(arg0) {
    return getObject(arg0).offsetY;
}

__exports.__widl_f_offset_y_MouseEvent = __widl_f_offset_y_MouseEvent;

function __widl_f_append_child_Node(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_append_child_Node = __widl_f_append_child_Node;

function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }

__exports.__widl_instanceof_Window = __widl_instanceof_Window;

function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_document_Window = __widl_f_document_Window;

function __wbg_newnoargs_b4526aa2a6db81de(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

__exports.__wbg_newnoargs_b4526aa2a6db81de = __wbg_newnoargs_b4526aa2a6db81de;

function __wbg_call_a7a8823c404228ab(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__wbg_call_a7a8823c404228ab = __wbg_call_a7a8823c404228ab;

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }

__exports.__wbindgen_string_new = __wbindgen_string_new;

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

__exports.__wbindgen_debug_string = __wbindgen_debug_string;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

__exports.__wbindgen_cb_drop = __wbindgen_cb_drop;

const __wbindgen_cb_forget = dropObject;

__exports.__wbindgen_cb_forget = __wbindgen_cb_forget;

function __wbindgen_rethrow(idx) { throw takeObject(idx); }

__exports.__wbindgen_rethrow = __wbindgen_rethrow;

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

function __wbindgen_closure_wrapper74(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(24);
    const d = wasm.__wbg_function_table.get(25);
    const cb = function() {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper74 = __wbindgen_closure_wrapper74;

function __wbindgen_closure_wrapper76(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(28);
    const d = wasm.__wbg_function_table.get(25);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper76 = __wbindgen_closure_wrapper76;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './d0': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    });
}

export default init;

