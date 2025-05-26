import { attachEvent } from "../miscellaneous";

class KeyEvent {

    func;
    constructor(func) {
        this.func = func;
    }

    keydown(event) {
        var key = event.key;
        var val = parseInt("0x"+key);
        if (val != NaN && this.in_range(val,0,15)) {
            this.func(val);
        }
    }
}

class KeyCodeEvent {

    func;

    constructor(func) {
        this.func = func;
    }

    keydown(event) {
        var keycode = event.keycode;
        switch(true) {
            case this.in_range(keycode,48,57):
                    this.func(keycode-48);
                break;

            case this.in_range(keycode,65,70):
                this.func(keycode-55);
                break;

            default:
                break;
        }
    }
}

class KeyDownEvent {

    keyEvent;
    constructor(func) {
        this.keyboard_event(func);
        this.set_key_down();

    }


    set_key_down() {
        misc.attachEvent(window,"keydown", this.keyEvent.keydown);
    }
    
    keyboard_event(func) {
        var event = new KeyboardEvent("keydown");
        if (event.hasOwnProperty("key")) {
            this.keyEvent = new KeyCodeEvent(func);
        } else {
            this.keyEvent = new KeyEvent(func);
        }
    }

}


class Keyboard {
    width;
    height;

    buttonWidth;
    buttonHeight;
    buttonMargin;
    marginPercentage = .2;

    dataFactory
    onkeydown;


    constructor(dataFactory, width,height) {
        this.width = width;
        this.height = height;
        this.calculate_button_dimensions();
        this.onkeydown = new KeyDownEvent(this.press);
        this.dataFactory = dataFactory;
    }


    calculate_button_dimensions() {
        
        this.buttonMargin = (this.width/4)*this.marginPercentage;
        this.buttonWidth = (this.width/4)*(1-this.marginPercentage);
        this.buttonHeight = this.buttonWidth;
    }

    in_range (value,lower,upper) {
        return (value >= lower && value <= upper);
    }
    


    press(key) {
        this.dataFactory.press(key);
    }



    create_button_element(number){
        var button = document.createElement("span");
        misc.attachEvent(button, "click", function(){ this.press(number)} );
        button.innerHTML = number;
        button.style="text-align:center; height:"+ buttonHeight+ "px; line-height:" +buttonHeight +"px";
        return button;
    }

}

export{Keyboard};