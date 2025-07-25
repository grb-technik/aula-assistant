import { AppBar } from "./components/layout/app-bar";
import { Toaster } from "@/components/ui/sonner";
import { WindowProvider } from "@/contexts/window";

import "./styles/global.css";

function App() {
    return (
        <>
            <WindowProvider>
                <AppBar />
            </WindowProvider>

            <Toaster />
        </>
    );
}

export default App;

