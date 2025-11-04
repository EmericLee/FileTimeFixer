<template>
  <div id="app">
    <header>
      <div class="container mx-auto px-4 py-6">
        <h1 class="text-2xl md:text-3xl font-bold text-white text-center">文件时间修复器</h1>
        <p class="text-white/90 text-center">高效管理和修复文件时间信息</p>
      </div>
    </header>
    
    <main>
      <div class="container mx-auto px-4 h-full">
        <!-- 控制区域 - 优化的Grid布局 -->
        <div class="grid grid-cols-12 gap-2 mt-2">
          <div class="col-span-12">
            <div class="grid grid-cols-12 gap-2 auto-cols-fr">
              <!-- 输入框在移动端占满宽度，在桌面端占较大比例 -->
              <div class="col-span-12 sm:col-span-7 md:col-span-6">
                <input 
                  v-model="currentDirectory" 
                  @keyup.enter="loadDirectory" 
                  placeholder="输入目录路径或点击选择"
                  class="w-full  px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  type="text"
                  autocomplete="off"
                />
              </div>
              <!-- 按钮在移动端垂直堆叠 -->
              <div class="col-span-6 sm:col-span-2 md:col-span-2">
                <button @click="selectDirectory" class="w-full h-full bg-blue-600 text-white px-3 py-2 rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50">选择</button>
              </div>
              <div class="col-span-6 sm:col-span-3 md:col-span-2">
                <button @click="loadDirectory" class="w-full h-full bg-gray-200 text-gray-800 px-3 py-2 rounded-md hover:bg-gray-300 transition-colors disabled:opacity-50" :disabled="loading">
                  {{ loading ? '加载中...' : '加载' }}
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 错误提示区域 -->
        <div v-if="error" class="mt-4 bg-red-100 border border-red-300 text-red-700 px-4 py-3 rounded-md">
          <strong class="font-bold">错误：</strong>{{ error }}
        </div>
        
        <!-- 文件列表区域 -->
        <div v-if="files.length > 0" class="mt-4 border border-gray-200 rounded-lg overflow-hidden">
          <div class="bg-blue-50 p-3 border-b border-gray-200">
            <h3 class="text-lg font-medium m-0">文件列表</h3>
            <p class="text-sm m-0 text-gray-600">找到 {{ files.length }} 个文件</p>
          </div>
          <div class="file-list">
            <!-- 表头 - 使用固定宽度的网格布局 -->
            <div class="file-item bg-gray-100 font-medium grid grid-cols-12 gap-4">
              <div class="col-span-12 sm:col-span-7 font-medium">文件名</div>
              <div class="col-span-4 sm:col-span-2 font-medium text-right">大小</div>
              <div class="col-span-8 sm:col-span-3 font-medium text-right">修改时间</div>
            </div>
            <!-- 文件项 - 使用相同的网格布局 -->
            <div 
              v-for="file in files" 
              :key="file.path" 
              class="file-item hover:bg-gray-50 transition-colors grid grid-cols-12 gap-4"
              :class="{ 'is-directory': file.isDirectory }"
            >
              <div class="col-span-12 sm:col-span-7 truncate" title="{{ file.name }}">{{ file.name }}</div>
              <span class="col-span-4 sm:col-span-2 text-right" v-if="!file.isDirectory">{{ formatFileSize(file.size) }}</span>
              <span class="col-span-4 sm:col-span-2 text-right" v-else>-</span>
              <span class="col-span-8 sm:col-span-3 text-right">{{ formatTime(file.modified) }}</span>
            </div>
          </div>
        </div>
        
        <!-- 空状态区域 -->
        <div v-else-if="!loading" class="mt-4 bg-blue-50 border border-blue-200 px-4 py-3 rounded-md">
          <p class="m-0">请选择一个目录来查看文件信息</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'App',
  data() {
    return {
      currentDirectory: '',
      files: [],
      loading: false,
      error: ''
    }
  },
  methods: {
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
        const result = await invoke('list_files', { directory: this.currentDirectory })
        this.files = result
        console.log(this.files)
      } catch (err) {
        this.error = '加载目录失败: ' + err
      } finally {
        this.loading = false
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
    }
  }
}
</script>

<style>
/* 基础布局结构样式 */
html {
  font-size: 85%; /* 整体字体缩小到60% */
}

html, body {
  height: 100%;
  overflow: hidden;
}

#app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

/* 主内容区域设置，确保文件列表可滚动 */
main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

main > .container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 自定义导航栏样式 */
header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

/* 文件列表样式 - 优化的Grid布局 */
.file-list {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1px;
  height: 100%;
  overflow-y: auto;
  background-color: #e5e7eb;
}

/* 自定义滚动条 */
.file-list::-webkit-scrollbar {
  width: 6px;
}

.file-list::-webkit-scrollbar-track {
  background: #f3f4f6;
}

.file-list::-webkit-scrollbar-thumb {
  background: #9ca3af;
  border-radius: 3px;
}

.file-list::-webkit-scrollbar-thumb:hover {
  background: #6b7280;
}

/* 文件项基础样式 - 现在使用组件内的grid布局 */
.file-item {
  padding: 0.75rem 1rem;
  background-color: white;
  align-items: center;
}

/* 响应式优化 - 移动设备 */
@media (max-width: 639px) {
  /* 表头在移动设备上隐藏 */
  .file-item.bg-gray-100 {
    display: none;
  }
  
  /* 文件项在移动端的样式调整 */
  .file-item:not(.bg-gray-100) {
    grid-template-rows: auto auto;
  }
  
  /* 第二行显示大小和时间信息 */
  .file-item .col-span-4,
  .file-item .col-span-8 {
    font-size: 0.85rem;
    color: #6b7280;
    display: inline-block;
  }
}

/* 平板和桌面设备的优化 */
@media (min-width: 640px) {
  /* 确保表格对齐良好 */
  .file-item {
    padding: 0.6rem 1rem;
  }
  
  /* 目录项样式区分 */
  .file-item.is-directory {
    background-color: #f9fafb;
  }
  
  .file-item.is-directory .file-name {
    font-weight: 500;
  }
}
</style>