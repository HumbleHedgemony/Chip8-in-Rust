import { attachEvent } from "./miscellaneous";
import { EventHandlerBuilder } from "./EventHandler";
import { DataFactory } from "./frontend/DataHandler";
import { Sound } from "./frontend/Sound";
import { Display } from "./frontend/Display";
import { Keyboard } from "./frontend/Keyboard";
/**
 * 
 * 
 * have to implement web worker for this script so it runs on a separate thread.
 */





    class Chip8 {
        gameLoop;
        display;
        keyboard;
        sound;
        wasmChip8;
        dataFactory;


        constructor(displayWidth,displayHeight, keyboardWidth, keyboardHeight) {

            this.sound = Sound();
            this.display = Display(displayWidth,displayHeight);
            this.dataFactory = DataFactory(this.display,this.sound);
            this.keyboard = Keyboard(this.dataFactory, keyboardWidth, keyboardHeight);

        }

        

        get_sound() {
            return this.sound;
        }

        get_display() {
            return this.display;
        }

        get_keyboard() {
            return this.keyboard;
        }

        set_cosmac() {
            this.wasmChip8.set_cosmac();
        }

        set_super() {
            this.wasmChip8.set_super();
        }

    }


    
    
export {Chip8};