import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Toaster } from "sonner";
import { ConfigBuilder } from "@/components/ConfigBuilder";
import "./styles.css";

const root = document.getElementById("root");
if (!root) throw new Error("Missing #root");

createRoot(root).render(
  <StrictMode>
    <ConfigBuilder />
    <Toaster
      theme="dark"
      position="bottom-right"
      toastOptions={{
        className: "border border-border bg-bg-elevated text-fg",
      }}
    />
  </StrictMode>,
);
