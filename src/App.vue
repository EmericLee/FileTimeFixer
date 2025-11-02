<template>
  <div id="app">
    <header class="header">
      <h1>文件时间修复器</h1>
      <p>选择目录查看文件及其时间信息</p>
    </header>
    
    <main class="main">
      <div class="controls">
        <button @click="selectDirectory" class="btn-primary">选择目录</button>
        <input 
          v-model="currentDirectory" 
          @keyup.enter="loadDirectory" 
          placeholder="输入目录路径或点击选择"
          class="directory-input"
        />
        <button @click="loadDirectory" class="btn-secondary">加载</button>
      </div>
      
      <div v-if="loading" class="loading">加载中...</div>
      
      <div v-if="error" class="error">{{ error }}</div>
      
      <div v-if="files.length > 0" class="file-list-container">
        <div class="file-count">找到 {{ files.length }} 个文件</div>
        <div class="file-list">
          <div 
            v-for="file in files" 
            :key="file.path" 
            class="file-item"
            :class="{ 'is-directory': file.isDirectory }"
          >
            <div class="file-name">{{ file.name }}</div>
            <div class="file-info">
              <span class="file-size" v-if="!file.isDirectory">{{ formatFileSize(file.size) }}</span>
              <span class="file-time">{{ formatTime(file.modified) }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <div v-else-if="!loading" class="empty-state">
        <p>请选择一个目录来查看文件信息</p>
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
      return new Date(timestamp).toLocaleString('zh-CN')
    }
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background-color: #f5f5f5;
}

#app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem;
  text-align: center;
}

.header h1 {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
}

.header p {
  opacity: 0.9;
  font-size: 1.1rem;
}

.main {
  flex: 1;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.controls {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  align-items: center;
  flex-wrap: wrap;
}

.btn-primary, .btn-secondary {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.btn-primary {
  background: #667eea;
  color: white;
}

.btn-primary:hover {
  background: #5a6fd8;
  transform: translateY(-2px);
}

.btn-secondary {
  background: #e9ecef;
  color: #495057;
}

.btn-secondary:hover {
  background: #dee2e6;
}

.directory-input {
  flex: 1;
  min-width: 300px;
  padding: 0.75rem;
  border: 2px solid #e9ecef;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.3s ease;
}

.directory-input:focus {
  outline: none;
  border-color: #667eea;
}

.loading, .error, .empty-state {
  text-align: center;
  padding: 2rem;
  font-size: 1.2rem;
}

.error {
  color: #dc3545;
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 8px;
}

.empty-state {
  color: #6c757d;
}

.file-list-container {
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.file-count {
  padding: 1rem 1.5rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
  font-weight: 600;
  color: #495057;
}

.file-list {
  max-height: 600px;
  overflow-y: auto;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #f8f9fa;
  transition: background-color 0.2s ease;
}

.file-item:hover {
  background-color: #f8f9fa;
}

.file-item:last-child {
  border-bottom: none;
}

.file-item.is-directory {
  background-color: #fff3cd;
}

.file-item.is-directory:hover {
  background-color: #ffeaa7;
}

.file-name {
  font-weight: 500;
  color: #212529;
  word-break: break-all;
}

.file-info {
  display: flex;
  gap: 1rem;
  align-items: center;
  font-size: 0.9rem;
  color: #6c757d;
}

.file-size {
  font-weight: 600;
}

.file-time {
  min-width: 180px;
  text-align: right;
}

@media (max-width: 768px) {
  .controls {
    flex-direction: column;
    align-items: stretch;
  }
  
  .directory-input {
    min-width: auto;
  }
  
  .file-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .file-info {
    width: 100%;
    justify-content: space-between;
  }
}
</style>