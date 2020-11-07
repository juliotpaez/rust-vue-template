import {WebsocketError, WebsocketMessage, WebsocketRequest, WebsocketResponse} from "src/types/api/WebsocketTypes";

export class WebsocketSystemMethod<T> {
    name: string;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor(name: string) {
        this.name = name;
    }
}

export class WebsocketRequestMethod<P, R> {
    name: string;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor(name: string) {
        this.name = name;
    }
}

export class WebsocketNotificationMethod<P> {
    name: string;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor(name: string) {
        this.name = name;
    }
}

export const WsMethods = {
    sys: {
        open: new WebsocketSystemMethod<Event>("open"),
        close: new WebsocketSystemMethod<CloseEvent>("close"),
        error: new WebsocketSystemMethod<Event>("error"),
        unhandledRequest: new WebsocketSystemMethod<WebsocketRequest<any>>("unhandled_request"),
        unhandledResponse: new WebsocketSystemMethod<WebsocketResponse<any>>("unhandled_response"),
        unhandledError: new WebsocketSystemMethod<WebsocketError>("unhandled_error"),
        incorrectMessage: new WebsocketSystemMethod<WebsocketMessage<any>>("incorrect_message"),
    },
    msg: {
        ping: new WebsocketRequestMethod<undefined, string>("ping"),
        echo: new WebsocketRequestMethod<string, string>("echo"),
        shutdown: new WebsocketRequestMethod<undefined, string>("shutdown"),
        askMe: new WebsocketNotificationMethod<string>("askMe"),
    },
};