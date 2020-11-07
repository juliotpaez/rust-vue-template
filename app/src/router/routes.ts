import {RouteConfig} from "vue-router";

const routes: RouteConfig[] = [{
    path: "/",
    name: "Login",
    component: () => import("pages/Login.vue"),
}, {
    path: "/app",
    component: () => import("layouts/MainLayout.vue"),
    meta: {
        requiresLocation: true,
    },
    children: [{
        path: "",
        name: "App",
        redirect: {name: "Project"},
    }, {
        path: "demo",
        name: "Demo",
        component: () => import("pages/Demo.vue"),
        meta: {
            requiresLocation: true,
        },
    }],
}, {
    path: "*",
    component: () => import("pages/404.vue"),
}];

export default routes;
