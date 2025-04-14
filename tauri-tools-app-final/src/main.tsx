import ReactDOM from "react-dom/client";
import { App } from "./components/App";

import "./bootstrap.min.css";

ReactDOM
  .createRoot(document.getElementById("root") as HTMLElement)
  .render(<App />);
