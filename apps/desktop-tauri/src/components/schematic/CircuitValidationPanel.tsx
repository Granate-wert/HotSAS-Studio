import { Button, Stack, Text } from "@mantine/core";
import { useState } from "react";
import { backend } from "../../api";
import type { CircuitValidationReportDto } from "../../types";

type CircuitValidationPanelProps = {
  report: CircuitValidationReportDto | null;
  onValidate: (report: CircuitValidationReportDto) => void;
};

export function CircuitValidationPanel({ report, onValidate }: CircuitValidationPanelProps) {
  const [busy, setBusy] = useState(false);

  const handleValidate = async () => {
    setBusy(true);
    try {
      const result = await backend.validateCurrentCircuit();
      onValidate(result);
    } catch (error) {
      onValidate({
        valid: false,
        warnings: [],
        errors: [
          {
            code: "validation_failed",
            message: String(error),
            component_id: null,
            net_id: null,
          },
        ],
      });
    } finally {
      setBusy(false);
    }
  };

  return (
    <Stack gap="sm">
      <Button size="compact-sm" onClick={handleValidate} disabled={busy} loading={busy}>
        Validate Circuit
      </Button>
      {report && (
        <>
          <Text size="sm" c={report.valid ? "green" : "red"}>
            {report.valid ? "Valid" : "Invalid"}
          </Text>
          {report.errors.length > 0 && (
            <Stack gap={4}>
              <Text fw={700} size="xs">
                Errors:
              </Text>
              {report.errors.map((err, i) => (
                <Text size="xs" c="red" key={i}>
                  [{err.code}] {err.message}
                  {err.component_id && ` (component: ${err.component_id})`}
                  {err.net_id && ` (net: ${err.net_id})`}
                </Text>
              ))}
            </Stack>
          )}
          {report.warnings.length > 0 && (
            <Stack gap={4}>
              <Text fw={700} size="xs">
                Warnings:
              </Text>
              {report.warnings.map((warn, i) => (
                <Text size="xs" c="yellow" key={i}>
                  [{warn.code}] {warn.message}
                  {warn.component_id && ` (component: ${warn.component_id})`}
                  {warn.net_id && ` (net: ${warn.net_id})`}
                </Text>
              ))}
            </Stack>
          )}
        </>
      )}
    </Stack>
  );
}
