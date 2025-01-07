import type { Event } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { PhysicalSize, Window } from '@tauri-apps/api/window'
import { createEffect, createSignal, on, onMount } from 'solid-js'
import './app.css'

// async function resizeToContent() {
//   const contentWidth = document.documentElement.scrollWidth
//   const contentHeight = document.documentElement.scrollHeight
//   console.log(contentWidth, contentHeight)
//   await Window.getCurrent().setSize(new PhysicalSize(contentWidth, contentHeight))
// }

function debounce(func: Function, wait: number) {
  let timeout: number | null = null
  return function (this: any, ...args: any[]) {
    const later = () => {
      timeout = null
      func.apply(this, args)
    }
    clearTimeout(timeout!)
    timeout = setTimeout(later, wait)
  }
}

function App() {
  const [key, setKey] = createSignal<string>('')
  const [timmer, setTimmer] = createSignal<number | null>(null)

  onMount(() => {
    const dbWindowToggle = debounce(async () => {
      setKey('')
      await invoke('handle_close_requested', {
        window: Window.getCurrent(),
        isShow: false,
      })
    }, 1000)

    listen<string>('global-key-event', async (event: Event<string>) => {
      console.log(event.payload, 1)
      const isShow = await Window.getCurrent().isVisible()
      // await Window.getCurrent().setFocus(false)
      if (!isShow) {
        await invoke('handle_close_requested', {
          window: Window.getCurrent(),
          isShow: true,
        })
      }

      setKey(pre => pre += event.payload)
      dbWindowToggle()
    })
  })

  createEffect(on(key, () => {
    const div = document.querySelector('.text-container') as HTMLDivElement
    if (div)
      div.scrollTop = div.scrollHeight
  }))

  return (
    <div data-tauri-drag-region class="w-[400px] h-[110px] flex items-center justify-center rounded-lg p-4 box-border bg-[rgba(0,0,0,0.5)]">
      <div data-tauri-drag-region class="text-container text-white font-bold box-border text-2xl  max-h-full max-w-full overflow-hidden leading-10">
        {key()}
      </div>
    </div>
  )
}

export default App
