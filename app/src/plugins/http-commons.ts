import axios, {AxiosInstance, AxiosPromise} from "axios";
import {
    WebsocketNotificationMethod, WebsocketRequestMethod, WebsocketSystemMethod, WsMethods,
} from "../types/api/WebsocketMethods";
import {
    WebsocketError, WebsocketMessage, WebsocketNotification, WebsocketRequest, WebsocketResponse,
} from "../types/api/WebsocketTypes";
import {ApiMethod} from "src/types/api/ApiMethods";

class AxiosManager {
    axios: AxiosInstance | null;

    // GETTERS ----------------------------------------------------------------

    get isNull() {
        return this.axios === null;
    }

    get baseURL() {
        return this.axios!!.defaults.baseURL;
    }

    // CONSTRUCTORS -----------------------------------------------------------

    constructor() {
        this.axios = null;
    }

    // METHODS ----------------------------------------------------------------

    init(location: string) {
        this.axios = axios.create({
            baseURL: `http://${location}`,
            headers: {
                "Content-type": "application/json",
            },
        });
    }

    clear() {
        this.axios = null;
    }

    send<P, R>(method: ApiMethod<P, R>, value: P): AxiosPromise<R> {
        if (method.method === "GET") {
            return this.axios!!.get<R>(method.path);
        } else {
            return this.axios!!.post<R>(method.path, value);
        }
    }
}

class WebsocketManager {
    scopeManager = new ScopeManager();
    nextId: number;
    websocket: WebSocket | null;
    sysHandlers: Map<string, ([WebsocketScope, (x: any) => void])[]>;
    requestHandlers: Map<string, ([WebsocketScope, (x: any) => void])[]>;
    responseHandlers: Map<string, [WebsocketScope, (x: any) => void]>;

    // GETTERS ----------------------------------------------------------------

    get isNull() {
        return this.websocket === null;
    }

    // CONSTRUCTORS -----------------------------------------------------------

    constructor() {
        this.nextId = 0;
        this.websocket = null;
        this.sysHandlers = new Map<string, [WebsocketScope, ((x: any) => void)][]>();
        this.requestHandlers = new Map<string, [WebsocketScope, ((x: any) => void)][]>();
        this.responseHandlers = new Map<string, [WebsocketScope, ((x: any) => void)]>();
    }

    // METHODS ----------------------------------------------------------------

    init(location: string) {
        const socket = new WebSocket(`ws://${location}/ws`);
        socket.onopen = (event) => {
            this.resolveSystemHandlers(WsMethods.sys.open, event);
        };

        socket.onmessage = (event) => {
            let data: WebsocketMessage<any>;
            try {
                data = JSON.parse(event.data);
            } catch (e) {
                console.error("Cannot parse data as JSON", event);
                return;
            }

            switch (data.type) {
                case "req":
                    if (this.requestHandlers.has(data.method)) {
                        this.resolveRequestHandlers(data);
                    } else {
                        this.resolveSystemHandlers(WsMethods.sys.unhandledRequest, data);
                    }
                    break;
                case "res":
                    if (this.responseHandlers.has(data.id)) {
                        this.resolveResponseHandlers(data);
                    } else {
                        this.resolveSystemHandlers(WsMethods.sys.unhandledResponse, data);
                    }
                    break;
                case "err":
                    if (!!data.id && this.responseHandlers.has(data.id)) {
                        this.resolveResponseHandlersWithError(data);
                    } else {
                        this.resolveSystemHandlers(WsMethods.sys.unhandledError, data);
                    }
                    break;
                default:
                    this.resolveSystemHandlers(WsMethods.sys.incorrectMessage, data);
                    break;
            }
        };

        socket.onclose = (event) => {
            this.resolveSystemHandlers(WsMethods.sys.close, event);
            this.clear();
        };

        socket.onerror = (event) => {
            this.resolveSystemHandlers(WsMethods.sys.error, event);
        };

        this.websocket = socket;
    }

    close() {
        this.resolveSystemHandlers(WsMethods.sys.close, new CloseEvent("manual"));
        this.clear();
    }

    clear() {
        this.scopeManager.clear();
        this.websocket?.close();
        this.websocket = null;
        this.sysHandlers.clear();
        this.requestHandlers.clear();
        this.responseHandlers.clear();
    }

    getNextId() {
        return (this.nextId++).toString();
    }

    onSystem<T>(message: WebsocketSystemMethod<T>, method: (value: T) => void, scope: WebsocketScope = null) {
        let handlers = this.sysHandlers.get(message.name) || [];
        handlers.push([scope, method]);

        this.sysHandlers.set(message.name, handlers);
    }

    onRequest<P, R>(message: WebsocketRequestMethod<P, R>, method: (value: WebsocketRequest<P>) => void,
        scope: WebsocketScope = null) {
        let handlers = this.requestHandlers.get(message.name) || [];
        handlers.push([scope, method]);

        this.requestHandlers.set(message.name, handlers);
    }

    send<S, R>(method: WebsocketRequestMethod<S, R>, message: S, scope: WebsocketScope = null): Promise<R> {
        let id = this.getNextId();
        let request: WebsocketRequest<S> = {
            type: "req",
            id: id,
            method: method.name,
            params: message,
        };

        let data = JSON.stringify(request);
        this.websocket!!.send(data);

        return new Promise<R>((resolve) => {
            this.responseHandlers.set(id, [scope, resolve]);
        });
    }

    sendNotification<S>(method: WebsocketNotificationMethod<S>, message: S) {
        let notification: WebsocketNotification<S> = {
            type: "not",
            method: method.name,
            params: message,
        };

        let data = JSON.stringify(notification);
        this.websocket!!.send(data);
    }

    sendResponse<R>(method: WebsocketRequestMethod<any, R>, id: string, result: R) {
        let response: WebsocketResponse<R> = {
            id,
            type: "res",
            result,
        };

        let data = JSON.stringify(response);
        this.websocket!!.send(data);
    }

    private resolveSystemHandlers<T>(message: WebsocketSystemMethod<T>, value: T) {
        let handlers = this.sysHandlers.get(message.name) || [];
        for (let [scope, handler] of handlers) {
            if (this.scopeManager.contains(scope)) {
                handler(value);
            }
        }
    }

    private resolveRequestHandlers(request: WebsocketRequest<any>) {
        let handlers = this.requestHandlers.get(request.method) || [];
        for (let [scope, handler] of handlers) {
            if (this.scopeManager.contains(scope)) {
                handler(request);
            }
        }
    }

    private resolveResponseHandlers(response: WebsocketResponse<any>) {
        let responseHandler = this.responseHandlers.get(response.id);
        if (!responseHandler) {
            return;
        }

        let [scope, handler] = responseHandler;
        if (this.scopeManager.contains(scope)) {
            handler(response.result);
        }

        this.responseHandlers.delete(response.id);
    }

    private resolveResponseHandlersWithError(response: WebsocketError) {
        if (response.id == null) {
            return;
        }

        let responseHandler = this.responseHandlers.get(response.id);
        if (!responseHandler) {
            return;
        }

        let [scope, handler] = responseHandler;
        if (this.scopeManager.contains(scope)) {
            handler(response);
        }

        this.responseHandlers.delete(response.id);
    }
}

export type WebsocketScope = string | null;

class ScopeManager {
    scopeCounter = 0;
    scopes: Set<string>;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor() {
        this.scopes = new Set<string>();
    }

    // METHODS ----------------------------------------------------------------

    getNextId() {
        return (this.scopeCounter++).toString();
    }

    contains(scope: WebsocketScope): boolean {
        if (scope === null) {
            return true;
        }

        return this.scopes.has(scope);
    }

    addScope(prefix: string): string {
        let scope = prefix + "-" + this.getNextId();
        this.scopes.add(scope);
        return scope;
    }

    deleteScope(name: string) {
        this.scopes.delete(name);
    }

    clear() {
        this.scopes.clear();
    }
}

export const http = {
    axios: new AxiosManager(),
    websocket: new WebsocketManager(),
};