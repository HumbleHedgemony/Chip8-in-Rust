import { attachEvent } from "./miscellaneous";

class OnResize {
    resize_arr = [];

    constructor () {
    }

    set_adapters(arrayOfAdapters) {
        for (let i = 0; i < arrayOfAdapters.length;i++) {
            this.resize_arr.push(arrayOfAdapters[i]);
        }
    }
    set_adapter(resizeClass) {
        this.resize_arr.push(resizeClass);
    }
    set_event() {
        attachEvent(window,"resize",this.resize);
    }

    resize() {
        for( let i in this.resize_arr) {
            i.resize();
        }
    }
}

class ResizeKeyboard {
    keyboard;

    constructor(keyboard) {
        this.keyboard = keyboard;
    }

    resize() {

    }
}

class ResizeDisplay {
    display;

    constructor(display) {
        this.display = display;
    }

    resize() {
        this.display.get_parent_inner_dimensions(parentObject);
        this.display.calculate_display(display.parentWidth,display.parentHeight);
        this.display.print_screen(display.frame);
    }
}

class ResizeAdapterFactory {


    display(displayClass) {

        return ResizeDisplay(displayClass);
    }

    keyboard(keyboardClass) {
        return ResizeKeyboard(keyboardClass);
    }

}


export {ResizeAdapterFactory,OnResize}