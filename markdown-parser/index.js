const rust = import('./pkg/index.js')

rust.then(wasm => {

  const btn = document.getElementById('parse')
  const previewArea = document.getElementById('output')

  btn.addEventListener('click', () => {
    const input = document.getElementById('markdown').value
    previewArea.innerHTML = wasm.parse(input)
  })

})

