<template>
    <own-page centered title="Demo">
        <div class="flex justify-between q-mt-md">
            <q-card bordered flat>
                <q-card-section>
                    <div class="text-h6">Send ping</div>
                </q-card-section>
                <q-card-actions>
                    <q-btn color="primary" label="Send" @click="sendPing"></q-btn>
                </q-card-actions>
            </q-card>
            <q-card bordered flat>
                <q-card-section>
                    <div class="text-h6">Send echo</div>
                </q-card-section>
                <q-card-section>
                    <q-input outlined v-model="echoValue"/>
                </q-card-section>
                <q-card-actions>
                    <q-btn color="primary" label="Send" @click="sendEcho"></q-btn>
                </q-card-actions>
            </q-card>
            <q-card bordered flat>
                <q-card-section>
                    <div class="text-h6">Send AskMe</div>
                </q-card-section>
                <q-card-section>
                    <div class="text-body1">Check the result in core's logs</div>
                </q-card-section>
                <q-card-section>
                    <q-input outlined v-model="askMeValue"/>
                </q-card-section>
                <q-card-actions>
                    <q-btn color="primary" label="Send" @click="sendAskMe"></q-btn>
                </q-card-actions>
            </q-card>
        </div>
    </own-page>
</template>

<script lang="ts">
import {Component, Vue} from "vue-property-decorator";
import {http, WebsocketScope} from "src/plugins/http-commons";
import OwnPage from "components/OwnPage.vue";
import {WsMethods} from "src/types/api/WebsocketMethods";

@Component({
    components: {
        OwnPage,
    },
})
export default class SettingsPage extends Vue {
    wsScope: WebsocketScope = null;
    treeNodes = [{
        key: "global",
        label: "Global",
    }, {
        key: "view",
        label: "View",
    }];
    selected = this.treeNodes[0].key;

    echoValue: string = "";
    askMeValue: string = "";

    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    async sendPing() {
        let response = await http.websocket.send(WsMethods.msg.ping, undefined, this.wsScope);

        this.$q.notify({
            type: "positive",
            caption: response + " received from server",
        });
    }

    async sendEcho() {
        let response = await http.websocket.send(WsMethods.msg.echo, this.echoValue, this.wsScope);

        this.$q.notify({
            type: "positive",
            caption: response + " received from server",
        });
    }

    async sendAskMe() {
        http.websocket.onRequest(WsMethods.msg.echo, (request) => {
            http.websocket.sendResponse(WsMethods.msg.echo, request.id, request.params);
        }, this.wsScope);

        await http.websocket.sendNotification(WsMethods.msg.askMe, this.askMeValue);
    }

    // HOOKS ------------------------------------------------------------------

    created() {
        this.wsScope = http.websocket.scopeManager.addScope("demo");
    }

    destroyed() {
        http.websocket.scopeManager.deleteScope(this.wsScope!!);
    }
}

</script>