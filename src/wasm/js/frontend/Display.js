import { attachEvent, innerDimensions} from "../miscellaneous";



const displayWidth = 64;
const displayHeight  = 32;


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

    parentObject;
    frame;

    constructor(parentObject) {
        this.get_parent_inner_dimensions(parentObject);
        this.calculate_display(this.parentWidth,this.parentHeight);
    }

    get_parent_inner_dimensions() {
        let dim = innerDimensions(this.parentObject);
        this.parentWidth = dim.width;
        this.parentHeight = dim.height;
    }

    get_canvas() {
        return this.canvas;
    }

    calculate_dimensions(destWidth, destHeight) {
        const scale = Math.min(this.parentWidth/displayWidth,this.parentHeight/displayHeight);

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
        this.frame = array;

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

export {Display};
