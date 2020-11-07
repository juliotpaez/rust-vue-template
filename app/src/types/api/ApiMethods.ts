import {VersionApiType} from "src/types/api/ApiTypes";

export class ApiMethod<P, R> {
    method: string;
    path: string;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor(method: "GET" | "POST", path: string) {
        this.method = method;
        this.path = path;
    }
}

export const ApiMethods = {
    version: new ApiMethod<undefined, VersionApiType>("GET", "/version"),
};