const canvasSize = 3100;
const canvasStyleSize = "700px";

import("../pkg/index.js").then(mod => {
    const canvas = document.getElementById("art");
    canvas.width = canvasSize;
    canvas.height = canvasSize;
    canvas.style.width = canvasStyleSize;
    canvas.style.height = canvasStyleSize;
    const ctx = canvas.getContext("2d");

    const overlay = document.getElementById("overlay");
    overlay.width = canvasSize;
    overlay.height = canvasSize;
    overlay.style.width = canvasStyleSize;
    overlay.style.height = canvasStyleSize;
    const overlayCtx = overlay.getContext("2d");

    const renderer = new mod.FieldRenderer();
    renderer.init(ctx);

    const frameCounter = document.getElementById("frame-counter");
    const frameRate = document.getElementById("frame-rate");

    let playing = true;
    let lastTimestamp = performance.now();

    let draw = () => {
        let frameCount = renderer.render_frame(ctx);
        frameCounter.innerText = `Frames: ${frameCount}`;

        renderer.render_overlay(overlayCtx);

        if (frameCount % 10 === 0) {
            const fps = 10 / ((performance.now() - lastTimestamp) / 1000);
            frameRate.innerText = `fps: ${fps.toFixed(2)}`
            lastTimestamp = performance.now();
        }

        if (!playing) return;
        requestAnimationFrame(draw)
    }

    const playButton = document.getElementById("pause-play");
    playButton.addEventListener("click", handlePausePlay);
    function handlePausePlay() {
        if (playing) {
            playing = false;
            playButton.innerText = "Play";
        } else {
            playing = true;
            playButton.innerText = "Pause";
            draw();
        }
    }

    const captureButton = document.getElementById("capture");
    captureButton.addEventListener("click", handleCaptureImage);
    function handleCaptureImage() {
        const imageUrl = canvas.toDataURL();
        const link = document.createElement("a");
        link.download = "flow";
        link.href = imageUrl;
        link.click();
    }

    const overlayButton = document.getElementById("overlay-show-hide");
    overlayButton.addEventListener("click", handleOverlay);
    let showOverlay = true;
    // overlay.style.display = "none";
    function handleOverlay() {
        showOverlay = !showOverlay;
        overlay.style.display = showOverlay ? "block" : "none";
    }

    draw();
}).catch(console.error);
