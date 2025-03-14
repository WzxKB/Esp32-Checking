<template>
    <div>
        <el-form :model="form" label-width="auto" style="max-width: 600px" label-position="right">
            <el-text size="large" style="">主机检验配置：</el-text>
            <br>
            <el-form-item label="内存检验：">
                <el-switch v-model="form.gateway.nvs_check" style="margin-left: 24px" inline-prompt :active-icon="Check"
                    :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="板载传感器检验：">
                <el-switch v-model="form.gateway.onboard_sensor_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="入网检验：">
                <el-switch v-model="form.gateway.online_check" style="margin-left: 24px" inline-prompt :active-icon="Check"
                    :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="蓝牙卡片初始化检验：">
                <el-switch v-model="form.gateway.blecard_init_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="蓝牙卡片接收检验：">
                <el-switch v-model="form.gateway.blecard_recv_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="蓝牙模式检验：">
                <el-switch v-model="form.gateway.ble_mode_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="接收传感器消息检验：">
                <el-switch v-model="form.gateway.recv_sensor_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-text size="large" style="">组网式传感器检验配置：</el-text>
            <br>
            <el-form-item label="内存检验：">
                <el-switch v-model="form.sensor.nvs_check" style="margin-left: 24px" inline-prompt :active-icon="Check"
                    :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="发送消息检验:">
                <el-switch v-model="form.sensor.send_data_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="蓝牙模式检验：">
                <el-switch v-model="form.sensor.ble_mode_check" style="margin-left: 24px" inline-prompt :active-icon="Check"
                    :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="主机配对检验：">
                <el-switch v-model="form.sensor.config_to_gateway" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-text size="large" style="">独立式传感器检验配置：</el-text>
            <br>
            <el-form-item label="内存检验：">
                <el-switch v-model="form.independent.nvs_check" style="margin-left: 24px" inline-prompt :active-icon="Check"
                    :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="入网检验:">
                <el-switch v-model="form.independent.online_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="蓝牙模式检验:">
                <el-switch v-model="form.independent.ble_mode_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-form-item label="发送消息检验:">
                <el-switch v-model="form.independent.send_data_check" style="margin-left: 24px" inline-prompt
                    :active-icon="Check" :inactive-icon="Close" />
            </el-form-item>
            <el-text size="large" style="">标签：</el-text>
            <br>
            <el-input-tag v-model="form.tags" placeholder="Please input"
                aria-label="Please click the Enter key after input" />
            <br>
            <el-form-item>
                <el-popover :visible="visible" placement="top" :width="160">
                    <p>确认清除所有缓存吗？确认后将不可恢复！</p>
                    <div style="text-align: right; margin: 0">
                        <el-button size="small" text @click="visible = false">取消</el-button>
                        <el-button size="small" type="primary" @click="onDelete">
                            确认
                        </el-button>
                    </div>
                    <template #reference>
                        <el-button @click="visible = true" type="danger">清除缓存</el-button>
                    </template>
                </el-popover>
            </el-form-item>
        </el-form>
        <br><br><br>
        <el-affix position="bottom" :offset="50" :style="{
            position: 'fixed',
            right: '50px',
            bottom: '50px',
            zIndex: 1000
        }">
            <el-button type="primary" @click="onSubmit">
                设置 </el-button>
        </el-affix>
    </div>
</template>

<script lang="ts" setup>
import { reactive, onMounted, ref } from 'vue'
import { Check, Close } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core';
// do not use same name with ref
import { ElNotification } from 'element-plus';
import 'element-plus/dist/index.css'
const visible = ref(false)
const form = reactive({
    gateway: {
        blecard_init_check: false,
        blecard_recv_check: false,
        nvs_check: true,
        onboard_sensor_check: true,
        online_check: true,
        ble_mode_check: true,
        recv_sensor_check: true,
    },
    sensor: {
        nvs_check: true,
        send_data_check: true,
        ble_mode_check: true,
        config_to_gateway: true,
    },
    independent: {
        nvs_check: true,
        online_check: true,
        send_data_check: true,
        ble_mode_check: true,
    },
    tags: [],
})

onMounted(() => {

    invoke('_read_config')
        .then((response: any) => {
            console.log("res: " + response);
            let json = JSON.parse(response);
            console.log("josn: " + json);
            Object.assign(form, json);
        })
        .catch((error: any) => {
            console.log(error);
        });

    invoke('_create_db')
        .then((response: any) => {

            if (response == '200') {
                console.log("数据库创建成功！");
            }
        })
        .catch((error: any) => {
            ElNotification({
                title: '报错',
                message: error,
                type: 'error',
            })
        });
});

const onDelete = () => {
    visible.value = false
    invoke('_ondelete')
        .then((response: any) => {
            if (response == '200') {
                ElNotification({
                    title: '提示',
                    message: '缓存清除成功',
                    type: 'success',
                })
                window.location.reload(); // 刷新页面
            }
        })
        .catch((error: any) => {
            ElNotification({
                title: '提示',
                message: '清除失败:' + error,
                type: 'warning',
            })
            console.log(error);
        });
}



const onSubmit = () => {
    let josn = JSON.stringify(form);
    invoke('_write_config', { config: josn })
        .then((response: any) => {
            if (response == '200') {
                ElNotification({
                    title: '提示',
                    message: '设置成功',
                    type: 'success',
                })
            }
        })
        .catch((error: any) => {
            ElNotification({
                title: '提示',
                message: '设置失败:' + error,
                type: 'warning',
            })
            console.log(error);
        });
}
</script>

<style scoped></style>