// シンプルなindex.jsのインポートのみ
import("./index.js")
  .catch(e => {
    console.error("Failed to load application:", e);
    document.getElementById('output').textContent = 'Failed to load application: ' + e;
  });