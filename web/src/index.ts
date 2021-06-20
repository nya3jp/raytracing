import {
    get_all_scenes,
    install_panic_hook,
    render,
    RenderParams,
} from 'engine-wasm';
import {memory} from '../../engine-wasm/pkg/engine_wasm_bg.wasm';

install_panic_hook();

const ALL_SCENES = get_all_scenes();

const DEFAULT_RENDER_PARAMS = RenderParams.new('book1/final', 120, 80, 10, false);

const FORM = document.forms.namedItem('config')!;
const FORM_SCENE_NAME = FORM.elements.namedItem('scene_name') as HTMLSelectElement;
const FORM_WIDTH = FORM.elements.namedItem('width') as HTMLInputElement;
const FORM_HEIGHT = FORM.elements.namedItem('height') as HTMLInputElement;
const FORM_SAMPLES_PER_PIXEL = FORM.elements.namedItem('samples_per_pixel') as HTMLInputElement;
const FORM_IMPORTANCE_SAMPLING = FORM.elements.namedItem('importance_sampling') as HTMLInputElement;

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
    const params = getRenderParams();
    console.time('render');
    const result = render(params);
    console.timeEnd('render');

    const data = new Uint8Array(memory.buffer, result.data(), 3 * result.width() * result.height());
    renderCanvas(data, result.width(), result.height());
}

function getRenderParams() {
    return RenderParams.new(
        FORM_SCENE_NAME.value,
        parseInt(FORM_WIDTH.value),
        parseInt(FORM_HEIGHT.value),
        parseInt(FORM_SAMPLES_PER_PIXEL.value),
        FORM_IMPORTANCE_SAMPLING.checked);
}

function updateForm(params: RenderParams) {
    FORM_SCENE_NAME.value = params.scene_name;
    FORM_WIDTH.value = String(params.width);
    FORM_HEIGHT.value = String(params.height);
    FORM_SAMPLES_PER_PIXEL.value = String(params.samples_per_pixel);
    FORM_IMPORTANCE_SAMPLING.checked = params.importance_sampling;
}

function onSceneNameChange() {
    const scene_name = FORM_SCENE_NAME.value;
    const params = ALL_SCENES.find((params) => params.scene_name === scene_name)!;
    updateForm(params);
}

function onInit() {
    // Fill <select name="scene_name">.
    for (const params of ALL_SCENES) {
        const optionElem = document.createElement('option');
        optionElem.value = params.scene_name;
        optionElem.textContent = params.scene_name;
        FORM_SCENE_NAME.add(optionElem);
    }
    FORM_SCENE_NAME.value = ALL_SCENES[0].scene_name;

    // Install the onchange handler.
    FORM_SCENE_NAME.addEventListener('change', onSceneNameChange);

    updateForm(DEFAULT_RENDER_PARAMS);
}

interface Exports {
    onRender(): void
}
declare var window: {raytracing: Exports};
window.raytracing = { onRender };

onInit();
