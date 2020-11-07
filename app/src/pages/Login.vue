<template>
    <q-layout view="hHh lpR fFf">
        <q-page-container>
            <q-page class="row items-center justify-evenly container">
                <q-card bordered class="q-pa-md" style="width: 450px">
                    <p class="q-mb-sm text-h6 text-center">Core Location</p>
                    <div class="row">
                        <div class="col q-pr-sm">
                            <q-input v-model="ip"
                                     :dense="true"
                                     :disable="loading"
                                     :error="ipErrorMessage !== null"
                                     :error-message="ipErrorMessage"
                                     bottom-slots
                                     class="q-mb-sm"
                                     label="Location"
                                     outlined/>
                        </div>
                        <div class="col-3">
                            <q-input v-model.number="port"
                                     :dense="true"
                                     :disable="loading"
                                     :error="portErrorMessage !== null"
                                     :error-message="portErrorMessage"
                                     bottom-slots
                                     class="q-mb-sm"
                                     label="Port"
                                     outlined
                                     type="number"/>
                        </div>
                    </div>
                    <div class="row justify-center">
                        <q-btn :loading="loading" color="primary" label="Access" @click="access(true)"/>
                    </div>
                </q-card>
            </q-page>
        </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import {Component, Vue} from "vue-property-decorator";
import {getModule} from "vuex-module-decorators";
import LocationStore from "src/store/modules/LocationStore";
import {http} from "src/plugins/http-commons";
import {WsMethods} from "src/types/api/WebsocketMethods";
import {ApiMethods} from "src/types/api/ApiMethods";

const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$/;
const domainRegex = /^(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)*[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$/;

@Component
export default class LoginPage extends Vue {
    ip = "localhost";
    port = 21012;
    loading = false;

    // COMPUTED -----------------------------------------------------------------

    get location() {
        return this.ip + ":" + this.port;
    }

    get ipErrorMessage() {
        if (this.ip.trim().length <= 0) {
            return "Cannot be empty";
        }

        if (domainRegex.test(this.ip)) {
            return null;
        }

        if (!ipRegex.test(this.ip)) {
            return "Do not match a domain or IP pattern e.g. 127.0.0.1";
        }

        return null;
    }

    get portErrorMessage(): string | null {
        if (!this.port && this.port !== 0) {
            return "Cannot be empty";
        }

        if (this.port < 0) {
            return "Cannot be lower than 0";
        }

        if (this.port > 65535) {
            return "Cannot be greater than 65535";
        }

        return null;
    }

    // METHODS ------------------------------------------------------------------

    async access() {
        this.loading = true;

        if (this.ipErrorMessage !== null || this.portErrorMessage !== null) {
            this.loading = false;
            return;
        }

        // Save in store.
        let store = getModule(LocationStore);
        store.setLocation({
            ip: this.ip,
            port: this.port.toString(),
        });

        // Create axios instance.
        let location = store.location;
        http.axios.init(location);

        // Get info.
        try {
            let response = await http.axios.send(ApiMethods.version, undefined);
            store.setVersion(response.data);
        } catch (e) {
            console.error("Cannot reach '" + http.axios.baseURL + "'", e);

            this.$q.notify({
                type: "negative",
                message: "Cannot reach '" + this.location + "'",
                classes: "text-no-wrap",
            });

            store.clearAllState();
            this.loading = false;
            return;
        }

        // Connect to ws.
        http.websocket.init(location);

        http.websocket.onSystem(WsMethods.sys.open, async () => {
            console.info("Connection established!");

            if (store.redirect === null) {
                this.$router.push({name: "Demo"});
            } else {
                let redirect = store.redirect;
                store.setRedirect(null);
                this.$router.push({name: redirect});
            }

            this.loading = false;
        });

        http.websocket.onSystem(WsMethods.sys.close, async () => {
            // Handle cannot open errors.
            if (this.$route.name === "Login") {
                this.$q.notify({
                    type: "negative",
                    message: "Cannot reach '" + this.location + "'",
                    classes: "text-no-wrap",
                });

                store.clearAllState();
                this.loading = false;
                return;
            }

            console.info("Connection closed!");

            store.clearAllState();
            this.$router.push({name: "Login"});
        });

        http.websocket.onSystem(WsMethods.sys.error, (e) => {
            console.info("Error", e);

            // If connection is closed, onClose will be called implicitly.
        });
    }

    // HOOKS --------------------------------------------------------------------

    mounted() {
        let store = getModule(LocationStore);
        if (store.tryLogin) {
            store.setTryLogin(false);
            this.access();
        }
    }
}
</script>

<style scoped>
.container {
    background-image: url("../assets/login_background.jpg");
    background-position: center;
    background-size: cover;
    height: 100%;
}
</style>