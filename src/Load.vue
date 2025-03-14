<template>
    <el-form label-width="auto" style="max-width: 2000px" inline="true">
        <el-form-item label="标签">
            <el-select clearable v-model="valueTag" placeholder="设置你的标签" style="width: 200px">
                <el-option v-for="item in tags" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
        </el-form-item>
        <el-form-item label="序列号" label-width="140px">
            <el-input v-model="macInput" autocomplete="off" />
        </el-form-item>
        <el-form-item label-width="140px">
            <el-button type="primary" @click="onQuery">
                查询
            </el-button>
        </el-form-item>
    </el-form>

    <el-table ref="multipleTableRef" :data="tableData" row-key="id" style="width: 100%" height="500"
        @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" />
        <el-table-column property="id" label="id" width="80" />
        <el-table-column property="mac" label="序列号" width="120" />
        <el-table-column property="board_name" label="设备名称" width="180" />
        <el-table-column property="tag" label="标签" width="120" />
        <el-table-column property="result" label="检验结果" width="240" />
        <el-table-column property="time" label="检验时间" width="120" />
        <el-table-column property="remark" label="备注" width="240" />
    </el-table>

    <br>
    <el-form :inline="true" ref="form" label-width="120px" style="width: 100%">
        <el-form-item label="保存至文件夹">
            <el-input style="width: 240px" placeholder="请选择文件夹路径" v-model="imgSavePath" class="input-with-select" readonly>
            </el-input>
        </el-form-item>
        <el-form-item>
            <el-button slot="append" icon="el-icon-folder" type="primary" @click="selectFolder">选择导出文件夹</el-button>
        </el-form-item>

        <el-form-item>
            <el-button type="primary" @click="onSubmit">
                导出
            </el-button>
        </el-form-item>

    </el-form>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ElNotification } from 'element-plus';
import 'element-plus/dist/index.css'

const valueTag = ref('');
const tags = ref<Array<{ value: string; label: string }>>([]);
const macInput = ref('');

import type { TableInstance } from 'element-plus'

interface Board {
    id: number
    mac: string
    board_name: string
    tag: string
    result: string
    time: string
    remark: string
}



const multipleTableRef = ref<TableInstance>()
const multipleSelection = ref<Board[]>([])
const imgSavePath = ref('')

const handleSelectionChange = (val: Board[]) => {
    multipleSelection.value = val
}

const tableData = ref<Board[]>([])

const onQuery = async () => {
    console.log('onQuery')

    // 将空值转换为 "NULL"
    const mac = macInput.value || "NULL";
    const tag = valueTag.value || "NULL";

    try {
        const response: any = await invoke('_query_db', {
            s: JSON.stringify({ mac: mac, tag: tag }) // 确保传递的参数对象中包含 `s` 键
        })
        const data = JSON.parse(response);
        console.log("data: ", data);

        // 确保后端返回的数据是一个数组
        if (Array.isArray(data)) {
            tableData.value = data.map((item: any) => ({
                id: item.id,
                mac: item.mac,
                board_name: item.board_name,
                tag: item.tag,
                result: item.result,
                time: item.time,
                remark: item.remark
            }));
        } else {
            ElNotification({
                title: '提示',
                message: '后端返回的数据格式不正确',
                type: 'warning',
            })
        }
    } catch (error: any) {
        ElNotification({
            title: '报错',
            message: error,
            type: 'error',
        })
    }
}
const onSubmit = async () => {
    console.log('onSubmit')
    console.log(multipleSelection.value);
    let id_arr = multipleSelection.value.map((selecte) => {
        return selecte.id;
    });
    let load_info = {
        id: id_arr,
        path: imgSavePath.value
    }

    await invoke('_load', {
        s: JSON.stringify(load_info) // 确保传递的参数对象中包含 `s` 键
    })
        .then((response: any) => {
            if (response == '200') {
                ElNotification({
                    title: '提示',
                    message: "导出成功！",
                    type: 'success',
                });
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



// 选择文件夹路径
const selectFolder = async () => {
    try {
        const selected = await open({
            directory: true, // 只允许选择文件夹
            multiple: false, // 不允许选择多个文件夹
        });
        if (selected) {
            imgSavePath.value = selected as string; // 将选择的路径赋值给 imgSavePath
        }
    } catch (error) {
        console.error('选择文件夹出错:', error);
    }
}

onMounted(async () => {
    await get_tags();
});

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
</script>

<style scoped></style>