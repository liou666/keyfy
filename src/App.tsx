import { createEffect,createSignal } from "solid-js";  
import { listen } from "@tauri-apps/api/event";  

function App() {  
  const [key, setKey] = createSignal("");  

  createEffect(() => {  
    const unlisten = listen("global-key-event", (event) => {  
      console.log(event.payload);
      setKey(event.payload); 
    });  

    return () => {  
      unlisten.then((fn) => fn()); 
    };  
  }, []);  

  return (  
    <div>  
      <h2>{key()}</h2>  
    </div>  
  );  
}  

export default App;
