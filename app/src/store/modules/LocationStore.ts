import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {VersionApiType} from "src/types/api/ApiTypes";
import {http} from "src/plugins/http-commons";
import store from "../";

// https://github.com/championswimmer/vuex-module-decorators

@Module({
    name: "location",
    store,
    dynamic: true,
})
export default class LocationStore extends VuexModule {
    tryLogin: boolean = true;
    redirect: string | null = null;
    ip: string | null = null;
    port: string | null = null;
    version: VersionApiType | null = null;

    // GETTERS ----------------------------------------------------------------

    get location() {
        return this.ip + ":" + this.port;
    }

    // MUTATIONS --------------------------------------------------------------

    @Mutation setTryLogin(data: boolean) {
        this.tryLogin = data;
    }

    @Mutation setRedirect(data: string | null) {
        this.redirect = data;
    }

    @Mutation setLocation(data: { ip: string, port: string }) {
        this.ip = data.ip;
        this.port = data.port;
    }

    @Mutation clearLocation() {
        this.ip = null;
        this.port = null;
    }

    @Mutation setVersion(version: VersionApiType | null) {
        this.version = version;
    }

    @Mutation clearState() {
        this.ip = null;
        this.port = null;
        this.version = null;
        http.axios.clear();
        http.websocket.clear();
    }

    // ACTIONS ----------------------------------------------------------------

    @Action clearAllState() {
        this.clearState();
    }
}