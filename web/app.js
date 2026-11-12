Module.onRuntimeInitialized = () => {
    console.log("Wasm module loaded successfully.");

    const fileInput = document.getElementById('fileInput');
    const previewContainer = document.getElementById('preview-container');

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