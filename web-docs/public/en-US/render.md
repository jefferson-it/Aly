# Graphics Rendering in Aly

Aly provides hardware-accelerated graphics rendering through OpenGL, Vulkan, and DirectX backends.

---

## 1. OpenGL Backend

```aly
import render.opengl

let ctx = render.opengl.create_context()
render.opengl.clear(0.0, 0.0, 0.0, 1.0)  # RGBA

let shader = render.opengl.compile_shader(vertex_src, fragment_src)
let vao = render.opengl.create_vao()
render.opengl.draw_arrays(vao, 3)  # Draw 3 vertices
render.opengl.swap_buffers(ctx)
```

---

## 2. Vulkan Backend

```aly
import render.vulkan

let instance = render.vulkan.create_instance("My App")
let device = render.vulkan.create_device(instance)
let pipeline = render.vulkan.create_graphics_pipeline(device, shader_config)
render.vulkan.submit(device, pipeline, command_buffer)
render.vulkan.destroy(device)
```

---

## 3. DirectX Backend

```aly
import render.directx

let device = render.directx.create_device()
let swapchain = render.directx.create_swapchain(device, hwnd)
render.directx.clear(device, 0.2, 0.3, 0.4, 1.0)
render.directx.present(swapchain)
```

> **Note**: DirectX is only available on Windows targets.
