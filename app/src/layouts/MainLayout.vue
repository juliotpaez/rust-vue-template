<template>
    <q-layout view="lHh lpR fFf">
        <q-drawer show-if-above
                  v-model="drawer"
                  side="left"
                  :mini="drawerMiniState && !showExitPopup"
                  @mouseover="drawerMiniState = false"
                  @mouseout="drawerMiniState = true"
                  mini-to-overlay
                  bordered
                  :width="200"
                  :breakpoint="0"
                  content-class="flex column">
            <q-list>
                <template v-for="(location, index) in upperLocations">
                    <q-item :key="index"
                            clickable
                            v-ripple
                            :active="location.pathName === $route.name"
                            @click="navigate(location.pathName)">
                        <q-item-section avatar>
                            <q-icon class="q-pa-xs" size="16px" :name="location.icon"/>
                        </q-item-section>
                        <q-item-section>
                            {{ location.label }}
                        </q-item-section>
                    </q-item>
                    <q-separator :key="'sep' + index"/>
                </template>
            </q-list>
            <q-space></q-space>
            <q-list>
                <template v-for="(location, index) in lowerLocations">
                    <q-separator :key="'sep' + index"/>
                    <q-item :key="index"
                            clickable
                            v-ripple
                            :active="location.pathName === $route.name"
                            @click="navigate(location.pathName)">
                        <q-item-section avatar>
                            <q-icon class="q-pa-xs" size="16px" :name="location.icon"/>
                        </q-item-section>
                        <q-item-section>
                            {{ location.label }}
                        </q-item-section>
                    </q-item>
                </template>
                <q-separator/>
                <q-item clickable v-ripple @click="showExitPopup = true">
                    <q-item-section avatar>
                        <q-icon class="q-pa-xs" size="16px" name="fas fa-sign-out-alt"/>
                    </q-item-section>
                    <q-item-section>
                        Leave
                    </q-item-section>
                    <q-popup-edit @show="showExitPopup = true"
                                  @hide="showExitPopup = false"
                                  v-model="showExitPopup"
                                  content-class="flex q-pa-sm justify-center items-center">
                        <div>
                            <q-btn color="warning" text-color="dark" size="sm" label="Close UI" @click="close_view"/>
                        </div>
                        <div>
                            <q-btn color="negative" size="sm" label="Shutdown" class="q-ml-sm" @click="shutdown_app"/>
                        </div>
                    </q-popup-edit>
                </q-item>
            </q-list>
        </q-drawer>

        <q-page-container>
            <router-view/>
        </q-page-container>

        <q-footer bordered class="q-px-md text-caption bg-dark own-footer">
            A Vue template
        </q-footer>
    </q-layout>
</template>

<script lang="ts">
import {Component, Vue} from "vue-property-decorator";
import {http, WebsocketScope} from "src/plugins/http-commons";
import {WsMethods} from "src/types/api/WebsocketMethods";

@Component({})
export default class MainLayout extends Vue {
    wsScope: WebsocketScope = null;

    drawer = false;
    drawerMiniState = true;
    showExitPopup = false;

    upperLocations = [{
        pathName: "Demo",
        icon: "fas fa-briefcase",
        label: "Demo",
    }];
    lowerLocations = [];

    // METHODS ------------------------------------------------------------------

    navigate(name: string) {
        if (this.$route.name !== name) {
            // Navigate.
            this.$router.push({name});
        }
    }

    close_view() {
        http.websocket.close(); // This will implicitly call sys:close messages and navigate to login.
    }

    shutdown_app() {
        this.$q.dialog({
            title: "Are you sure to shutdown?",
            message: "The remaining work will be lost",
            ok: {
                label: "Leave",
                color: "negative",
            },
            cancel: {
                label: "Stay",
                color: "positive",
            },
        }).onOk(async () => {
            await http.websocket.send(WsMethods.msg.shutdown, undefined, this.wsScope);

            http.websocket.close(); // This will implicitly call sys:close messages and navigate to login.
        });
    }

    // HOOKS ------------------------------------------------------------------

    created() {
        this.wsScope = http.websocket.scopeManager.addScope("main-layout");
    }

    destroyed() {
        http.websocket.scopeManager.deleteScope(this.wsScope!!);
    }
}
</script>

<style scoped lang="scss">
@import "src/css/app";

.own-footer {
    height: $own-footer-height;
}
</style>