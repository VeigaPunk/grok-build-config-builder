import { createFileRoute } from "@tanstack/react-router";
import { ConfigBuilder } from "@/components/ConfigBuilder";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  return <ConfigBuilder />;
}
