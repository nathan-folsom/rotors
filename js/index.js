import("../pkg/index.js").then(mod => {
    const canvas = document.getElementById("art");
    canvas.width = 700;
    canvas.height = 700;
    const ctx = canvas.getContext("2d");

    const renderer = new mod.FieldRenderer();

    const frameCounter = document.getElementById("frame-counter");
    const frameRate = document.getElementById("frame-rate");

    let playing = true;
    let lastTimestamp = performance.now();

    let draw = () => {
        let frameCount = renderer.render_frame(ctx);
        frameCounter.innerText = `Frames: ${frameCount}`;

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

    draw();
}).catch(console.error);
