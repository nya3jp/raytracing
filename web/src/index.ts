import {
    install_panic_hook,
    render,
    RenderParams,
} from 'engine-wasm';
import {memory} from '../../engine-wasm/pkg/engine_wasm_bg.wasm';

install_panic_hook();

function renderCanvas(data: Uint8Array, width: number, height: number): void {
    const canvasElem = document.getElementById('canvas') as HTMLCanvasElement;
    canvasElem.width = width;
    canvasElem.height = height;

    const ctx = canvasElem.getContext('2d');
    if (!ctx) {
        throw new Error('Canvas context unavailable');
    }

    // Convert RGB -> RGBA.
    const adata = new Uint8ClampedArray(width * height * 4);
    for (let i = 0; i < width * height; ++i) {
        adata[i*4+0] = data[i*3+0];
        adata[i*4+1] = data[i*3+1];
        adata[i*4+2] = data[i*3+2];
        adata[i*4+3] = 255;
    }

    ctx.putImageData(new ImageData(adata, width, height), 0, 0);
}

function onRender() {
    const scene = 'one_weekend::balls';
    const params = RenderParams.new(120, 80, 2, false);
    console.time('render');
    const result = render(scene, params);
    console.timeEnd('render');

    const data = new Uint8Array(memory.buffer, result.data(), 3 * result.width() * result.height());
    renderCanvas(data, result.width(), result.height());
}

interface Exports {
    onRender(): void
}
declare var window: {raytracing: Exports};
window.raytracing = { onRender };
