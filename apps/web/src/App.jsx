import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import rustLogo from "./assets/rust.svg";
import viteLogo from "/vite.svg";
import "./App.css";

import init, { performance_test } from "core";

function App() {
  const [data, setData] = useState("Run performance test");

  useEffect(() => {
    const fetchUserData = async () => {
      await init();
    };
    fetchUserData();
  }, []);

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="https://rust-lang.org" target="_blank">
          <img src={rustLogo} className="logo rust" alt="Rust logo" />
        </a>
      </div>
      <h1>YDX</h1>
      <div className="card">
        <button
          disabled={data == "Loading..."}
          onClick={() => {
            new Promise((resolve, reject) => {
              setTimeout(() => {
                try {
                  resolve(performance_test());
                } catch (error) {
                  reject(error);
                }
              }, 0);
            })
              .then((value) => {
                setData(`The test was performed: ${value}ms`);
              })
              .catch((error) => {
                setData(`Error: ${error}`);
              });
            setData("Loading...");
          }}
        >
          {data}
        </button>
      </div>
      <p className="read-the-docs">
        Click to test the performance of the Rust WebAssembly library.
      </p>
    </>
  );
}

export default App;
