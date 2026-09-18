// A fake WebGL2 context and the handful of browser globals the renderer touches.
//
// It records every resource created and deleted and every listener added and removed, so a test can
// ask the only question that matters about teardown: is the ledger balanced?

export function createGlStub() {
  const created = { texture: [], buffer: [], program: [], shader: [], vao: [], framebuffer: [] }
  const deleted = { texture: [], buffer: [], program: [], shader: [], vao: [], framebuffer: [] }
  let nextId = 1

  const make = kind => {
    const handle = { kind, id: nextId }
    nextId += 1
    created[kind].push(handle)
    return handle
  }
  const drop = (kind, handle) => {
    if (handle) deleted[kind].push(handle)
  }

  const gl = {
    // Enough of the enum surface for the renderer to name what it means.
    VERTEX_SHADER: 0x8b31,
    FRAGMENT_SHADER: 0x8b30,
    ARRAY_BUFFER: 0x8892,
    STREAM_DRAW: 0x88e0,
    TRIANGLES: 4,
    FLOAT: 0x1406,
    TEXTURE_2D: 0x0de1,
    TEXTURE0: 0x84c0,
    RGBA: 0x1908,
    UNSIGNED_BYTE: 0x1401,
    TEXTURE_MIN_FILTER: 0x2801,
    TEXTURE_MAG_FILTER: 0x2800,
    LINEAR: 0x2601,
    NEAREST: 0x2600,
    LINEAR_MIPMAP_LINEAR: 0x2703,
    NEAREST_MIPMAP_NEAREST: 0x2700,
    BLEND: 0x0be2,
    SRC_ALPHA: 0x0302,
    ONE_MINUS_SRC_ALPHA: 0x0303,
    COLOR_BUFFER_BIT: 0x4000,
    LINK_STATUS: 0x8b82,
    COMPILE_STATUS: 0x8b81,
    MAX_TEXTURE_SIZE: 0x0d33,

    createShader: () => make('shader'),
    createProgram: () => make('program'),
    createBuffer: () => make('buffer'),
    createTexture: () => make('texture'),
    createVertexArray: () => make('vao'),
    createFramebuffer: () => make('framebuffer'),
    deleteShader: handle => drop('shader', handle),
    deleteProgram: handle => drop('program', handle),
    deleteBuffer: handle => drop('buffer', handle),
    deleteTexture: handle => drop('texture', handle),
    deleteVertexArray: handle => drop('vao', handle),
    deleteFramebuffer: handle => drop('framebuffer', handle),

    shaderSource: () => {},
    compileShader: () => {},
    attachShader: () => {},
    detachShader: () => {},
    linkProgram: () => {},
    useProgram: () => {},
    getProgramParameter: () => true,
    getShaderParameter: () => true,
    getShaderInfoLog: () => '',
    getProgramInfoLog: () => '',
    getAttribLocation: () => 0,
    getUniformLocation: () => ({ kind: 'uniform' }),
    getParameter: () => 8192,
    getExtension: () => null,
    bindBuffer: () => {},
    bufferData: () => {},
    enableVertexAttribArray: () => {},
    vertexAttribPointer: () => {},
    bindTexture: () => {},
    activeTexture: () => {},
    texImage2D: () => {},
    texParameteri: () => {},
    generateMipmap: () => {},
    uniform2f: () => {},
    uniform4f: () => {},
    uniform1i: () => {},
    drawArrays: () => {},
    enable: () => {},
    blendFunc: () => {},
    viewport: () => {},
    clearColor: () => {},
    clear: () => {},
  }

  return {
    gl,
    created,
    deleted,
    /** Resources made and never released. */
    leaked() {
      const leaks = {}
      for (const kind of Object.keys(created)) {
        const outstanding = created[kind].filter(handle => !deleted[kind].includes(handle))
        if (outstanding.length > 0) leaks[kind] = outstanding.length
      }
      return leaks
    },
  }
}

/** A canvas that records its listeners and hands out the stub context. */
export function createCanvasStub(gl, { width = 1200, height = 700 } = {}) {
  const listeners = []
  return {
    width,
    height,
    clientWidth: width,
    clientHeight: height,
    style: {},
    listeners,
    getContext: name => (name === 'webgl2' ? gl : null),
    getBoundingClientRect: () => ({ left: 0, top: 0, width, height }),
    addEventListener: (type, handler) => listeners.push({ type, handler }),
    removeEventListener: (type, handler) => {
      const index = listeners.findIndex(entry => entry.type === type && entry.handler === handler)
      if (index >= 0) listeners.splice(index, 1)
    },
    toDataURL: () => 'data:image/png;base64,',
  }
}

/**
 * Installs the browser globals the renderer reaches for, and returns a handle that undoes it and
 * reports what is still attached.
 */
export function installBrowserGlobals() {
  const saved = new Map()
  const windowListeners = []
  const observers = []
  const frames = new Set()
  let nextFrame = 1

  // Some of these are getter-only on the global object (navigator), so they are defined rather
  // than assigned, and put back the same way.
  const set = (name, value) => {
    saved.set(name, Object.getOwnPropertyDescriptor(globalThis, name))
    Object.defineProperty(globalThis, name, { value, configurable: true, writable: true })
  }

  const fakeWindow = {
    devicePixelRatio: 1,
    listeners: windowListeners,
    addEventListener: (type, handler) => windowListeners.push({ type, handler }),
    removeEventListener: (type, handler) => {
      const index = windowListeners.findIndex(
        entry => entry.type === type && entry.handler === handler,
      )
      if (index >= 0) windowListeners.splice(index, 1)
    },
    setInterval: () => 0,
    clearInterval: () => {},
  }

  set('window', fakeWindow)
  set('navigator', { getGamepads: () => [], maxTouchPoints: 0, clipboard: { writeText: async () => {} } })
  set('performance', globalThis.performance ?? { now: () => 0 })
  set('requestAnimationFrame', () => {
    const id = nextFrame
    nextFrame += 1
    frames.add(id)
    return id
  })
  set('cancelAnimationFrame', id => frames.delete(id))
  set(
    'ResizeObserver',
    class {
      constructor(callback) {
        this.callback = callback
        this.connected = false
        observers.push(this)
      }
      observe() {
        this.connected = true
      }
      disconnect() {
        this.connected = false
      }
    },
  )
  set(
    'Image',
    class {
      constructor() {
        this.onload = null
        this.src = ''
      }
    },
  )

  return {
    window: fakeWindow,
    windowListeners,
    observers,
    pendingFrames: () => frames.size,
    restore() {
      for (const [name, descriptor] of saved) {
        if (descriptor === undefined) delete globalThis[name]
        else Object.defineProperty(globalThis, name, descriptor)
      }
    },
  }
}
