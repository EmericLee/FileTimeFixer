<template>
  <q-layout id="app" view="hHh lpr fff" style="height: 100vh;">
    <!-- 头部区域 -->
    <q-header elevated class="row justify-between q-pa-lg">
      <div class="title">
        <q-icon name="folder_open" class="title-icon" />
        文件时间修复器2
      </div>
      <div class="subtitle">高效管理和修复文件时间信息</div>
      <div>
        <q-btn unelevated class="full-height q-mr-sm" @click="demonstrateEmitEvents" color="primary" icon="info"
               label="演示事件" />
        <q-btn unelevated class="full-height" @click="showConfigDialog" color="secondary" icon="settings" label="配置" />
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
                <q-input v-model="config.rootDirectory" placeholder="输入目录路径或点击选择" @keyup.enter="scanDirectory" dense=""
                         readonly outlined>
                  <template v-slot:append>
                    <q-btn color="grey" flat @click="scanDirectory" :loading="loading" icon="refresh" />
                  </template>
                </q-input>
              </div>
              <div class="col-auto">
                <q-btn unelevated class="full-height" @click="selectRootDirectory" color="primary" icon="folder"
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
          <q-linear-progress instant-feedback :value="filesAllLength > 0 ? files.length / filesAllLength : 0"
                             color="primary"
                             size="25px" class="q-mt-sm">
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
                  共 {{ files.length }} / {{ filesAllLength }} 个文件
                </q-chip>
              </div>
            </div>
          </q-card-section>

          <q-separator />

          <q-card-section class="col">
            <q-virtual-scroll ref="fileList" :items="files" separator v-slot="{ item, index }"
            type="table"                  
            virtual-scroll-item-size="48"
                              style="height:300px">
              <q-item clickable class="file-item">
                <q-item-section side>
                  {{ index + 1 }}
                </q-item-section>
                <q-item-section avatar>
                  <q-icon :name="item.is_directory ? 'folder' : 'image'" size="md"
                          :color="item.is_directory ? 'yellow' : ''" />
                </q-item-section>

                <q-item-section>
                  <q-item-label class="ellipsis">
                    {{ formatPath(item.path) }}
                  </q-item-label>
                </q-item-section>

                <q-item-section side v-if="!item.is_directory">
                  <q-item-label caption>
                    <q-chip color="transparent" text-color="grey-6" size="sm" dense>
                      {{ formatFileSize(item.size) }}
                    </q-chip>
                  </q-item-label>
                </q-item-section>

                <q-item-section side>
                  <q-item-label caption>
                    <div class="row items-center justify-end">
                      <div v-if="item.modified > 0">
                        <q-icon name="schedule" class="q-mr-sm" :color="item.is_directory ? 'yellow' : ''" />
                        {{ formatTime(item.modified) }}
                      </div>
                      <div v-else>
                        --
                      </div>
                    </div>
                  </q-item-label>
                </q-item-section>
              </q-item>
              <q-separator v-if="index < files.length - 1" />
            </q-virtual-scroll>
          </q-card-section>
        </q-card>

        <!-- 空状态区域 -->
        <q-card v-else-if="!loading" flat class="col q-ma-lg">
          <q-card-section class="text-center">
            <div class="empty-state">
              <q-icon name="folder_open" size="4rem" color="grey-6" />
              <div class="text-h6 q-mt-md">请选择一个目录来查看文件信息</div>
              <q-btn color="primary" icon="folder" label="选择目录" @click="selectRootDirectory" class="q-mt-md" />
            </div>
          </q-card-section>
        </q-card>

        <!-- 配置对话框 -->
        <q-dialog v-model="configDialog" persistent>
          <q-card style="width: 500px; max-width: 90vw;">
            <q-card-section>
              <div class="text-h5">应用配置</div>
              <div class="q-mt-sm text-subtitle2 text-grey">配置文件扫描和处理的相关参数</div>
            </q-card-section>

            <q-card-section class="q-pt-none">
              <q-form @submit="saveConfig">
                <!-- 批处理数量 -->
                <div class="q-gutter-y-md">
                  <q-input v-model.number="config.batchSize" label="批处理数量" type="number" min="1" max="1000" step="10"
                           dense
                           outlined>
                    <template v-slot:append>
                      <span class="text-grey-6 q-ml-sm">每次处理的文件数量</span>
                    </template>
                  </q-input>

                  <!-- 扫描深度 -->
                  <q-input v-model.number="config.scanDepth" label="扫描深度" type="number" min="1" max="20" step="1" dense
                           outlined>
                    <template v-slot:append>
                      <span class="text-grey-6 q-ml-sm">递归扫描的目录层级</span>
                    </template>
                  </q-input>

                  <!-- 扫描根目录 -->
                  <q-input v-model="config.rootDirectory" label="系统根目录" dense readonly outlined>
                    <template v-slot:append>
                      <q-btn color="grey" flat @click="selectRootDirectory" icon="folder" />
                    </template>
                  </q-input>

                  <!-- DryRun模式 -->
                  <div class="row items-center justify-between">
                    <div>
                      <div class="text-base">DryRun模式</div>
                      <div class="text-sm text-grey-6">开启时不会实际修改文件</div>
                    </div>
                    <q-toggle v-model="config.dryRun" color="primary" checked-icon="check" unchecked-icon="clear" />
                  </div>
                </div>
              </q-form>
            </q-card-section>

            <q-card-actions align="right" class="q-pt-none">
              <q-btn flat label="取消" @click="configDialog = false" />
              <q-btn flat label="恢复默认" @click="resetConfig" />
              <q-btn color="primary" label="保存" @click="saveConfig" />
            </q-card-actions>
          </q-card>
        </q-dialog>
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export default {
  name: 'App',
  data() {
    return {
      files: [],
      fileAll: [],
      filesAllLength: 100,
      loading: false,
      error: '',
      // 配置参数
      configDialog: false,
      config: {
        batchSize: 20,
        scanDepth: 5,
        rootDirectory: 'C:/',
        dryRun: true
      }
    }
  },
  mounted() {
    // 加载保存的配置
    this.loadConfig();
  },

  methods: {
    // 加载配置
    loadConfig() {
      try {
        const savedConfig = localStorage.getItem('fileTimeFixerConfig');
        if (savedConfig) {
          const parsedConfig = JSON.parse(savedConfig);
          this.config = {
            ...this.config,
            ...parsedConfig
          };
        }
      } catch (err) {
        console.warn('加载配置失败:', err);
        // 如果加载失败，使用默认配置
        this.resetConfig();
      }
    },

    // 演示事件
    async demonstrateEmitEvents() {
      try {
        await invoke('demonstrate_emit_events')
        console.log('演示事件成功触发')
      } catch (err) {
        this.error = '演示事件失败: ' + err
      }
    },

    async scanDirectory() {
      if (!this.config.rootDirectory.trim()) {
        this.error = '请输入目录路径'
        return
      }

      this.loading = true
      this.error = ''
      this.files = []
      this.fileAll = []
      this.filesAllLength = 0

      try {

        // 设置扫描进度事件监听器
        this.scanReadyUnlisten = await listen('scan_directory:ready', (event) => {
          // console.log('扫描进度:', event.payload)
          // this.files.push(event.payload)
          this.fileAll = event.payload
          this.filesAllLength = event.payload.length
          const random = Math.random() * 100
          this.$q.notify({
            type: 'info',
            message: `目录扫描完成，共找到 ${this.filesAllLength} 个文件`,
            group: false,
            transitionHide: 'fade',
            position: 'bottom-right'
          })
        })

        // 设置扫描进度事件监听器
        this.scanProgressUnlisten = await listen('scan_directory:progress', (event) => {
          // console.log('扫描进度:', event.payload)
          this.files.push(...event.payload)

          // 使用nextTick确保DOM更新后再滚动
          this.$nextTick(() => {
            // 获取虚拟滚动组件并滚动到底部
            if (this.$refs.fileList) {
              // this.$refs.fileList.scrollTo(this.files.length - 1)
            }
          })
        })

        // 设置扫描完成事件监听器
        this.scanCompleteUnlisten = await listen('scan_directory:complete', () => {
          this.loading = false
          // 扫描完成后显示通知
          this.$q.notify({
            message: `目录分析完成，共处理 ${this.files.length} 个文件`,
            type: 'positive',
            group: false,
            position: 'bottom-right',
            progress: true,
            timeout: 20000,
            actions: [
              { label: 'ok', color: 'white', handler: () => { /* ... */ } }
            ]
          })
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
        })

        // 启动扫描，但不等待完整结果
        invoke('scan_directory', { directory: this.config.rootDirectory, config: this.config })

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
    formatPath(fullPath) {
      if (!this.config.rootDirectory || !fullPath) {
        return fullPath;
      }

      // 确保路径格式一致
      const currentDir = this.config.rootDirectory.replace(/\\/g, '/');
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
    },

    // 显示配置对话框
    showConfigDialog() {
      this.configDialog = true;
    },

    // 选择根目录
    async selectRootDirectory() {
      try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          directory: false,
          multiple: false,
          title: '选择根目录'
        });
        if (selected) {
          this.config.rootDirectory = selected;
          return selected;
        }
      } catch (err) {
        this.error = '选择目录失败: ' + err;
      }
    },

    // 获取系统根目录
    getSystemRoot() {
      // 在浏览器环境中默认返回C盘根目录
      return 'C:/';

      // 注意：在Tauri运行时环境中，可以使用
      // window.__TAURI__?.process?.platform 来判断
    },

    // 保存配置
    saveConfig() {
      // 验证配置
      if (this.config.batchSize < 1) this.config.batchSize = 1;
      if (this.config.scanDepth < 1) this.config.scanDepth = 1;

      // 保存到本地存储
      try {
        localStorage.setItem('fileTimeFixerConfig', JSON.stringify(this.config));
      } catch (err) {
        console.warn('无法保存配置到本地存储:', err);
      }

      this.$q.notify({
        message: '配置已保存',
        color: 'positive',
        position: 'top'
      });

      this.configDialog = false;
    },

    // 恢复默认配置
    resetConfig() {
      this.config = {
        batchSize: 50,
        scanDepth: 5,
        rootDirectory: this.getSystemRoot(),
        dryRun: true
      };

      this.$q.notify({
        message: '配置已恢复默认',
        color: 'info',
        position: 'top'
      });
    }
  },

  // 清理事件监听器
  beforeUnmount() {
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