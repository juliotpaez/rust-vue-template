import Vuex from "vuex";
import {Vue} from "vue-property-decorator";

Vue.use(Vuex);

export default new Vuex.Store({
    // enable strict mode (adds overhead!)
    // for dev mode only
    strict: !!process.env.DEBUGGING,
});
