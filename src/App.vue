<template>
  <q-layout id="app" view="hHh lpr fff" style="height: 100vh;">
    <!-- 头部区域 -->
    <q-header elevated class="row justify-between q-pa-lg">
      <div class="title">
        <q-icon name="folder_open" class="title-icon" />
        文件时间修复器
      </div>
      <div class="subtitle">高效管理和修复文件时间信息</div>
      <div>
        <q-btn unelevated class="full-height"
               @click="demonstrateEmitEvents"
               color="primary" icon="info"
               label="演示事件" />
      </div>
    </q-header>

    <!-- 主内容区域 -->
    <q-page-container class="full-height">
      <q-page class="full-height column">
        <!-- 控制区域 -->
        <q-card flat class="col-auto" style="background-color: aliceblue;">
          <q-card-section>
            <div class="row q-gutter-md">
              <div class="col">
                <q-input
                         v-model="currentDirectory"
                         placeholder="输入目录路径或点击选择"
                         @keyup.enter="loadDirectory"
                         dense=""
                         readonly
                         outlined>
                  <template v-slot:append>
                    <q-btn color="grey" flat @click="loadDirectory" :loading="loading" icon="refresh" />
                  </template>
                </q-input>
              </div>
              <div class="col-auto">
                <q-btn unelevated class="full-height" @click="selectDirectory" color="primary" icon="folder"
                       label="选择目录" />
              </div>
            </div>

            <!-- 错误提示区域 -->
            <q-banner v-if="error" class="text-white bg-red q-mt-md" inline-actions>
              {{ error }}
              <template v-slot:action>
                <q-btn flat color="white" label="关闭" @click="error = ''" />
              </template>
            </q-banner>

          </q-card-section>
          <!-- 扫描进度区域 -->
          <q-linear-progress :value="files.length / filesAllLength" color="primary" size="25px" class="q-mt-sm">
            <div class="absolute-full flex flex-center">
              <q-badge color="white" text-color="primary" :label="files.length + '/' + filesAllLength" />
            </div>
          </q-linear-progress>
        </q-card>

        <!-- 文件列表区域 -->
        <q-card v-if="files.length > 0" flat class="col q-ma-md full-height column">
          <q-card-section class="col-auto">
            <div class="row items-center justify-between">
              <div class="text-h6">
                <q-icon name="description" class="q-mr-sm" />
                文件列表
              </div>
              <div>
                <q-chip color="info" text-color="white" icon="folder_open">
                  共 {{ files.length }} 个文件
                </q-chip>
              </div>
            </div>
          </q-card-section>
          <q-separator />
          <q-card-section class="col">
            <q-table ref="fileTable" :rows="files" :columns="columns" row-key="name" :rows-per-page-options="[0]" flat
                     class="full-height">
              <template v-slot:body-cell-name="props">
                <q-td :props="props">
                  <div class="row items-center">
                    <!-- 缩进根据深度调整 -->
                    <div v-for="n in props.row.depth" :key="n" class="q-mr-md">-</div>
                    <q-icon :name="props.row.is_directory ? 'folder' : 'description'" size="md"
                            :color="props.row.is_directory ? 'yellow' : ''"
                            class="q-mr-sm" />
                    {{ getRelativePath(props.row.path) }} {{ props.row.name }}
                  </div>
                </q-td>
              </template>

              <template v-slot:body-cell-size="props">
                <q-td :props="props" class="text-right">
                  <q-chip v-if="!props.row.is_directory" color="transparent" text-color="grey-6" size="sm" dense>
                    {{ formatFileSize(props.row.size) }}
                  </q-chip>
                </q-td>
              </template>

              <template v-slot:body-cell-modified="props">
                <q-td :props="props" class="text-right">
                  <div class="row items-center justify-end">
                    <div v-if="props.row.modified > 0">
                      <q-icon name="schedule" class="q-mr-sm"
                              :color="props.row.is_directory ? 'yellow' : ''" />
                      {{ formatTime(props.row.modified) }}
                    </div>
                    <div v-else>
                      --
                    </div>
                  </div>
                </q-td>
              </template>
            </q-table>
          </q-card-section>
        </q-card>

        <!-- 空状态区域 -->
        <q-card v-else-if="!loading" flat class="col q-ma-lg">
          <q-card-section class="text-center">
            <div class="empty-state">
              <q-icon name="folder_open" size="4rem" color="grey-6" />
              <div class="text-h6 q-mt-md">请选择一个目录来查看文件信息</div>
              <q-btn color="primary" icon="folder" label="选择目录" @click="selectDirectory" class="q-mt-md" />
            </div>
          </q-card-section>
        </q-card>
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

export default {
  name: 'App',
  data() {
    return {
      currentDirectory: '',
      files: [],
      fileAll: [],
      filesAllLength: 100,
      loading: false,
      error: '',
      columns: [
        {
          name: 'name',
          required: true,
          label: '文件名',
          align: 'left',
          field: 'name',
          sortable: true,
          style: 'min-width: 300px'
        },
        {
          name: 'size',
          align: 'right',
          label: '大小',
          field: 'size',
          sortable: true,
          style: 'width: 120px'
        },
        {
          name: 'modified',
          align: 'right',
          label: '修改时间',
          field: 'modified',
          sortable: true,
          style: 'width: 180px'
        }
      ]
    }
  },
  methods: {

    // 演示事件
    async demonstrateEmitEvents() {
      try {
        await invoke('demonstrate_emit_events')
        console.log('演示事件成功触发')
      } catch (err) {
        this.error = '演示事件失败: ' + err
      }
    },

    async selectDirectory() {
      try {
        const selected = await invoke('select_directory')
        if (selected) {
          this.currentDirectory = selected
          this.loadDirectory()
        }
      } catch (err) {
        this.error = '选择目录失败: ' + err
      }
    },

    async loadDirectory() {
      if (!this.currentDirectory.trim()) {
        this.error = '请输入目录路径'
        return
      }

      this.loading = true
      this.error = ''
      this.files = []

      try {

        // 设置扫描进度事件监听器
        this.scanReadyUnlisten = await listen('scan_directory:ready', (event) => {
          // console.log('扫描进度:', event.payload)
          // this.files.push(event.payload)
          this.fileAll = event.payload
          this.filesAllLength = event.payload.length
          this.$q.notify({
            message: `目录扫描完成，共找到 ${this.filesAllLength} 个文件`,
            color: 'positive',
            position: 'top'
          })
        })

        // 设置扫描进度事件监听器
        this.scanProgressUnlisten = await listen('scan_directory:progress', (event) => {
          // console.log('扫描进度:', event.payload)
          this.files.push(event.payload)

          // 使用nextTick确保DOM更新后再滚动
          this.$nextTick(() => {
            // 获取表格的滚动容器
            const scrollContainer = this.$refs.fileTable?.$el?.querySelector('.q-table__middle')
            // 滚动到底部
            if (scrollContainer) {
              scrollContainer.scrollTop = scrollContainer.scrollHeight
            }
          })
        })

        // 设置扫描完成事件监听器
        this.scanCompleteUnlisten = await listen('scan_directory:complete', () => {
          this.loading = false
          // 扫描完成后显示通知
          this.$q.notify({
            message: `目录扫描完成，共找到 ${this.files.length} 个文件`,
            color: 'positive',
            position: 'top'
          })
          // 清理事件监听器
          if (this.scanProgressUnlisten) {
            this.scanProgressUnlisten()
            this.scanProgressUnlisten = null
          }
          if (this.scanCompleteUnlisten) {
            this.scanCompleteUnlisten()
            this.scanCompleteUnlisten = null
          }
        })

        // 启动扫描，但不等待完整结果
        invoke('scan_directory', { directory: this.currentDirectory })

      } catch (err) {
        this.error = '加载目录失败: ' + err
        this.loading = false

        // 清理事件监听器
        if (this.scanReadyUnlisten) {
          this.scanReadyUnlisten()
          this.scanReadyUnlisten = null
        }
        if (this.scanProgressUnlisten) {
          this.scanProgressUnlisten()
          this.scanProgressUnlisten = null
        }
        if (this.scanCompleteUnlisten) {
          this.scanCompleteUnlisten()
          this.scanCompleteUnlisten = null
        }
      }
    },

    formatFileSize(bytes) {
      if (bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    },

    formatTime(timestamp) {
      // 将秒级Unix时间戳转换为毫秒级
      // JavaScript的Date对象不能自动判断是秒级还是毫秒级时间戳
      // 秒级时间戳通常长度在10位左右，毫秒级在13位左右
      const numTimestamp = Number(timestamp);
      // 如果是秒级时间戳（小于10^12），乘以1000转成毫秒级
      const timestampMs = numTimestamp < 1000000000000 ? numTimestamp * 1000 : numTimestamp;
      return new Date(timestampMs).toLocaleString('zh-CN')
    },

    // 获取相对于当前目录的路径
    getRelativePath(fullPath) {
      if (!this.currentDirectory || !fullPath) {
        return fullPath;
      }

      // 确保路径格式一致
      const currentDir = this.currentDirectory.replace(/\\/g, '/');
      const filePath = fullPath.replace(/\\/g, '/');

      // 如果文件路径以当前目录路径开头，则去掉重复部分
      if (filePath.startsWith(currentDir)) {
        let relativePath = filePath.substring(currentDir.length);

        // 去掉开头的斜杠（如果存在）
        if (relativePath.startsWith('/')) {
          relativePath = relativePath.substring(1);
        }

        // 如果相对路径为空，则显示当前目录
        if (relativePath === '') {
          return '.';
        }

        return relativePath;
      }

      // 如果不是当前目录下的文件，返回完整路径
      return fullPath;
    }
  },
  beforeUnmount() {
    // 清理事件监听器
    if (this.scanReadyUnlisten) {
      this.scanReadyUnlisten()
      this.scanReadyUnlisten = null
    }
    if (this.scanProgressUnlisten) {
      this.scanProgressUnlisten()
      this.scanProgressUnlisten = null
    }
    if (this.scanCompleteUnlisten) {
      this.scanCompleteUnlisten()
      this.scanCompleteUnlisten = null
    }
  }
}
</script>

<style scoped>
/* 自定义导航栏样式 */
.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.title {
  font-size: 24px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-icon {
  font-size: 28px;
}

.subtitle {
  text-align: center;
  padding: 8px 16px 16px;
  font-size: 16px;
  opacity: 0.9;
  font-weight: 300;
}

.control-card {
  margin-bottom: 16px;
}

.file-list-card {
  min-height: 400px;
}

.file-table {
  height: 100%;
}

.empty-card {
  min-height: 400px;
}

.empty-state {
  padding: 60px 20px;
}
</style>