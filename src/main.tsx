import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// Basic CSS reset
const style = document.createElement("style");
style.textContent = "body { margin: 0; padding: 0; }";
document.head.appendChild(style);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
