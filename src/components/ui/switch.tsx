import { cn } from "@/lib/utils";

type SwitchProps = {
  checked: boolean;
  onCheckedChange: (next: boolean) => void;
  "aria-label"?: string;
  className?: string;
};

export function Switch({ checked, onCheckedChange, className, ...rest }: SwitchProps) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      onClick={() => onCheckedChange(!checked)}
      className={cn(
        "relative h-6 w-11 shrink-0 rounded-full border border-border transition-colors duration-150",
        checked ? "bg-accent" : "bg-bg-subtle",
        className,
      )}
      {...rest}
    >
      <span
        className={cn(
          "absolute top-0.5 left-0.5 size-[18px] rounded-full transition-transform duration-150",
          checked ? "translate-x-5 bg-accent-fg" : "translate-x-0 bg-fg-muted",
        )}
      />
    </button>
  );
}
