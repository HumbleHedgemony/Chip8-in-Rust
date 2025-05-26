// unit testing chip 8 javascript
import { expect, test } from "@jest/globals";

class Display {
    id = "Chip8Display";
    aspectRatio = 2;

    outputWidth = 0;
    outputHeight = 0;

    blankSpaceXAxis;
    blankSpaceYAxis;
    pixelDimensions;

    canvas;

    parentWidth;
    parentHeight;

    constructor(width, height) {
        this.parentWidth = width;
        this.parentHeight = height;
        this.calculate_display(width,height);
    }

    get_canvas() {
        return this.canvas;
    }

    calculate_dimensions(destWidth, destHeight) {
        const scale = Math.min(destWidth/displayWidth,destHeight/displayHeight);

        this.outputWidth = displayWidth*scale;
        this.outputHeight = displayHeight*scale;
        
        this.pixelDimensions = Math.floor(scale);

    }
    

    calculate_blank_space(width, height) {
        this.blankSpaceXAxis = width - this.outputWidth;
        this.blankSpaceYAxis = height - this.outputHeight;
    }

    create_canvas() {
        this.canvas = document.createElement(canvas);
        this.canvas.width = this.outputWidth;
        this.canvas.height = this.outputHeight;
        this.canvas.id = this.id;
    
    }



    draw_blank_space() {

        const xAxis = Math.floor(this.blankSpaceXAxis/2);
        const yAxis = Math.floor(this.blankSpaceYAxis/2);
        this.canvas.style = "margin:"+ yAxis + "px 0 0 " + xAxis+"px";

    }

    print_pixel( imageData, r, g, b, a) {

        for (var j = 0; j < imageData.data.length; j += 4) {
            imageData.data[j] = r;
            imageData.data[j+1] = g;
            imageData.data[j+2] = b;
            imageData.data[j+3] = a;
        }

    }
    print_screen(array) {

        var ctx = this.canvas.getContext("2d");

        for (var y = 0; y < this.displayHeight; y++) {
            for (var x = 0; x < this.displayWidth; x++) {
                var imageData = ctx.createImageData(this.pixelDimensions,this.pixelDimensions);
                var colour = array[x][y] * 255; // either black for turned off or white for turned on
                print_pixel(imageData, colour,colour,colour, 255);
                ctx.putImageData(imageData, x*this.pixelDimensions,y*this.pixelDimensions);
            }
        }

    }



    calculate_display(width, height) {
        this.calculate_dimensions(width,height);
        this.calculate_blank_space(width,height);
        this.create_canvas();
        this.draw_blank_space();

    }




}

var display = new Display(100,100);


test("display_calculate_dimensions", () => {
    var display = new Display(100,100);

    display.calculate_dimensions(100,100);

    expect(display.outputWidth).toBe(100);
    expect(display.outputHeight).toBe(100/64*32);

});


test("display_calculate_blank_space", () =>{
    var display = new Display(100,100);

    display.calculate_dimensions(100,100);
    display.calculate_blank_space(100,100);


    expect(display.blankSpaceXAxis).toBe(0);
    expect(display.blankSpaceYAxis).toBe(100 - display.outputHeight);
    


});

test("display_pixel_dimensions", () =>{

    var display = new Display(100,100);

    display.calculate_dimensions(100,100);

    expect(display.pixelDimensions).toBe(Math.floor(100/64));
    expect(display.pixelDimensions).toBe(Math.floor(display.outputHeight/32));
    expect(display.pixelDimensions).toBe(Math.floor(display.outputWidth/64));

});


test("draw_blank_space",() => {
    var display = new Display(100,100);

    display.calculate_dimensions(100,100);
    display.calculate_blank_space(100,100);
    display.create_canvas();
    display.draw_blank_space();

    var canvas = display.get_canvas();
    var style = canvas.currentStyle || window.getComputedStyle(canvas);

    expect(parseInt(style.marginTop,10)).toBe(Math.floor(display.blankSpaceYAxis/2));
    expect(parseInt(style.marginLeft,10)).toBe(Math.floor(display.blankSpaceXAxis/2));



})



test("display_print_pixel", () =>{
    var display = new Display(100,100);

    display.calculate_dimensions(100,100);
    display.calculate_blank_space(100,100);
    display.create_canvas();

    var canvas = display.get_canvas();
    var imageData = canvas.createImageData(display.pixelDimensions, display.pixelDimensions);
    
    print_pixel(imageData,255,255,255,255);
    length = imageData.data.length;

    canvas.putImageData(imageData,0,0);

    imageData = canvas.getImageData(0,0);

    for (var i = 0; i < length; i+= 4) {
        expect(imageData.data[i]).toBe(255);
        expect(imageData.data[i+1]).toBe(255);
        expect(imageData.data[i+2]).toBe(255);
        expect(imageData.data[i+3]).toBe(255);
    }


});

test("display_print_screen", () => {
    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }

    function createRandomArray() {
        var arr = [];
        for (var y = 0; y < 32; y++) {
            for (var x = 0; x < 64; x++) {
                arr.push(getRandomInt(2));
            }
        }
    }
    var frame = createRandomArray();

    var display = new Display(100,100);

    display.calculate_dimensions(100,100);
    display.calculate_blank_space(100,100);
    display.create_canvas();
    display.print_screen(frame);

    var canvas = display.get_canvas();

    for (var y = 0; y < 32; y++) {
        for (var x = 0; x < 64; x++) {
            imageData = canvas.getImageData(x*display.pixelDimensions,y*display.pixelDimensions);
            for (var i = 0; i < length; i+= 4) {
                expect(imageData.data[i]).toBe(255);
                expect(imageData.data[i+1]).toBe(255);
                expect(imageData.data[i+2]).toBe(255);
                expect(imageData.data[i+3]).toBe(255);
            }

        }

    }
});


test("keyboard_")