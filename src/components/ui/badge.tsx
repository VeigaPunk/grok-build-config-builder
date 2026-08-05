import { cn } from "@/lib/utils";

export function Badge({
  children,
  variant = "default",
  className,
}: {
  children: React.ReactNode;
  variant?: "default" | "success" | "subtle";
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium",
        variant === "default" && "border-border bg-bg-subtle text-fg-muted",
        variant === "success" &&
          "border-success/30 bg-success-bg text-success",
        variant === "subtle" && "border-transparent bg-bg-subtle text-fg-subtle",
        className,
      )}
    >
      {children}
    </span>
  );
}
