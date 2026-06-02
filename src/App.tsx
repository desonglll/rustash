import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    const result = await invoke<string>("greet", { name });
    setGreetMsg(result);
  }

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-center">
      <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-8">
        Rustash
      </h1>
      <div className="flex gap-2">
        <input
          id="greet-input"
          className="px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button
          className="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-medium transition-colors"
          onClick={greet}
        >
          Greet
        </button>
      </div>
      <p className="mt-4 text-gray-700 dark:text-gray-300">{greetMsg}</p>
    </div>
  );
}

export default App;
