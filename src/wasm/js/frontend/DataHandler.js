import { EventHandlerBuilder } from "../EventHandler";



class GameData {
    data="game"



    handle(data) {
        for (const prop in data) {
            switch(prop) {
                case "rom": 
                    this.rom(data[prop]);
                break;

                default:
                    break;

            }
        }
    }

    rom(data) {

    }

}

class SoundData {
    data = "sound";
    sound;
    subData = [];

    constructor(sound) {
        this.sound = sound;
    }

    handle(data){
        for (const prop in data) {
            switch(prop) {
                case "action": 
                    this.action(data[prop]);
                break;

                default:
                    break;

            }
        }
    }

    action(value) {
        switch (value) {
            case "play":
                this.sound.play();
            break;

            case "pause": 
                this.sound.pause();
            break;

            case "stop": 
                this.sound.stop();
                this.sound.reset();
            break;

            default:
                break;
        }
    }
    
}

class DisplayData {
    data = "display";
    display;

    constructor(display) {
        this.display = display;
    }

    handle(data) {
        for (const prop in data) {
            switch(prop) {
                case "array": 
                    this.array(data[prop]);
                break;

                default:
                    break;
                    
            }
        }
    }

    array(array) {

        this.display.print_screen(array);

    }


}


class DataHandler {
    handlers = [DisplayData(),SoundData()];
    handler;
    constructor(display, sound) {
        handlers = [DisplayData(display),SoundData(sound)];
    }

    handle(data) {

        for (var i = 0; i < this.handlers.length;i++) {
            this.handler = this.handlers[i];
            if (data.hasOwnProperty(this.handler.data)) {
                this.handler.handle(data[this.handler.data]);
            }
        }

    }

}


  

class DataFactory {
    eventHandlerBuilder;

    dataHandler;
    eventHandler;
    constructor(backendWorker) {
        this.dataHandler = DataHandler();
        this.eventHandlerBuilder = EventHandlerBuilder(this.dataHandler);
        this.eventHandler = this.eventHandlerBuilder.get_event_handler(backendWorker,backendWorker,"message");
    }

    rom(romData) {
        let data = {"game": {"rom":romdata}};
        this.eventHandler.send(data);
    }

    cosmac() {
        let data = {"game": {"init":"cosmac"}};
        this.eventHandler.send(data);
    }

    super() {
        let data = {"game": {"init":"super"}};
        this.eventHandler.send(data);
    }

    start() {
        let data = {"game":{"tick":"start"}};
        this.eventHandler.send(data);
    }

    stop() {
        let data = {"game":{"tick":"stop"}};
        this.eventHandler.send(data);
    }


    keyboard(buttonPressed) {
        let data = {"keyboard":{"button":buttonPressed}};
        this.eventHandler.send(data);
    }
}


export {DataFactory}