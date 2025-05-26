
const ticksPerSecond = 700;
const framesPerSecond = 60;
const millisPerFrame = 1000 / framesPerSecond;
const millisPerTick = 1000 / ticksPerSecond;



class GameLoop {
    lastTimeStamp;
    nextTick;
    wasmChip8;
    interval;

    
    constructor (wasmChip8,display,sound) {
        this.WasmChip8 = wasmChip8;
        this.display = display;
        this.sound = sound;
    }



    start() {
        this.interval = setInterval(this.tick,millisPerTick);
    }

    pause() {
        clearInterval(this.interval);
    }

    tick() {
        this.WasmChip8.tick();

        // update display 60fps
        var now = Date.now();
        if (now - this.lastTimeStamp > millisPerFrame) {
            this.display.update();
            this.sound.update();
            this.lastTimeStamp = now;
        }
    }

}

export {GameLoop}