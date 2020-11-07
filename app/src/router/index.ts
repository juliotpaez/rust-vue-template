import {route} from "quasar/wrappers";
import VueRouter from "vue-router";
import {Store} from "vuex";
import routes from "./routes";
import {http} from "src/plugins/http-commons";
import LocationStore from "src/store/modules/LocationStore";
import {getModule} from "vuex-module-decorators";

/*
 * If not building with SSR mode, you can
 * directly export the Router instantiation
 */

export default route<Store<any>>(function ({Vue}) {
    Vue.use(VueRouter);

    const Router = new VueRouter({
        scrollBehavior: () => ({
            x: 0,
            y: 0,
        }),
        routes,

        // Leave these as is and change from quasar.conf.js instead!
        // quasar.conf.js -> build -> vueRouterMode
        // quasar.conf.js -> build -> publicPath
        mode: process.env.VUE_ROUTER_MODE,
        base: process.env.VUE_ROUTER_BASE,
    });

    Router.beforeEach((to, from, next) => {
        if (to.matched.some(record => record.meta.requiresLocation)) {
            if (http.websocket.isNull) {
                let store = getModule(LocationStore);
                store.setRedirect(to.name || null);

                next({name: "Login"});
            } else {
                next();
            }
        } else {
            next();
        }
    });

    return Router;
});
