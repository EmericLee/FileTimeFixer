import { defineConfig, presetUno, transformerDirectives } from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
  ],
  transformers: [
    transformerDirectives(),
  ],
  safelist: ['btn', 'btn-primary', 'alert', 'alert-danger', 'card', 'card-header', 'card-body'],
})