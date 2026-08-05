import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-[opacity,transform,background-color,border-color,color] duration-150 ease-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/50 disabled:pointer-events-none disabled:opacity-50 active:scale-[0.98]",
  {
    variants: {
      variant: {
        primary: "bg-accent text-accent-fg hover:opacity-90",
        secondary:
          "bg-bg-subtle text-fg border border-border hover:border-border-strong",
        outline:
          "bg-transparent text-fg border border-border hover:border-border-strong hover:bg-bg-subtle/50",
        ghost: "bg-transparent text-fg-muted hover:text-fg hover:bg-bg-subtle/60",
        pill: "rounded-full border border-border bg-bg-elevated text-fg-muted hover:text-fg",
        "pill-active": "rounded-full bg-accent text-accent-fg border border-transparent",
      },
      size: {
        sm: "min-h-8 px-3 text-xs rounded-xs",
        md: "min-h-10 px-4 text-[13px] rounded-sm",
        lg: "min-h-11 px-5 text-sm rounded-sm",
        icon: "size-10 rounded-sm",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  },
);

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof buttonVariants>;

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => (
    <button ref={ref} className={cn(buttonVariants({ variant, size }), className)} {...props} />
  ),
);
Button.displayName = "Button";
