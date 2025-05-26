import * as wasm from './js_hello_world_bg';

export class Foo {
    static __construct(ptr) {
        return new Foo(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_WasmChip8_free(ptr);
    }

    static new() {
        const ret = wasm.WasmChip8_new();
        return Foo.__construct(ret)
    }

    get() {
        const ret = wasm.WasmChip8_get(this.ptr);
        return ret;
    }

    tick() {
        const ret = wasm.WasmChip8_tick(this.ptr);
        return ret;
    }


    get_display() {
        const ret = wasm.WasmChip8_get_display(this.ptr);
        return ret;
    }

    set_key(key) {
        const ret = wasm.WasmChip8_set_key(this.ptr, key);
        return ret;
    }

    set_super() {
        const ret = wasm.WasmChip8_set_super(this.ptr);
        return ret;
    }

    set_cosmac() {
        const ret = wasm.WasmChip8_set_cosmac(this.ptr);
        return ret;
    }

    get_sound() {
        const ret = wasm.WasmChip8_get_sound(this.ptr);
        return ret;
    }

    get_delay() {
        const ret = wasm.WasmChip8_get_delay(this.ptr);
        return ret;
    } 

}