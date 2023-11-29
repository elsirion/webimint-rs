let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function debugString(val) {
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
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
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
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0ae7abd03c0ec6d7(arg0, arg1, addHeapObject(arg2));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6730492de49547bb(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_42(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2816eed2b2964108(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_45(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h81bf58d062ce2057(arg0, arg1);
}

function __wbg_adapter_48(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3d09aba4ae487408(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_55(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h23704e464ba585a9(arg0, arg1);
}

function __wbg_adapter_58(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfa8566cb69247caa(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_61(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb18dfd3fe49d0ad3(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
* @param {any} js_value
* @returns {string}
*/
export function process_js_value_with_cast(js_value) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.process_js_value_with_cast(retptr, addHeapObject(js_value));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        if (r3) {
            throw takeObject(r2);
        }
        var v1 = getCachedStringFromWasm0(r0, r1);
    if (r0 !== 0) { wasm.__wbindgen_free(r0, r1, 1); }
    return v1;
} finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
}
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
    console.error(v0);
};
imports.wbg.__wbg_mark_40e050a77cc39fea = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    performance.mark(v0);
};
imports.wbg.__wbg_measure_aa7a73f17813f708 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
var v1 = getCachedStringFromWasm0(arg2, arg3);
if (arg2 !== 0) { wasm.__wbindgen_free(arg2, arg3, 1); }
performance.measure(v0, v1);
}, arguments) };
imports.wbg.__wbg_log_c9486ca5d8e2cbe8 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
console.log(v0);
};
imports.wbg.__wbg_log_aba5996d9bde071f = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
var v1 = getCachedStringFromWasm0(arg2, arg3);
var v2 = getCachedStringFromWasm0(arg4, arg5);
var v3 = getCachedStringFromWasm0(arg6, arg7);
console.log(v0, v1, v2, v3);
};
imports.wbg.__wbg_new_314e9a73e4a7e909 = function(arg0, arg1, arg2) {
    const ret = new QrScanner(getObject(arg0), getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_start_3a584cd9ef5336bb = function(arg0) {
    getObject(arg0).start();
};
imports.wbg.__wbg_stop_04c276c9febf511c = function(arg0) {
    getObject(arg0).stop();
};
imports.wbg.__wbg_destroy_4ffb99b5f1db7e95 = function(arg0) {
    getObject(arg0).destroy();
};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbindgen_is_null = function(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
};
imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !getObject(arg0);
    return ret;
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};
imports.wbg.__wbg_fetch_57429b87be3dcc33 = function(arg0) {
    const ret = fetch(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_Window_cde2416cf5126a72 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_document_183cf1eecfdbffee = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_navigator_7078da62d92ff5ad = function(arg0) {
    const ret = getObject(arg0).navigator;
    return addHeapObject(ret);
};
imports.wbg.__wbg_localStorage_e11f72e996a4f5d9 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).localStorage;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_body_11da0c1aa9610cb3 = function(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_head_806a8bdbacfc5108 = function(arg0) {
    const ret = getObject(arg0).head;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_createComment_3b18d83b12cfbf48 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createComment(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createDocumentFragment_acf4f9887c44b09f = function(arg0) {
    const ret = getObject(arg0).createDocumentFragment();
    return addHeapObject(ret);
};
imports.wbg.__wbg_createElement_9ce3fdea8322ff34 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createElementNS_6a08d8f33e767e18 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    const ret = getObject(arg0).createElementNS(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_01a7250c5ca46b04 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_getElementById_328f8c4a5bb51ba8 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_querySelector_391afe271b8236d5 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).querySelector(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_classList_7fd39dc155235d8a = function(arg0) {
    const ret = getObject(arg0).classList;
    return addHeapObject(ret);
};
imports.wbg.__wbg_setinnerHTML_b88bf159b62c2334 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).innerHTML = v0;
};
imports.wbg.__wbg_removeAttribute_dbd76981f9bb9f59 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_setAttribute_aebcae2169f2f869 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_before_cf3a1481df28e2d5 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_remove_e8b7a30da3792f66 = function(arg0) {
    getObject(arg0).remove();
};
imports.wbg.__wbg_setdata_21158d298225cbec = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).data = v0;
};
imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};
imports.wbg.__wbg_signal_1ed842bebd6ae322 = function(arg0) {
    const ret = getObject(arg0).signal;
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_e4960143e41697a4 = function() { return handleError(function () {
    const ret = new AbortController();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_abort_8355f201f30300bb = function(arg0) {
    getObject(arg0).abort();
};
imports.wbg.__wbg_error_f0a6627f4b23c19d = function(arg0) {
    console.error(getObject(arg0));
};
imports.wbg.__wbg_warn_4affe1093892a4ef = function(arg0) {
    console.warn(getObject(arg0));
};
imports.wbg.__wbg_settext_e52e50ab1c672b72 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).text = v0;
}, arguments) };
imports.wbg.__wbg_fetch_701fcd2bde06379a = function(arg0, arg1) {
    const ret = getObject(arg0).fetch(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_append_885b90a83e43a189 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).append(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_instanceof_ShadowRoot_f85f709c953844de = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ShadowRoot;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_host_73c8e95bf9b31ccd = function(arg0) {
    const ret = getObject(arg0).host;
    return addHeapObject(ret);
};
imports.wbg.__wbg_key_ab25ddd694dd43f3 = function(arg0, arg1) {
    const ret = getObject(arg1).key;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_data_624fd2b6a4a5ac9e = function(arg0) {
    const ret = getObject(arg0).data;
    return addHeapObject(ret);
};
imports.wbg.__wbg_add_dc5c00591ed65268 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).add(v0);
}, arguments) };
imports.wbg.__wbg_remove_9517d3139a6031f1 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).remove(v0);
}, arguments) };
imports.wbg.__wbg_addEventListener_0f2891b0794e07fa = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_addEventListener_51709b9747ad8980 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3), getObject(arg4));
}, arguments) };
imports.wbg.__wbg_dispatchEvent_2434b822eb17a7b5 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).dispatchEvent(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_removeEventListener_104d11302bb212d1 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_new_19676474aa414d62 = function() { return handleError(function () {
    const ret = new Headers();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_append_feec4143bbf21904 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).append(v0, v1);
}, arguments) };
imports.wbg.__wbg_register_c70122ead214695d = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).register(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_readyState_13e55da5ad6d64e2 = function(arg0) {
    const ret = getObject(arg0).readyState;
    return ret;
};
imports.wbg.__wbg_setbinaryType_dcb62e0f2b346301 = function(arg0, arg1) {
    getObject(arg0).binaryType = takeObject(arg1);
};
imports.wbg.__wbg_new_b9b318679315404f = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new WebSocket(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_close_f4135085ec3fc8f0 = function() { return handleError(function (arg0) {
    getObject(arg0).close();
}, arguments) };
imports.wbg.__wbg_send_c1cc1284964b7434 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).send(v0);
}, arguments) };
imports.wbg.__wbg_send_2860805104507701 = function() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).send(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };
imports.wbg.__wbg_innerText_04f0cf82d8f317be = function(arg0, arg1) {
    const ret = getObject(arg1).innerText;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_value_5e860795f53217cd = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_length_568acd526026fc11 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).length;
    return ret;
}, arguments) };
imports.wbg.__wbg_getItem_c81cd3ae30cd579a = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getItem(v0);
    var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len2;
    getInt32Memory0()[arg0 / 4 + 0] = ptr2;
}, arguments) };
imports.wbg.__wbg_key_d26370b28fd78f46 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg1).key(arg2 >>> 0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_setItem_fe04f524052a3839 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setItem(v0, v1);
}, arguments) };
imports.wbg.__wbg_wasClean_f80e269a7035a33a = function(arg0) {
    const ret = getObject(arg0).wasClean;
    return ret;
};
imports.wbg.__wbg_code_8a4e6c814f55b956 = function(arg0) {
    const ret = getObject(arg0).code;
    return ret;
};
imports.wbg.__wbg_reason_38737e2df79b7571 = function(arg0, arg1) {
    const ret = getObject(arg1).reason;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_newwitheventinitdict_ceffda7854c21a6b = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new CloseEvent(v0, getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_target_6efb4504c149139f = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_cancelBubble_ee3f70328e901584 = function(arg0) {
    const ret = getObject(arg0).cancelBubble;
    return ret;
};
imports.wbg.__wbg_composedPath_ee37eece046b69a2 = function(arg0) {
    const ret = getObject(arg0).composedPath();
    return addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_9299867e06da6909 = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_serviceWorker_7b169f924cee9c9e = function(arg0) {
    const ret = getObject(arg0).serviceWorker;
    return addHeapObject(ret);
};
imports.wbg.__wbg_parentNode_e1c214fc3f362af0 = function(arg0) {
    const ret = getObject(arg0).parentNode;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_childNodes_e62200b977bdb1a0 = function(arg0) {
    const ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
};
imports.wbg.__wbg_previousSibling_de863aa81066e2fe = function(arg0) {
    const ret = getObject(arg0).previousSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_nextSibling_d029031876ed1b1b = function(arg0) {
    const ret = getObject(arg0).nextSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_settextContent_57c7c19d2b0e7614 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_appendChild_2e6a6c9d1f0d443d = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_cloneNode_df9cbf9711e3707d = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).cloneNode();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_removeChild_a63022ebbfa6ebf5 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_length_eae3bc233f10d60d = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_newwithstrandinit_29038da14d09e330 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Request(v0, getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Response_944e2745b5db71f5 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Response;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_url_1f609e63ff1a7983 = function(arg0, arg1) {
    const ret = getObject(arg1).url;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_status_7841bb47be2a8f16 = function(arg0) {
    const ret = getObject(arg0).status;
    return ret;
};
imports.wbg.__wbg_headers_ea7ef583d1564b08 = function(arg0) {
    const ret = getObject(arg0).headers;
    return addHeapObject(ret);
};
imports.wbg.__wbg_arrayBuffer_e32d72b052ba31d7 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_text_39a6fb98be736e16 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).text();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_clearTimeout_76877dbc010e786d = function(arg0) {
    const ret = clearTimeout(takeObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_setTimeout_75cb9b6991a4031d = function() { return handleError(function (arg0, arg1) {
    const ret = setTimeout(getObject(arg0), arg1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};
imports.wbg.__wbg_crypto_c48a774b022d20ac = function(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_process_298734cf255a885d = function(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};
imports.wbg.__wbg_versions_e2e78e134e3e5d01 = function(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};
imports.wbg.__wbg_node_1cd7a5d853dbea79 = function(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};
imports.wbg.__wbg_require_8f08ceecec0f4fee = function() { return handleError(function () {
    const ret = module.require;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_msCrypto_bcb970640f50a1e8 = function(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_randomFillSync_dc1e9a60c158336d = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).randomFillSync(takeObject(arg1));
}, arguments) };
imports.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_next_15da6a3df9290720 = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};
imports.wbg.__wbg_value_0570714ff7d75f35 = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
};
imports.wbg.__wbg_iterator_7ee1a391d310f8e4 = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_c728d68b8b34487e = function() {
    const ret = new Object();
    return addHeapObject(ret);
};
imports.wbg.__wbg_self_3fad056edded10bd = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_a4f46c98a61d4089 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_17eff828815f7d84 = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_46f939f6541643c5 = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_get_4a9aa5157afeb382 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_ArrayBuffer_c7cc317e5c29cc0d = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ArrayBuffer;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_instanceof_Error_9f5881c3c4149389 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Error;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_message_35f9b952e1b922e2 = function(arg0) {
    const ret = getObject(arg0).message;
    return addHeapObject(ret);
};
imports.wbg.__wbg_name_e1152a59269f79e5 = function(arg0) {
    const ret = getObject(arg0).name;
    return addHeapObject(ret);
};
imports.wbg.__wbg_toString_d0cefe4046ecb265 = function(arg0) {
    const ret = getObject(arg0).toString();
    return addHeapObject(ret);
};
imports.wbg.__wbg_newnoargs_ccdcae30fd002262 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_669127b9d730c650 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_call_53fc3abd42e24ec8 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_next_1989a20442400aaa = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).next();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_done_bc26bf4ada718266 = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
};
imports.wbg.__wbg_getTime_ed6ee333b702f8fc = function(arg0) {
    const ret = getObject(arg0).getTime();
    return ret;
};
imports.wbg.__wbg_new0_ad75dd38f92424e2 = function() {
    const ret = new Date();
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_Object_3c95bd459efa5c3c = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Object;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_is_c74aa9bb973d6109 = function(arg0, arg1) {
    const ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
};
imports.wbg.__wbg_resolve_a3252b2860f0a09e = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_89e1c559530b85cf = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_1bbc9edafd859b06 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_buffer_344d9b41efe96da7 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbg_newwithbyteoffsetandlength_2dc04d99088b15e3 = function(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_d8a000788389a31e = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_newwithlength_13b5319ab422dcf6 = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_subarray_6ca5cfa7fbb9abbe = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_a5587d6cd79ab197 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_set_dcfd613a3420f908 = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_stringify_4039297315a25b00 = function() { return handleError(function (arg0) {
    const ret = JSON.stringify(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_get_2aff440840bb6202 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_has_cdf8b85f6e903c80 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.has(getObject(arg0), getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_set_40f7786a25a9cc7e = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper798 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 711, __wbg_adapter_32);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper800 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 711, __wbg_adapter_35);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper802 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 711, __wbg_adapter_32);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper804 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 711, __wbg_adapter_32);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper8100 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 2208, __wbg_adapter_42);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper8332 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 2227, __wbg_adapter_45);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper20753 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 4927, __wbg_adapter_48);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper20755 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 4927, __wbg_adapter_48);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper20757 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 4927, __wbg_adapter_48);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper20759 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 4927, __wbg_adapter_55);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper23281 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 5140, __wbg_adapter_58);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper23385 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 5160, __wbg_adapter_61);
    return addHeapObject(ret);
};

return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('fedimint-leptos-test-b64c863edfa89ad4_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
