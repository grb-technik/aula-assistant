import { Toaster } from "@/components/ui/sonner";
import { Views } from "./views";

import "./styles/global.css";

export default function App() {
    return (
        <>
            <main className="mt-0 min-h-dvh w-full">
                <Views />
            </main>

            <Toaster duration={1000} visibleToasts={1} position="bottom-center" richColors />
        </>
    );
}
