<script>
  import { recognizeText } from 'tauri-plugin-text-recognition-api'
  import { open } from '@tauri-apps/plugin-dialog'
  import { mkdir, readFile, writeFile } from '@tauri-apps/plugin-fs'
  import { appCacheDir, join } from '@tauri-apps/api/path'

  const MAX_SIDE = 2048

  let shrink = $state(false)
  let busy = $state(false)
  let imageUrl = $state(null)
  let result = $state(null)
  let took = $state(0)
  let error = $state('')
  let selected = $state(null)

  let lineCount = $derived(result ? result.blocks.reduce((n, b) => n + b.lines.length, 0) : 0)

  async function fromInput(event) {
    const file = event.currentTarget.files?.[0]
    event.currentTarget.value = ''
    if (file) await read(file, null)
  }

  async function fromPicker() {
    const uri = await open({
      filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp', 'heic'] }],
    })
    if (uri) await read(new Blob([await readFile(uri)]), uri)
  }

  async function read(blob, uri) {
    busy = true
    error = ''
    result = null
    selected = null
    try {
      const image = shrink ? await shrunk(blob) : blob
      show(image)
      const path = uri && !shrink ? uri : await saveToCache(image)
      const started = performance.now()
      result = await recognizeText({ path })
      took = Math.round(performance.now() - started)
    } catch (e) {
      error = String(e)
    } finally {
      busy = false
    }
  }

  function show(blob) {
    if (imageUrl) URL.revokeObjectURL(imageUrl)
    imageUrl = URL.createObjectURL(blob)
  }

  // Always the same file, so trying many photos does not fill the cache.
  async function saveToCache(blob) {
    const dir = await appCacheDir()
    await mkdir(dir, { recursive: true })
    const path = await join(dir, 'image-to-read')
    await writeFile(path, new Uint8Array(await blob.arrayBuffer()))
    return path
  }

  async function shrunk(blob) {
    const bitmap = await createImageBitmap(blob)
    const scale = Math.min(1, MAX_SIDE / Math.max(bitmap.width, bitmap.height))
    const canvas = document.createElement('canvas')
    canvas.width = Math.round(bitmap.width * scale)
    canvas.height = Math.round(bitmap.height * scale)
    canvas.getContext('2d').drawImage(bitmap, 0, 0, canvas.width, canvas.height)
    bitmap.close()
    return await new Promise((resolve) => canvas.toBlob(resolve, 'image/jpeg', 0.9))
  }

  function percent(value) {
    return value === null ? '–' : `${Math.round(value * 100)} %`
  }
</script>

<main>
  <h1>Text recognition</h1>

  <div class="actions">
    <label class="button">
      Take a photo
      <input type="file" accept="image/*" capture="environment" onchange={fromInput} disabled={busy} />
    </label>
    <button onclick={fromPicker} disabled={busy}>Choose an image</button>
  </div>
  <label class="option">
    <input type="checkbox" bind:checked={shrink} disabled={busy} />
    Shrink to {MAX_SIDE} px before reading
  </label>

  {#if busy}<p>Reading…</p>{/if}
  {#if error}<p class="error">{error}</p>{/if}

  {#if result}
    <p class="summary">
      {result.imageWidth} × {result.imageHeight} px · read in {took} ms ·
      {result.blocks.length} blocks · {lineCount} lines
    </p>
  {/if}

  {#if imageUrl}
    <div class="frame">
      <img src={imageUrl} alt="The image that was read" />
      {#if result}
        <svg viewBox="0 0 {result.imageWidth} {result.imageHeight}" preserveAspectRatio="none">
          {#each result.blocks as block, i}
            {#if block.bounds}
              <rect
                class="block"
                class:selected={selected === i}
                x={block.bounds.left}
                y={block.bounds.top}
                width={block.bounds.width}
                height={block.bounds.height}
                vector-effect="non-scaling-stroke"
              />
            {/if}
            {#each block.lines as line}
              {#if line.bounds}
                <rect
                  class="line"
                  x={line.bounds.left}
                  y={line.bounds.top}
                  width={line.bounds.width}
                  height={line.bounds.height}
                  vector-effect="non-scaling-stroke"
                />
              {/if}
            {/each}
          {/each}
        </svg>
      {/if}
    </div>
  {/if}

  {#if result}
    {#if result.blocks.length === 0}
      <p>No text found.</p>
    {/if}
    <ol class="blocks">
      {#each result.blocks as block, i}
        <li class:selected={selected === i}>
          <button class="block-button" onclick={() => (selected = selected === i ? null : i)}>
            {#each block.lines as line}
              <span class="line-text">
                {line.text}
                <small>{percent(line.confidence)} · {line.angle === null ? '–' : `${Math.round(line.angle)}°`}</small>
              </span>
            {/each}
          </button>
        </li>
      {/each}
    </ol>
  {/if}
</main>

<style>
  main {
    padding: 16px;
    max-width: 960px;
    margin: 0 auto;
  }

  h1 {
    font-size: 1.4rem;
  }

  .actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  button,
  .button {
    font: inherit;
    padding: 10px 16px;
    border-radius: 8px;
    border: 1px solid #888;
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  .button input {
    display: none;
  }

  .option {
    display: block;
    margin: 12px 0;
  }

  .error {
    color: #d33;
  }

  .summary {
    font-variant-numeric: tabular-nums;
  }

  .frame {
    position: relative;
  }

  .frame img {
    display: block;
    width: 100%;
  }

  .frame svg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  rect {
    fill: none;
  }

  .block {
    stroke: #f0a;
    stroke-width: 2;
  }

  .block.selected {
    stroke: #fd0;
    stroke-width: 4;
  }

  .line {
    stroke: #0cf;
    stroke-width: 1;
  }

  .blocks {
    padding-left: 24px;
  }

  .blocks li.selected {
    background: rgb(255 221 0 / 0.2);
  }

  .block-button {
    border: none;
    padding: 4px 0;
    text-align: left;
    width: 100%;
  }

  .line-text {
    display: block;
  }

  small {
    opacity: 0.6;
    white-space: nowrap;
  }
</style>
