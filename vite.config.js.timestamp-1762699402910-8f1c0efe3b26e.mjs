// vite.config.js
import { defineConfig } from "file:///D:/Devs/FileTimeFixer/node_modules/.pnpm/vite@4.5.14_@types+node@24.10.0_sass@1.32.12/node_modules/vite/dist/node/index.js";
import vue from "file:///D:/Devs/FileTimeFixer/node_modules/.pnpm/@vitejs+plugin-vue@4.6.2_vi_ddaebc20a21bdae220959a1d427f3ea4/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { quasar, transformAssetUrls } from "file:///D:/Devs/FileTimeFixer/node_modules/.pnpm/@quasar+vite-plugin@1.10.0__9ace6aa4bbe72c1181cfade125853b92/node_modules/@quasar/vite-plugin/src/index.js";
import VueDevTools from "file:///D:/Devs/FileTimeFixer/node_modules/.pnpm/vite-plugin-vue-devtools@7._644a1f863e8a50f2daf91633481f6621/node_modules/vite-plugin-vue-devtools/dist/vite.mjs";
var vite_config_default = defineConfig(async () => ({
  plugins: [
    vue({
      template: { transformAssetUrls }
    }),
    quasar({
      sassVariables: "src/quasar-variables.sass"
    }),
    // 添加Vue DevTools插件
    VueDevTools({
      standalone: true
    })
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  },
  // Build configuration
  build: {
    outDir: "dist",
    assetsDir: "assets",
    sourcemap: true,
    // 启用源映射以便更好地调试
    minify: "esbuild",
    target: "esnext"
  },
  // 优化依赖
  optimizeDeps: {
    include: ["vue", "quasar", "vite-plugin-vue-devtools"]
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcuanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxEZXZzXFxcXEZpbGVUaW1lRml4ZXJcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkQ6XFxcXERldnNcXFxcRmlsZVRpbWVGaXhlclxcXFx2aXRlLmNvbmZpZy5qc1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRDovRGV2cy9GaWxlVGltZUZpeGVyL3ZpdGUuY29uZmlnLmpzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHZ1ZSBmcm9tIFwiQHZpdGVqcy9wbHVnaW4tdnVlXCI7XHJcbmltcG9ydCB7IHF1YXNhciwgdHJhbnNmb3JtQXNzZXRVcmxzIH0gZnJvbSAnQHF1YXNhci92aXRlLXBsdWdpbic7XHJcbmltcG9ydCBWdWVEZXZUb29scyBmcm9tICd2aXRlLXBsdWdpbi12dWUtZGV2dG9vbHMnO1xyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XHJcbiAgcGx1Z2luczogW1xyXG4gICAgdnVlKHtcclxuICAgICAgdGVtcGxhdGU6IHsgdHJhbnNmb3JtQXNzZXRVcmxzIH1cclxuICAgIH0pLFxyXG4gICAgcXVhc2FyKHtcclxuICAgICAgc2Fzc1ZhcmlhYmxlczogJ3NyYy9xdWFzYXItdmFyaWFibGVzLnNhc3MnXHJcbiAgICB9KSxcclxuICAgIC8vIFx1NkRGQlx1NTJBMFZ1ZSBEZXZUb29sc1x1NjNEMlx1NEVGNlxyXG4gICAgVnVlRGV2VG9vbHMoe1xyXG4gICAgICBzdGFuZGFsb25lOiB0cnVlXHJcbiAgICB9KVxyXG4gIF0sXHJcblxyXG4gIC8vIFZpdGUgb3B0aW9ucyB0YWlsb3JlZCBmb3IgVGF1cmkgZGV2ZWxvcG1lbnQgYW5kIG9ubHkgYXBwbGllZCBpbiBgdGF1cmkgZGV2YCBvciBgdGF1cmkgYnVpbGRgXHJcbiAgLy9cclxuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcclxuICBjbGVhclNjcmVlbjogZmFsc2UsXHJcbiAgLy8gMi4gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICBzZXJ2ZXI6IHtcclxuICAgIHBvcnQ6IDE0MjAsXHJcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gICAgd2F0Y2g6IHtcclxuICAgICAgLy8gMy4gdGVsbCB2aXRlIHRvIGlnbm9yZSB3YXRjaGluZyBgc3JjLXRhdXJpYFxyXG4gICAgICBpZ25vcmVkOiBbXCIqKi9zcmMtdGF1cmkvKipcIl0sXHJcbiAgICB9LFxyXG4gIH0sXHJcblxyXG4gIC8vIEJ1aWxkIGNvbmZpZ3VyYXRpb25cclxuICBidWlsZDoge1xyXG4gICAgb3V0RGlyOiAnZGlzdCcsXHJcbiAgICBhc3NldHNEaXI6ICdhc3NldHMnLFxyXG4gICAgc291cmNlbWFwOiB0cnVlLCAvLyBcdTU0MkZcdTc1MjhcdTZFOTBcdTY2MjBcdTVDMDRcdTRFRTVcdTRGQkZcdTY2RjRcdTU5N0RcdTU3MzBcdThDMDNcdThCRDVcclxuICAgIG1pbmlmeTogJ2VzYnVpbGQnLFxyXG4gICAgdGFyZ2V0OiAnZXNuZXh0J1xyXG4gIH0sXHJcblxyXG4gIC8vIFx1NEYxOFx1NTMxNlx1NEY5RFx1OEQ1NlxyXG4gIG9wdGltaXplRGVwczoge1xyXG4gICAgaW5jbHVkZTogWyd2dWUnLCAncXVhc2FyJywgJ3ZpdGUtcGx1Z2luLXZ1ZS1kZXZ0b29scyddXHJcbiAgfVxyXG59KSk7Il0sCiAgIm1hcHBpbmdzIjogIjtBQUF1UCxTQUFTLG9CQUFvQjtBQUNwUixPQUFPLFNBQVM7QUFDaEIsU0FBUyxRQUFRLDBCQUEwQjtBQUMzQyxPQUFPLGlCQUFpQjtBQUd4QixJQUFPLHNCQUFRLGFBQWEsYUFBYTtBQUFBLEVBQ3ZDLFNBQVM7QUFBQSxJQUNQLElBQUk7QUFBQSxNQUNGLFVBQVUsRUFBRSxtQkFBbUI7QUFBQSxJQUNqQyxDQUFDO0FBQUEsSUFDRCxPQUFPO0FBQUEsTUFDTCxlQUFlO0FBQUEsSUFDakIsQ0FBQztBQUFBO0FBQUEsSUFFRCxZQUFZO0FBQUEsTUFDVixZQUFZO0FBQUEsSUFDZCxDQUFDO0FBQUEsRUFDSDtBQUFBO0FBQUE7QUFBQTtBQUFBLEVBS0EsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixPQUFPO0FBQUE7QUFBQSxNQUVMLFNBQVMsQ0FBQyxpQkFBaUI7QUFBQSxJQUM3QjtBQUFBLEVBQ0Y7QUFBQTtBQUFBLEVBR0EsT0FBTztBQUFBLElBQ0wsUUFBUTtBQUFBLElBQ1IsV0FBVztBQUFBLElBQ1gsV0FBVztBQUFBO0FBQUEsSUFDWCxRQUFRO0FBQUEsSUFDUixRQUFRO0FBQUEsRUFDVjtBQUFBO0FBQUEsRUFHQSxjQUFjO0FBQUEsSUFDWixTQUFTLENBQUMsT0FBTyxVQUFVLDBCQUEwQjtBQUFBLEVBQ3ZEO0FBQ0YsRUFBRTsiLAogICJuYW1lcyI6IFtdCn0K
