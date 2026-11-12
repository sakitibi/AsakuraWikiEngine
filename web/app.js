Module.onRuntimeInitialized = () => {
    console.log("Wasm module loaded successfully.");

    const fileInput = document.getElementById('fileInput');
    const previewContainer = document.getElementById('preview-container');

    const OPEN_ICON_PATH = '<path fill="currentColor" d="M384 32H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64h320c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM320 272H128c-13.3 0-24-10.7-24-24s10.7-24 24-24h192c13.3 0 24 10.7 24 24s-10.7 24-24 24z"></path>';
    const CLOSE_ICON_PATH = '<path fill="currentColor" d="M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM200 344l0-64-64 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l64 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 64 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-64 0 0 64c0 13.3-10.7 24-24 24s-24-10.7-24-24z"></path>';

    previewContainer.addEventListener('click', (event) => {
        const header = event.target.closest('.accordion-header');
        if (!header) return;

        const container = header.closest('.accordion-container');
        const content = container?.querySelector('.accordion-content');
        const svg = header.querySelector('svg');
        if (!content || !svg) return;

        if (content.classList.contains('accordion-open')) {
            content.classList.remove('accordion-open');
            content.classList.add('accordion-close');
            svg.innerHTML = CLOSE_ICON_PATH;
        } else {
            content.classList.remove('accordion-close');
            content.classList.add('accordion-open');
            svg.innerHTML = OPEN_ICON_PATH;
        }
    });

    fileInput.addEventListener('change', async (event) => {
        const file = event.target.files[0];
        if (!file) return;

        try {
            const arrayBuffer = await file.arrayBuffer();
            const byteArray = new Uint8Array(arrayBuffer);

            if (byteArray.length === 0) {
                alert("空のファイルです。");
                return;
            }

            const bufferPtr = Module._malloc(byteArray.length);
            Module.HEAPU8.set(byteArray, bufferPtr);

            const htmlPtr = Module._render_wiki("emerald", bufferPtr, byteArray.length);
            const renderedHtml = Module.UTF8ToString(htmlPtr);

            Module._free(bufferPtr);

            previewContainer.innerHTML = renderedHtml;

        } catch (error) {
            console.error("Wasm rendering error:", error);
            previewContainer.innerHTML = `<p style="color: red;">エラーが発生しました: ${error.message}</p>`;
        }
    });
};