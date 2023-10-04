import("../pkg/index.js").then(mod => {
    const canvas = document.getElementById("art");
    canvas.width = 700;
    canvas.height = 700;
    const ctx = canvas.getContext("2d");

    const renderer = new mod.FieldRenderer();

    const frameCounter = document.getElementById("frame-counter");

    let playing = true;

    let draw = () => {
        let frameCount = renderer.render_frame(ctx);
        frameCounter.innerText = `Frame count: ${frameCount}`;
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

    draw();
}).catch(console.error);
