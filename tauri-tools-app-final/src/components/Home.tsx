import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export const Home = () => {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [count, setCount] = useState(0);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function increment() {
    setCount(await invoke("increment", { count }));
  }

  return (
    <div className="container my-5 p-4 bg-dark text-light rounded shadow">
      <header className="text-center mb-4">
        <h1 className="display-5 text-warning">Welcome to Tools App</h1>
      </header>

      <section className="mb-5">
        <form
          className="row g-3 justify-content-center"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <div className="col-md-6">
            <input
              id="greet-input"
              type="text"
              className="form-control bg-dark text-light border-secondary"
              placeholder="Enter a name..."
              onChange={(e) => setName(e.currentTarget.value)}
            />
          </div>
          <div className="col-auto">
            <button type="submit" className="btn btn-warning">
              Greet
            </button>
          </div>
        </form>
        {greetMsg && (
          <p className="text-center mt-3 alert alert-success">{greetMsg}</p>
        )}
      </section>

      <section className="text-center">
        <button
          className="btn btn-primary mb-3"
          onClick={increment}
        >
          Increment
        </button>
        <p className="fs-4">Count: <span className="text-warning">{count}</span></p>
      </section>
    </div>
  );
};
