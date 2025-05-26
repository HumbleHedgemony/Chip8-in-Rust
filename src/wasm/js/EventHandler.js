import {attachEvent} from "./miscellaneous";

class EventHandler {
    dataHandler;
    element;

    constructor(element, dataHandler) {
        this.element = element;
        this.dataHandler = dataHandler;
    }

    recieve(event) {
        this.dataHandler.handle(event.data);
    }

    send(data) {
        this.element.postMessage(data);
    }
}

class EventHandlerJson {
    eventHandler;
    dataHandler;
    constructor(element, dataHandler) {
        this.eventHandler = EventHandler(element, dataHandler);
    }

    recieve(event) {
        var data = JSON.parse(event.data);
        event.data = data;
        this.eventHandler.recieve(event);
    }

    send(data) {
        var json = JSON.stringify(data);
        this.eventHandler.send(json);
    }
}


class EventHandlerBuilder {

    onlyStrings;
    eventHandler;
    listener;
    constructor(dataHandler) {
        this.post_message_allows_only_strings();
    }

    post_message_allows_only_strings() {
        try{window.postMessage({toString:function(){this.onlyStrings=true;}},"*");}catch(e){}
    }

    set_event_handler(postElement,dataHandler) {
        if (onlyStrings) {
            this.eventHandler = EventHandlerJson(postElement,dataHandler);
        } else {
            eventHandler = EventHandler(postElement, dataHandler);
        }
    }

    set_event(element, event) {
        this.listener = attachEvent(element,event,this.eventHandler.recieve);
    }

    get_event_handler_without_listener(postElement,dataHandler) {
        this.set_event_handler(postElement,dataHandler);
        return this.eventHandler;
    }

    get_event_handler (postElement, element, event) {
        this.set_event_handler(postElement,dataHandler);
        this.set_event(element,event);
        return this.eventHandler;
    }
}


export{EventHandlerBuilder}

