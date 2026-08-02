import * as React from "react";
import * as SwitchPrimitives from "@radix-ui/react-switch";
import { cn } from "@/lib/utils";

export const Switch = React.forwardRef<
  React.ComponentRef<typeof SwitchPrimitives.Root>,
  React.ComponentPropsWithoutRef<typeof SwitchPrimitives.Root>
>(({ className, ...props }, ref) => (
  <SwitchPrimitives.Root
    className={cn(
      "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border border-border transition-colors",
      "data-[state=checked]:bg-accent data-[state=unchecked]:bg-bg-subtle",
      "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
      "disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
    ref={ref}
  >
    <SwitchPrimitives.Thumb
      className={cn(
        "pointer-events-none block h-5 w-5 rounded-full shadow-sm transition-transform",
        "data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0.5",
        "data-[state=checked]:bg-accent-fg data-[state=unchecked]:bg-fg-muted",
      )}
    />
  </SwitchPrimitives.Root>
));
Switch.displayName = "Switch";
