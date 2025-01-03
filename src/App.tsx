import { createEffect, createSignal } from 'solid-js'
import { listen, Event } from '@tauri-apps/api/event'
import './app.css'

import { Window } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
function App() {
  const [key, setKey] = createSignal<string>('')

  createEffect(() => {
    const unlisten = listen<string>('global-key-event', (event: Event<string>) => {
      console.log(event.payload)
      setKey(event.payload)
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [])

  function handleClick() {
    console.log('click')
    document.body.classList.add('fade-out');  
    // invoke('handle_close_requested', Window.getCurrent(),false)
    // setTimeout(() => {
    //   invoke('handle_close_requested', {  
    //     window: Window.getCurrent(),  
    //     isShow: true  
    // })  
    // },3000)
    // invoke('handle_close_requested', {  
    //   window: Window.getCurrent(),  
    //   isShow: false  
    // })  
    
    setTimeout(async () => {  
      await invoke('handle_close_requested', {  
        window: Window.getCurrent(),  
        isShow: false  
      });  
      document.body.classList.remove('fade-out');  
    }, 800); 
  }

  return (
    <div class="w-[100vw] h-[100vh] flex items-center justify-center overflow-hidden">
      <button class='bg-yellow-400' onClick={() => handleClick()}>click</button>

      <div data-tauri-drag-region class=" w-[100vw] h-[100vh] text-white font-bold">
        {key()}
      </div>
    </div>
  )
}

export default App
