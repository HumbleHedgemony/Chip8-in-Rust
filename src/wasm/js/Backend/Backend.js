import { attachEvent} from "./miscellaneous";
import { EventHandlerBuilder } from "./EventHandler";
import  { Chip8 } from "./frontend"

/*
 *
 * worker classes
 * 
 * 
 */

let eventHandler;
let eventHandlerBuilder;
let workerWasmAdaptor;
let workerChip8;
var chip8;



class WorkerDisplay {
    wasmChip8Adaptor;
    array;
    constructor (wasmChip8Adaptor) {
        this.wasmChip8Adaptor = wasmChip8;
    }

    update() {
        array = this.wasmChip8Adaptor.get_display();
        eventHandler.send({'display':{'array':this.array}});
    }

}

class WorkerGame {
    wasmChip8;


    constructor() {

    }

    data(data) {
        switch (true) {
            case data.hasOwnProperty("array"):
                this.write_rom(data.array);
            break;
        }
    }

    write_rom(array) {
        let encoder = TextEncoder();
        let uint8 = encoder.encode(array);
        this.wasmChip8.write_rom(uint8);
    }

}



class WorkerKeyboard {
    wasmChip8;


    constructor() {

    }

    data(data) {
        switch (true) {
            case data.hasOwnProperty("button"):
                this.press(data.button);
            break;
        }
    }

    press(key) {
        this.wasmChip8.press(key);
    }
}

class WorkerSound {
    wasmChip8Adaptor;
    timer;

    constructor(wasmChip8Adaptor) {
        this.wasmChip8Adaptor = wasmChip8Adaptor;
    }

    update() {
        let currentTimer = this.wasmChip8.get_sound();
        if (this.timer != currentTimer) {
            eventHandler.send({'sound':{'timer':currentTimer}});
        }
        this.timer = currentTimer;
    }
}

class WorkerChip8 {
    gameLoop;
    display;
    keyboard;
    sound;
    workerWasmAdaptor;
    rom;

    constructor(workerWasmAdaptor) {
        

        this.keyboard = new WorkerKeyboard();
        this.workerWasmAdaptor = workerWasmAdaptor;
        this.sound = new WorkerSound(workerWasmAdaptor);
        this.display = new WorkerDisplay(workerWasmAdaptor);
        this.rom = WorkerGame(workerWasmAdaptor);
        this.gameLoop = new GameLoop(workerWasmAdaptor,this.display, this.sound);
        
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

    get_game() {
        return this.rom;
    }

    get_game_loop() {
        return this.gameLoop;
    }



}

class WorkerWasmAdaptor {

    wasmChip8;

    constructor(wasmChip8) {
        this.wasmChip8 = wasmChip8;
    }

    get_sound() {
        return this.wasmChip8.get_sound();
    }

    get_display() {
        return this.wasmChip8.get_display();
    }

    press(key) {
        this.wasmChip8.set_key(key);
    }

    tick() {
        this.wasmChip8.tick();
    }

    write_rom(array) {
        this.wasmChip8.write_rom(array);
        
    }

}




    
    
    class DataHandler {
        workerChip8;

        constructor(workerChip8) {
            this.workerChip8 = workerChip8;
        }

        handle(data) {
            switch (true) {
    
                /*
                case event.data.hasOwnProperty("sound"):
                    /* do nothing 
                continue;*/
        
                case data.hasOwnProperty("keyboard"):
                    workerChip8.get_keyboard().data(data.keyboard);
                break;

                case data.hasOwnProperty("game"): 
                    workerChip8.get_game().data(data.game);
                break;
        
                default:
                    break;
        
            }
        }
    
    }
    chip8 = Chip8()
    workerWasmAdaptor = WorkerWasmAdaptor();
    workerChip8 = WorkerChip8(workerWasmAdaptor);
    dataHandler = DataHandler(workerChip8);
    eventHandlerBuilder = EventHandlerBuilder(window,dataHandler);
    eventHandler = eventHandlerBuilder.get_event_handler(window,"message");
    






