
my_image_app = {
    UpdateUI(from_rust_str) {
        document.getElementById("messages").append(from_rust_str + "<br/><hr/>")
    },
    // This function is called from Rust
    show_file_preview(url, mime) {
        const container = document.getElementById('preview');
        const el = document.createElement('div');
        el.style.margin = '10px';
        el.style.display = 'inline-block';

        if (mime.startsWith('image/')) {
            const img = document.createElement('img');
            img.src = url;
            img.style.maxWidth = '300px';
            el.appendChild(img);
        } else if (mime.startsWith('video/')) {
            const video = document.createElement('video');
            video.src = url;
            video.controls = true;
            video.style.maxWidth = '300px';
            el.appendChild(video);
        } else {
            const a = document.createElement('a');
            a.href = url;
            a.textContent = 'Download ' + url.split('/').pop();
            a.download = '';
            el.appendChild(a);
        }

        container.appendChild(el);
    }
}