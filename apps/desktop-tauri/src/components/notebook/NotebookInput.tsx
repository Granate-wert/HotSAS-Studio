import { Button, Group, Textarea } from "@mantine/core";

export type NotebookInputProps = {
  input: string;
  onChange: (value: string) => void;
  onEvaluate: () => void;
  onClear: () => void;
  loading: boolean;
};

const PLACEHOLDER = `R = 10k
C = 100n
rc_low_pass_cutoff(R=10k, C=100n)
ohms_law(I=2m, R=10k)
voltage_divider(Vin=5, R1=10k, R2=10k)
nearestE(15.93k, E96, Ohm)`;

export function NotebookInput({
  input,
  onChange,
  onEvaluate,
  onClear,
  loading,
}: NotebookInputProps) {
  return (
    <>
      <Textarea
        label="Input"
        placeholder={PLACEHOLDER}
        value={input}
        onChange={(e) => onChange(e.currentTarget.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            onEvaluate();
          }
        }}
        minRows={3}
      />
      <Group>
        <Button onClick={onEvaluate} loading={loading} size="compact-sm">
          Evaluate
        </Button>
        <Button onClick={onClear} variant="light" size="compact-sm" color="red">
          Clear
        </Button>
      </Group>
    </>
  );
}
