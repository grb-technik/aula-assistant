import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";

document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
});

createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
