import { Button, Card, Group, Select, Stack, Text } from "@mantine/core";
import { Loader2, Play, Trash2 } from "lucide-react";
import type { FilterAnalysisMethod } from "../../types";

interface Props {
  method: FilterAnalysisMethod;
  onMethodChange: (method: FilterAnalysisMethod) => void;
  onRun: () => void;
  onClear: () => void;
  loading: boolean;
  disabled: boolean;
}

export function FilterRunControls({
  method,
  onMethodChange,
  onRun,
  onClear,
  loading,
  disabled,
}: Props) {
  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Stack gap="sm">
        <Text fw={600} size="sm">
          Analysis
        </Text>
        <Group gap="sm" align="flex-end">
          <Select
            label="Method"
            data={[
              { value: "auto", label: "Auto" },
              { value: "template_analytic", label: "Template Analytic" },
              { value: "mock", label: "Mock" },
              { value: "ngspice", label: "ngspice" },
            ]}
            value={method}
            onChange={(value) => {
              if (value) onMethodChange(value as FilterAnalysisMethod);
            }}
            style={{ flex: 1 }}
          />
          <Button
            leftSection={loading ? <Loader2 size={16} className="spin" /> : <Play size={16} />}
            onClick={onRun}
            disabled={disabled || loading}
          >
            Run Analysis
          </Button>
          <Button
            leftSection={<Trash2 size={16} />}
            variant="light"
            color="red"
            onClick={onClear}
            disabled={loading}
          >
            Clear
          </Button>
        </Group>
      </Stack>
    </Card>
  );
}
