import {
  Alert,
  Button,
  Code,
  Group,
  Select,
  Stack,
  Table,
  Text,
  TextInput,
  Title,
} from "@mantine/core";
import { Calculator, FileText, Play } from "lucide-react";
import { useState } from "react";
import { backend } from "../api";
import { logger } from "../utils/logger";
import type { DcdcCalculationResultDto, DcdcTemplateDto, SimulationResultDto } from "../types";

const TOPOLOGY_OPTIONS = [
  { value: "buck", label: "Buck (Step-Down)" },
  { value: "boost", label: "Boost (Step-Up)" },
  { value: "inverting_buck_boost", label: "Inverting Buck-Boost" },
  { value: "four_switch_buck_boost", label: "4-Switch Buck-Boost" },
];

export function DcdcCalculatorScreen() {
  const [topology, setTopology] = useState<string>("buck");
  const [vin, setVin] = useState("12");
  const [vout, setVout] = useState("5");
  const [iout, setIout] = useState("1");
  const [fs, setFs] = useState("100k");
  const [inductor, setInductor] = useState("47u");
  const [capacitor, setCapacitor] = useState("100u");
  const [ripplePercent, setRipplePercent] = useState<string>("30");
  const [efficiency, setEfficiency] = useState<string>("90");

  const [result, setResult] = useState<DcdcCalculationResultDto | null>(null);
  const [netlist, setNetlist] = useState<string | null>(null);
  const [transient, setTransient] = useState<SimulationResultDto | null>(null);
  const [templates, setTemplates] = useState<DcdcTemplateDto[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTemplates = async () => {
    try {
      const list = await backend.listDcdcTemplates();
      setTemplates(list);
    } catch (e) {
      logger.error(String(e));
    }
  };

  const calculate = async () => {
    setLoading(true);
    setError(null);
    setResult(null);
    setNetlist(null);
    setTransient(null);
    try {
      const res = await backend.calculateDcdc({
        topology,
        vin,
        vout,
        iout,
        switching_frequency: fs,
        inductor: inductor.trim() || null,
        output_capacitor: capacitor.trim() || null,
        target_inductor_ripple_percent: ripplePercent ? parseFloat(ripplePercent) : null,
        estimated_efficiency_percent: efficiency ? parseFloat(efficiency) : null,
      });
      setResult(res);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setLoading(false);
    }
  };

  const previewNetlist = async () => {
    setLoading(true);
    try {
      const nl = await backend.generateDcdcNetlistPreview({
        topology,
        vin,
        vout,
        iout,
        switching_frequency: fs,
      });
      setNetlist(nl);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setLoading(false);
    }
  };

  const previewTransient = async () => {
    setLoading(true);
    try {
      const sim = await backend.runDcdcMockTransientPreview({
        topology,
        vin,
        vout,
        iout,
        switching_frequency: fs,
        inductor: inductor.trim() || null,
        output_capacitor: capacitor.trim() || null,
        target_inductor_ripple_percent: ripplePercent ? parseFloat(ripplePercent) : null,
        estimated_efficiency_percent: efficiency ? parseFloat(efficiency) : null,
      });
      setTransient(sim);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Group justify="space-between" align="center">
          <Title order={2}>DC-DC Calculators</Title>
          <Button variant="light" onClick={() => void loadTemplates()}>
            Load Templates
          </Button>
        </Group>

        {error && (
          <Alert color="red" variant="light">
            {error}
          </Alert>
        )}

        <Stack gap="sm">
          <Select
            label="Topology"
            data={TOPOLOGY_OPTIONS}
            value={topology}
            onChange={(value) => {
              setTopology(value || "buck");
              setResult(null);
            }}
          />

          <Group gap="sm">
            <TextInput
              label="Vin (V)"
              value={vin}
              onChange={(e) => setVin(e.currentTarget.value)}
            />
            <TextInput
              label="Vout (V)"
              value={vout}
              onChange={(e) => setVout(e.currentTarget.value)}
            />
            <TextInput
              label="Iout (A)"
              value={iout}
              onChange={(e) => setIout(e.currentTarget.value)}
            />
            <TextInput label="fs (Hz)" value={fs} onChange={(e) => setFs(e.currentTarget.value)} />
          </Group>

          <Group gap="sm">
            <TextInput
              label="Inductor (H)"
              value={inductor}
              onChange={(e) => setInductor(e.currentTarget.value)}
              placeholder="optional"
            />
            <TextInput
              label="Cout (F)"
              value={capacitor}
              onChange={(e) => setCapacitor(e.currentTarget.value)}
              placeholder="optional"
            />
            <TextInput
              label="Ripple %"
              value={ripplePercent}
              onChange={(e) => setRipplePercent(e.currentTarget.value)}
            />
            <TextInput
              label="Efficiency %"
              value={efficiency}
              onChange={(e) => setEfficiency(e.currentTarget.value)}
            />
          </Group>

          <Group gap="sm">
            <Button
              leftSection={<Calculator size={16} />}
              loading={loading}
              onClick={() => void calculate()}
            >
              Calculate
            </Button>
            <Button
              leftSection={<FileText size={16} />}
              variant="light"
              onClick={() => void previewNetlist()}
            >
              Netlist Preview
            </Button>
            <Button
              leftSection={<Play size={16} />}
              variant="light"
              onClick={() => void previewTransient()}
            >
              Mock Transient
            </Button>
          </Group>
        </Stack>

        {result && (
          <Stack gap="sm" mt="md">
            <Alert color="blue" variant="light">
              <Text size="sm" fw={700}>
                Operating Mode: {result.operating_mode}
              </Text>
            </Alert>

            {result.warnings.length > 0 && (
              <Stack gap="xs">
                {result.warnings.map((w) => (
                  <Alert
                    key={w.code}
                    color={w.severity === "error" ? "red" : "yellow"}
                    variant="light"
                  >
                    <Text size="sm">
                      {w.code}: {w.message}
                    </Text>
                  </Alert>
                ))}
              </Stack>
            )}

            {result.values.length > 0 && (
              <Table>
                <Table.Thead>
                  <Table.Tr>
                    <Table.Th>Parameter</Table.Th>
                    <Table.Th>Value</Table.Th>
                    <Table.Th>Formula</Table.Th>
                  </Table.Tr>
                </Table.Thead>
                <Table.Tbody>
                  {result.values.map((v) => (
                    <Table.Tr key={v.id}>
                      <Table.Td>{v.label}</Table.Td>
                      <Table.Td>{v.value.display}</Table.Td>
                      <Table.Td>
                        <Text size="xs" c="dimmed">
                          {v.formula || "—"}
                        </Text>
                      </Table.Td>
                    </Table.Tr>
                  ))}
                </Table.Tbody>
              </Table>
            )}

            {result.assumptions.length > 0 && (
              <Alert color="blue" variant="light">
                <Text size="sm" fw={500}>
                  Assumptions
                </Text>
                {result.assumptions.map((a) => (
                  <Text key={a} size="sm">
                    • {a}
                  </Text>
                ))}
              </Alert>
            )}

            {result.limitations.length > 0 && (
              <Alert color="orange" variant="light">
                <Text size="sm" fw={500}>
                  Limitations
                </Text>
                {result.limitations.map((l) => (
                  <Text key={l} size="sm">
                    • {l}
                  </Text>
                ))}
              </Alert>
            )}
          </Stack>
        )}

        {netlist && (
          <Stack gap="xs" mt="md">
            <Title order={5}>Netlist Preview</Title>
            <Code block>{netlist}</Code>
          </Stack>
        )}

        {transient && (
          <Stack gap="xs" mt="md">
            <Title order={5}>Mock Transient Preview</Title>
            {transient.graph_series.map((gs) => (
              <Alert key={gs.name} color="teal" variant="light">
                <Text size="sm" fw={500}>
                  {gs.name}
                </Text>
                <Text size="xs">Points: {gs.points.length}</Text>
                <Text size="xs">
                  X: {gs.x_unit} | Y: {gs.y_unit}
                </Text>
              </Alert>
            ))}
            {transient.warnings.map((w) => (
              <Alert key={w} color="yellow" variant="light">
                {w}
              </Alert>
            ))}
          </Stack>
        )}

        {templates.length > 0 && (
          <Stack gap="xs" mt="md">
            <Title order={5}>Templates</Title>
            {templates.map((t) => (
              <Alert key={t.id} color="gray" variant="light">
                <Text size="sm" fw={500}>
                  {t.title}
                </Text>
                <Text size="xs">{t.description}</Text>
              </Alert>
            ))}
          </Stack>
        )}
      </div>
    </section>
  );
}
