<template>
    <el-form label-width="auto" style="max-width: 2000px" inline="true">
        <el-form-item label="串口号">
            <el-select v-model="valueSerial" placeholder="设置你的串口号" style="width: 200px" :disabled=is_open>
                <el-option v-for="item in serials" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
        </el-form-item>
        <el-form-item label="波特率">
            <el-select v-model="valueBaud" placeholder="设置你的波特率" style="width: 200px" :disabled=is_open>
                <el-option v-for="item in bauds" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
        </el-form-item>
        <el-form-item label="标签">
            <el-select v-model="valueTag" placeholder="设置你的标签" style="width: 200px" :disabled=is_open>
                <el-option v-for="item in tags" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
        </el-form-item>
        <el-form-item label="检验" style="width: 200px">
            <el-button :type="button_type" @click="onSubmit" :loading=loading>
                {{ button_label }}
            </el-button>
        </el-form-item>
    </el-form>
    <div>
        <el-card style="width:500px" shadow="always">
            <span>设备名称: {{ board_name }}</span>
            <br>
            <span>MAC地址: {{ mac }}</span>
        </el-card>
    </div>
    <br>
    <el-row :gutter="20">
        <el-col :span="6">
            <div style="height: 300px; max-width: 600px">
                <el-steps :active="tips" direction="vertical" finish-status="success">
                    <el-step title="读取设备信息.." />
                    <el-step title="检验中.." />
                    <el-step title="检验结束" />
                    <el-step title="记录中.." />
                    <el-step title="结束" />
                </el-steps>

            </div>
        </el-col>
        <el-col :span="10" style="height: 400px;">
            <el-table :data="tableData" height="300" style="width: 100%">
                <el-table-column prop="date" label="时间" width="180" />
                <el-table-column prop="message" label="信息" width="180" />
                <el-table-column prop="result" label="结果" />
            </el-table>
        </el-col>

        <el-col :span="8">
            <div>
                <el-progress type="dashboard" :percentage="percentage" :color="colors">
                    <template #default="{ percentage }">
                        <span class="percentage-value">{{ percentage }}%</span>
                        <span class="percentage-label">检验中</span>
                    </template>
                </el-progress>
            </div>
        </el-col>
    </el-row>
    <div>
        <el-dialog v-model="dialogTableVisible" title="Shipping address" width="800" close-on-click-modal="false"
            show-close="false">
            <el-table :data="tableData">
                <el-table-column prop="date" label="时间" width="150" />
                <el-table-column prop="message" label="信息" width="150" />
                <el-table-column prop="result" label="结果" />
            </el-table>
            <el-form>
                <el-form-item label="备注" label-width="140px">
                    <el-input v-model="remark" autocomplete="off" />
                </el-form-item>
            </el-form>
            <template #footer>
                <div class="dialog-footer">
                    <el-button type="primary" @click="LoadData">
                        记录
                    </el-button>
                </div>
            </template>
        </el-dialog>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { ElNotification } from 'element-plus';
import 'element-plus/dist/index.css'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
const valueSerial = ref('')
const valueBaud = ref('')
const valueTag = ref('')
const serials = ref<Array<{ value: string; label: string }>>([]);
const tags = ref<Array<{ value: string; label: string }>>([]);

const button_label = ref('开启');
const loading = ref(false);
const button_type = ref('primary');
const is_open = ref(false);
const board_name = ref('')
const mac = ref('')
const tips = ref(0)
const tableData: any = ref([])
const percentage = ref(0)
const dialogTableVisible = ref(false)
const bauds: any = [
    {
        value: 115200,
        label: 115200,
    },
    {
        value: 460800,
        label: 460800,
    },
    {
        value: 1152000,
        label: 1152000,
    },
]

const remark = ref('检验成功');

let board_id: any = 0;
let is_push_table_flag: boolean = false;


const message: any = {
    "blecard_init_check": "蓝牙卡片检验...",
    "blecard_recv_check": "蓝牙卡片接收检验...",
    "nvs_check": "内存检验...",
    "onboard_sensor_check": "板载传感器检验...",
    "online_check": "入网检验...",
    "send_data_check": "发送消息检验...",
    "ble_mode_check": "蓝牙模式检验...",
    "config_to_gateway": "主机配对检验...",
    "recv_sensor_check": "接收传感器消息检验..."
}

const message2: any = {
    "blecard_init_check": "蓝牙卡片检验结束",
    "blecard_recv_check": "蓝牙卡片接收检验结束",
    "nvs_check": "内存检验结束",
    "onboard_sensor_check": "板载传感器检验结束",
    "online_check": "入网检验结束",
    "send_data_check": "发送消息检验结束",
    "ble_mode_check": "蓝牙模式检验结束",
    "config_to_gateway": "主机配对检验结束",
    "recv_sensor_check": "接收传感器消息结束"
}



const colors = [
    { color: '#f56c6c', percentage: 20 },
    { color: '#e6a23c', percentage: 40 },
    { color: '#5cb87a', percentage: 60 },
    { color: '#1989fa', percentage: 80 },
    { color: '#6f7ad3', percentage: 100 },
]


let timer: number | null = null;



onMounted(async () => {
    await get_tags();
    // 获取串口列表
    const fetchSerialPorts = async () => {

        await invoke('_get_serial_port').then((response: any) => {
            const ports = JSON.parse(response);

            // 转换数据格式并更新响应式数组
            serials.value = ports.map((port: string) => ({
                value: port,
                label: port
            }));
            if (ports.length > 0 && !valueSerial.value) {
                valueSerial.value = ports[0];
            }
            if (ports.length > 0 && !valueBaud.value) {
                valueBaud.value = bauds[0].value;
            }

        })
            .catch((error: any) => {
                console.error('获取串口列表失败:', error);
            });

    };

    // 立即执行一次然后定时刷新
    fetchSerialPorts();
    timer = setInterval(fetchSerialPorts, 1000);

    // 监听事件
    await listen_board_info_event();
    await listen_inspect_tips_event();

});

const stop_serial = async () => {
    if (is_open.value == true) {
        invoke('_stop_inspect')
            .then((response: any) => {
                console.log(response);
                if (response == '200') {
                    button_label.value = '开启';
                    button_type.value = 'primary';
                    is_open.value = false;
                    board_name.value = '';
                    mac.value = '';
                    percentage.value = 0;
                    tableData.value = [];
                    tips.value = 0;
                    is_push_table_flag = false;
                    //清空
                    console.log("点击关闭按钮");
                }
            })
            .catch((error: any) => {
                ElNotification({
                    title: '报错',
                    message: error,
                    type: 'error',
                });
            });
    }
}

const start_serial = async () => {

    if (is_open.value == false) {
        loading.value = true;
        button_label.value = 'connect';
        invoke('_start_inspect', {
            port: valueSerial.value, baud: valueBaud.value, tag: valueTag.value
        }).then((response: any) => {
            loading.value = false;
            console.log(response);
            if (response == '200') {
                button_label.value = '停止';
                button_type.value = 'danger';
                is_open.value = true;
                console.log("点击开启按钮");
            }

        })
            .catch((error: any) => {
                loading.value = false;
                button_label.value = '开启';
                ElNotification({
                    title: '报错',
                    message: error,
                    type: 'error',
                })
            });
    }
}

onUnmounted(() => {
    // 组件卸载时清除定时器
    if (timer) clearInterval(timer);
    stop_serial();

});

const set_percentage = async () => {
    const total = tableData.value.length; // 总条目数
    console.log(" tableData length" + total);
    if (total === 0) return 0; // 如果数组为空，进度为 0

    const successCount = tableData.value.filter((item: { result: string; }) => (item.result === '成功') || (item.result === '失败')).length; // 成功的条目数
    console.log("successCount" + successCount)
    if (successCount == 0) { return }

    let progress = (successCount / total) * 100; // 计算百分比
    console.log("progress" + progress)
    // if (progress > 100) { progress = 100 }
    progress = progress | 0;
    console.log(progress);
    percentage.value = progress;
    if (percentage.value > 100) { percentage.value = 100 }
    if (percentage.value == 100) {
        tips.value = 3;
        dialogTableVisible.value = true;
    }
}

const board_check_get = (key: string): boolean | null => {
    console.log("key: ", key);
    console.log("tableData: ", tableData.value);

    // 使用 find 方法查找数组中 key 属性匹配的对象
    const item = tableData.value.find((item: any) => item.key === key);

    if (!item) {
        return null; // 如果找不到对象，返回 null
    }

    if (item.result === '成功') {
        return true;
    } else {
        return false;
    }
};
const LoadData = () => {
    tips.value = 4;
    let json = {
        "mac": mac.value,
        "board_id": board_id,
        "board_name": board_name.value,
        "tag": valueTag.value,
        "remark": remark.value,
        "blecard_init_check": board_check_get("blecard_init_check"),
        "blecard_recv_check": board_check_get("blecard_recv_check"),
        "nvs_check": board_check_get("nvs_check"),
        "onboard_sensor_check": board_check_get("onboard_sensor_check"),
        "online_check": board_check_get("online_check"),
        "send_data_check": board_check_get("send_data_check"),
        "ble_mode_check": board_check_get("ble_mode_check"),
        "config_to_gateway": board_check_get("config_to_gateway")
    }
    let josn_str = JSON.stringify(json);
    console.log(josn_str);
    invoke('_insert_db', { s: josn_str })
        .then((response: any) => {
            if (response == '200') {
                dialogTableVisible.value = false;
                tips.value = 5;
                stop_serial();
            }
        }
        )
        .catch((error: any) => {
            ElNotification({
                title: '报错',
                message: error,
                type: 'error',
            });
        })

}

const onSubmit = async () => {
    if (valueSerial.value != '' && valueBaud.value != '' && valueTag.value != '') {
        stop_serial();
        start_serial();
    }
    else {

        ElNotification({
            title: '提示',
            message: '请选择串口号、波特率、标签，否则无法开启检验',
            type: 'warning',
        })
    }

}
const listen_board_info_event = async () => {
    await listen('board_info_event', (event: any) => {
        if (is_push_table_flag === true) return;
        console.log('Received event:', event.payload);
        let json = JSON.parse(event.payload);
        board_name.value = json['board_name'];
        mac.value = json['mac'];
        //收到设备信息后，步骤为1
        tips.value = 1;
        //根据收到的设备信息确定需要检测的项目
        //bord_id为1表示250303主机

        invoke('_read_config').then((response: any) => {
            let config = JSON.parse(response);
            console.log("push josn" + JSON.stringify(config));
            board_id = parseInt(json['board_id'], 10);
            if (json['board_id'] == 1) {
                for (let key in config['gateway']) {
                    //只有为true的需要检验

                    if (config['gateway'][key] == true) {
                        console.log("push data:" + message[key])
                        tips.value = 2;
                        tableData.value.push({
                            date: new Date().toLocaleString(),
                            message: message[key], // 使用 message[key] 来获取对应的中文描述
                            key: key,
                            result: '检验中'
                        });
                        is_push_table_flag = true;
                    }

                }
            }
            else if (json['board_id'] == 0) {
                tableData.value = [];
                tips.value = 0;

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
};

const get_tags = async () => {
    invoke('_read_config').then((response: any) => {
        let config = JSON.parse(response);
        console.log("tags ：" + config['tags'])
        console.log("tagsss ：" + JSON.stringify(config))
        tags.value = config['tags'].map((tag: string) => ({
            value: tag,
            label: tag
        }));
    })
        .catch((error: any) => {
            ElNotification({
                title: '报错',
                message: error,
                type: 'error',
            })
        });

};

const listen_inspect_tips_event = async () => {
    await listen('inspect_tips_event', (event: any) => {
        console.log('Received event:', event.payload);
        let json = JSON.parse(event.payload);
        const messageKey = json['message']; // 获取消息的 key，例如 'blecard_recv_check'
        console.log("messageKey " + messageKey);
        // 查找 tableData 中对应的条目
        const index = tableData.value.findIndex((item: { key: any; }) => item.key === messageKey);

        if (index !== -1) {
            // 如果找到对应的条目，更新它
            tableData.value[index].date = new Date().toLocaleString();
            tableData.value[index].result = json['content'] ? '成功' : '失败';
            tableData.value[index].message = message2[messageKey]; // 更新中文描述
        }
        set_percentage();
    });
};
</script>

<style scoped>
.percentage-value {
    display: block;
    margin-top: 10px;
    font-size: 28px;
}

.percentage-label {
    display: block;
    margin-top: 10px;
    font-size: 12px;
}
</style>
