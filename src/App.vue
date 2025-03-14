<template>
  <div class="common-layout">
    <el-container>
      <el-header width="200px">
        <div>
          <!-- 菜单侧栏导航栏 -->
          <Menu @message="handleMenuMessage"></Menu>
        </div>
      </el-header>
      <el-main>
        <!-- 主要内容 -->
        <div>
          <component :is="tabs[currentTab]"></component>
        </div>
      </el-main>

    </el-container>

  </div>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import Menu from './Menu.vue'
import ControlPanel from './ControlPanel.vue'
import Load from './Load.vue'
// import Flash from './Flash.vue'
import Inspector from './Inspector.vue'
import NotFound from './NotFound.vue'
import 'element-plus/dist/index.css'
//动态路由处理
const tabs: any = {
  ControlPanel,
  Load,
  // Flash,
  Inspector,
  NotFound
}

const currentTab = ref('ControlPanel')


// 处理菜单消息的事件函数
const handleMenuMessage = (message: string) => {
  switch (message) {
    case "1":
      currentTab.value = "ControlPanel"
      break
    case "2-1":
      currentTab.value = "Flash"
      break
    case "2-2":
      currentTab.value = "Inspector"
      break
    case "2-3":
      currentTab.value = "Load"
      break
    default:
      currentTab.value = "ControlPanel"
      break
  }
}


console.log('currentTab.value:', currentTab.value)
{
  // 这里可以根据接收到的 message 执行路由跳转或其他操作
  // 例如：router.push(message) 如果配置了对应路由
}

</script>
