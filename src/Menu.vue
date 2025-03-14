<template>
    <div class="fixed-menu">
        <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal" :ellipsis="false"
            @select="handleSelect" active-text-color="#943037" background-color="#FFFFFF">
            <el-menu-item index="0">
                <img style="width: 100px" src="./assets/compute.svg" alt="esp-check" />
                <span>{{ func_selection }}</span>
            </el-menu-item>
            <el-menu-item index="1">控制中心</el-menu-item>
            <el-sub-menu index="2">
                <template #title>工作区</template>
                <!-- <el-menu-item index="2-1">烧录</el-menu-item> -->
                <el-menu-item index="2-2">检验</el-menu-item>
                <el-menu-item index="2-3">记录</el-menu-item>
            </el-sub-menu>
        </el-menu>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
const func_selection = ref("")
const activeIndex = ref('1')

const emits = defineEmits(['message']);

const handleSelect = (key: string) => {
    switch (key) {
        case "1":
            func_selection.value = "控制中心"
            break
        // case "2-1":
        //     func_selection.value = "烧录"
        //     break
        case "2-2":
            func_selection.value = "检验"
            break
        case "2-3":
            func_selection.value = "记录"
            break
    }
    emits('message', key)
}
</script>

<style scoped>
.el-menu--horizontal>.el-menu-item:nth-child(1) {
    margin-right: auto;
}

.fixed-menu {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
    background: white;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

.el-menu--horizontal {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
}
</style>
