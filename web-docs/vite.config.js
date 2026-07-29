import { defineConfig } from 'vite';

export default defineConfig({
  root: '.',
  base: '/aly/',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
  server: {
    port: 3000,
  },
});
