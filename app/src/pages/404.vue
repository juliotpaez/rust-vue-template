<template>
    <div>here</div>
</template>

<script lang="ts">
import {Component, Vue} from "vue-property-decorator";

@Component
export default class ProjectPage extends Vue {
    created() {
        this.$router.push({name: "Project"});
    }
}
</script>