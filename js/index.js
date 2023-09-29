import("../pkg/index.js").then(mod => {
    const canvas = document.getElementById("art");
    canvas.width = 500;
    canvas.height = 500;
    const ctx = canvas.getContext("2d");

    const renderer = new mod.FieldRenderer();

    const frameCounter = document.getElementById("frame-counter");

    let draw = () => {
        let frameCount = renderer.render_frame(ctx);

        frameCounter.innerText = `Frame count: ${frameCount}`;

        requestAnimationFrame(draw)
    }

    draw();
}).catch(console.error);
