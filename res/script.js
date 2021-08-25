export function renderImage(resolution) {
    let svg = document.querySelector('.art'),
        target = document.querySelector('.image'),
        canvas = document.createElement('canvas'),
        ctx = canvas.getContext('2d'),
        image = new Image;

    image.width = canvas.width = resolution;
    image.height = canvas.height = resolution;
    image.onload = function () {
        ctx.drawImage(image, 0, 0, image.width, image.height);
        target.src = canvas.toDataURL();
    };
    image.src = 'data:image/svg+xml,' + encodeURIComponent((new XMLSerializer).serializeToString(svg));
}
