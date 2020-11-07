<template>
    <q-page class="own-height-without-header-footer scroll">
        <q-header class="bg-dark" bordered>
            <div class="own-content-center q-mx-auto">
                <q-bar class="bg-transparent">
                    <div class="text-h5">{{ title }}</div>
                    <slot name="appbar"></slot>
                </q-bar>
            </div>
        </q-header>
        <div :class="contentClasses">
            <slot></slot>
        </div>
        <q-inner-loading :showing="loading"></q-inner-loading>
    </q-page>
</template>

<script lang="ts">
import {Component, Prop, Vue} from "vue-property-decorator";

@Component
export default class TrvDialogWindow extends Vue {
    @Prop({
        type: String,
        required: true,
    }) readonly title!: string;
    @Prop({
        type: Boolean,
        default: false,
    }) readonly centered!: boolean;
    @Prop({
        type: Boolean,
        default: false,
    }) readonly fullHeight!: boolean;
    @Prop({
        type: Boolean,
        default: false,
    }) readonly loading!: boolean;

    // GETTERS ----------------------------------------------------------------

    get contentClasses() {
        return {
            "own-content-center": this.centered,
            "q-mx-auto": this.centered,
            "own-height-without-header-footer": this.fullHeight,
        };
    }

    // EVENTS -----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    // HOOKS ------------------------------------------------------------------
}
</script>