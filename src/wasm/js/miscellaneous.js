

    var attachEvent = function (element, event, callbackFunction) {
        if (element.addEventListener) {
            element.addEventListener(event, callbackFunction, false);
        } else if (element.attachEvent) {
            element.attachEvent('on' + event, callbackFunction);
        }
    };


    const innerDimensions = (node) => {

        function old_browser() {
            let width = parseFloat(node.currentStyle.width);
            let height = parseFloat(node.currentStyle.height);
            return { width, height };
        }
    
        function new_browser() {
            var computedStyle = getComputedStyle(node)
      
            let width = node.clientWidth // width with padding
            let height = node.clientHeight // height with padding
          
            height -= parseFloat(computedStyle.paddingTop) + parseFloat(computedStyle.paddingBottom)
            width -= parseFloat(computedStyle.paddingLeft) + parseFloat(computedStyle.paddingRight)
            return { width, height }
        }
    
        if (!getComputedStyle) {
            return old_browser();
        } else {
            return new_browser();
        }
    
    
      }
    

   export {attachEvent,innerDimensions};
