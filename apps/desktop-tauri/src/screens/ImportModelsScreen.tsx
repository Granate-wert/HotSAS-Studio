import { useState } from "react";
import {
  Button,
  Paper,
  Stack,
  Tabs,
  Text,
  Textarea,
  Table,
  ScrollArea,
  Badge,
  Group,
  Alert,
} from "@mantine/core";
import { AlertCircle, CheckCircle, Upload } from "lucide-react";
import { backend } from "../api";
import { useHotSasStore } from "../store";
import type {
  ImportedModelDetailsDto,
  ImportedModelSummaryDto,
  SpiceImportReportDto,
  TouchstoneImportReportDto,
} from "../types";

export function ImportModelsScreen() {
  const {
    spiceImportReport,
    touchstoneImportReport,
    importedModels,
    selectedImportedModel,
    setSpiceImportReport,
    setTouchstoneImportReport,
    setImportedModels,
    setSelectedImportedModel,
    busy,
    setBusy,
    setError,
  } = useHotSasStore();

  const [spiceContent, setSpiceContent] = useState("");
  const [touchstoneContent, setTouchstoneContent] = useState("");
  const [activeTab, setActiveTab] = useState<string | null>("spice");
  const [localBusy, setLocalBusy] = useState(false);

  const run = async <T,>(operation: () => Promise<T>, onResult: (result: T) => void) => {
    setLocalBusy(true);
    setBusy(true);
    setError(null);
    try {
      const result = await operation();
      onResult(result);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setLocalBusy(false);
      setBusy(false);
    }
  };

  const handleImportSpice = () => {
    run(
      () =>
        backend.importSpiceModel({
          source_name: null,
          content: spiceContent,
        }),
      (report: SpiceImportReportDto) => {
        setSpiceImportReport(report);
        refreshList();
      },
    );
  };

  const handleImportTouchstone = () => {
    run(
      () =>
        backend.importTouchstoneModel({
          source_name: null,
          content: touchstoneContent,
        }),
      (report: TouchstoneImportReportDto) => {
        setTouchstoneImportReport(report);
        refreshList();
      },
    );
  };

  const refreshList = () => {
    run(backend.listImportedModels, (models: ImportedModelSummaryDto[]) => {
      setImportedModels(models);
    });
  };

  const handleSelectModel = (id: string) => {
    run(
      () => backend.getImportedModel(id),
      (model: ImportedModelDetailsDto) => {
        setSelectedImportedModel(model);
      },
    );
  };

  const isLoading = busy || localBusy;

  return (
    <Stack gap="md" p="md">
      <Text size="xl" fw={700}>
        Import Models
      </Text>
      <Text size="sm" c="dimmed">
        Paste SPICE model definitions (.model / .subckt) or Touchstone S-parameter data (.s1p /
        .s2p) to import them into the project.
      </Text>

      <Tabs value={activeTab} onChange={setActiveTab}>
        <Tabs.List>
          <Tabs.Tab value="spice">SPICE Model / Subcircuit</Tabs.Tab>
          <Tabs.Tab value="touchstone">Touchstone S-Parameters</Tabs.Tab>
          <Tabs.Tab value="library">Imported Library</Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="spice" pt="md">
          <Stack gap="md">
            <Textarea
              label="SPICE content"
              description="Paste .model or .subckt definitions"
              placeholder=".model 1N4148 D(IS=2.52n RS=0.568 N=1.752 CJO=4p M=0.4 TT=20n)"
              minRows={8}
              maxRows={16}
              value={spiceContent}
              onChange={(e) => setSpiceContent(e.currentTarget.value)}
            />
            <Group>
              <Button
                leftSection={<Upload size={16} />}
                onClick={handleImportSpice}
                loading={isLoading}
                disabled={!spiceContent.trim()}
              >
                Import SPICE
              </Button>
            </Group>
            {spiceImportReport && <SpiceReportView report={spiceImportReport} />}
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="touchstone" pt="md">
          <Stack gap="md">
            <Textarea
              label="Touchstone content"
              description="Paste .s1p or .s2p data"
              placeholder="# GHz S MA R 50\n1.0 0.95 -0.32"
              minRows={8}
              maxRows={16}
              value={touchstoneContent}
              onChange={(e) => setTouchstoneContent(e.currentTarget.value)}
            />
            <Group>
              <Button
                leftSection={<Upload size={16} />}
                onClick={handleImportTouchstone}
                loading={isLoading}
                disabled={!touchstoneContent.trim()}
              >
                Import Touchstone
              </Button>
            </Group>
            {touchstoneImportReport && <TouchstoneReportView report={touchstoneImportReport} />}
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="library" pt="md">
          <Stack gap="md">
            <Group>
              <Button variant="light" onClick={refreshList} loading={isLoading}>
                Refresh List
              </Button>
            </Group>
            {importedModels.length === 0 ? (
              <Text c="dimmed" size="sm">
                No imported models yet. Use the SPICE or Touchstone tabs to import.
              </Text>
            ) : (
              <ScrollArea>
                <Table striped highlightOnHover>
                  <Table.Thead>
                    <Table.Tr>
                      <Table.Th>Name</Table.Th>
                      <Table.Th>Kind</Table.Th>
                      <Table.Th>Source</Table.Th>
                    </Table.Tr>
                  </Table.Thead>
                  <Table.Tbody>
                    {importedModels.map((m) => (
                      <Table.Tr
                        key={m.id}
                        data-testid={`model-row-${m.id}`}
                        style={{ cursor: "pointer" }}
                        onClick={() => handleSelectModel(m.id)}
                      >
                        <Table.Td>{m.name}</Table.Td>
                        <Table.Td>
                          <Badge size="sm">{m.kind}</Badge>
                        </Table.Td>
                        <Table.Td>{m.source_format}</Table.Td>
                      </Table.Tr>
                    ))}
                  </Table.Tbody>
                </Table>
              </ScrollArea>
            )}
            {selectedImportedModel && <ModelDetailsView model={selectedImportedModel} />}
          </Stack>
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}

function SpiceReportView({ report }: { report: SpiceImportReportDto }) {
  return (
    <Paper p="md" withBorder>
      <Group gap="xs" mb="sm">
        <Text fw={600}>Import Result</Text>
        <Badge
          color={report.errors.length > 0 ? "red" : report.warnings.length > 0 ? "yellow" : "green"}
        >
          {report.status}
        </Badge>
      </Group>

      {report.models.length > 0 && (
        <Stack gap="xs" mb="sm">
          <Text fw={500} size="sm">
            Models ({report.models.length})
          </Text>
          {report.models.map((m) => (
            <Paper key={m.id} p="xs" withBorder>
              <Group gap="xs">
                <Badge size="sm">{m.kind}</Badge>
                <Text size="sm" fw={500}>
                  {m.name}
                </Text>
              </Group>
              {m.parameters.length > 0 && (
                <Text size="xs" c="dimmed" mt={4}>
                  {m.parameters
                    .slice(0, 5)
                    .map((p) => `${p.name}=${p.value}`)
                    .join(", ")}
                  {m.parameters.length > 5 && " ..."}
                </Text>
              )}
            </Paper>
          ))}
        </Stack>
      )}

      {report.subcircuits.length > 0 && (
        <Stack gap="xs" mb="sm">
          <Text fw={500} size="sm">
            Subcircuits ({report.subcircuits.length})
          </Text>
          {report.subcircuits.map((s) => (
            <Paper key={s.id} p="xs" withBorder>
              <Group gap="xs">
                <Badge size="sm">{s.detected_kind}</Badge>
                <Text size="sm" fw={500}>
                  {s.name}
                </Text>
              </Group>
              <Text size="xs" c="dimmed" mt={4}>
                Pins: {s.pins.join(", ")}
              </Text>
            </Paper>
          ))}
        </Stack>
      )}

      {report.warnings.length > 0 && (
        <Alert icon={<AlertCircle size={16} />} color="yellow" variant="light" mb="xs">
          <Text size="xs">{report.warnings.join("; ")}</Text>
        </Alert>
      )}
      {report.errors.length > 0 && (
        <Alert icon={<AlertCircle size={16} />} color="red" variant="light">
          <Text size="xs">{report.errors.join("; ")}</Text>
        </Alert>
      )}
      {report.models.length === 0 &&
        report.subcircuits.length === 0 &&
        report.errors.length === 0 && (
          <Text size="sm" c="dimmed">
            No models or subcircuits detected in the provided content.
          </Text>
        )}
    </Paper>
  );
}

function TouchstoneReportView({ report }: { report: TouchstoneImportReportDto }) {
  return (
    <Paper p="md" withBorder>
      <Group gap="xs" mb="sm">
        <Text fw={600}>Import Result</Text>
        <Badge
          color={report.errors.length > 0 ? "red" : report.warnings.length > 0 ? "yellow" : "green"}
        >
          {report.status}
        </Badge>
      </Group>

      {report.summary && (
        <Stack gap="xs" mb="sm">
          <Group gap="xs">
            <CheckCircle size={16} color="green" />
            <Text size="sm" fw={500}>
              {report.summary.name}
            </Text>
          </Group>
          <Text size="xs" c="dimmed">
            {report.summary.port_count}-port | {report.summary.point_count} points |{" "}
            {report.summary.parameter_format} | {report.summary.reference_impedance_ohm} Ω
          </Text>
          {report.summary.start_frequency_hz !== null && (
            <Text size="xs" c="dimmed">
              Frequency range: {report.summary.start_frequency_hz} Hz →{" "}
              {report.summary.stop_frequency_hz} Hz
            </Text>
          )}
        </Stack>
      )}

      {report.warnings.length > 0 && (
        <Alert icon={<AlertCircle size={16} />} color="yellow" variant="light" mb="xs">
          <Text size="xs">{report.warnings.join("; ")}</Text>
        </Alert>
      )}
      {report.errors.length > 0 && (
        <Alert icon={<AlertCircle size={16} />} color="red" variant="light">
          <Text size="xs">{report.errors.join("; ")}</Text>
        </Alert>
      )}
      {!report.summary && report.errors.length === 0 && (
        <Text size="sm" c="dimmed">
          No network data detected in the provided content.
        </Text>
      )}
    </Paper>
  );
}

function ModelDetailsView({ model }: { model: ImportedModelDetailsDto }) {
  return (
    <Paper p="md" withBorder mt="md">
      <Text fw={600} mb="sm">
        Model Details
      </Text>
      <Group gap="xs" mb="sm">
        <Badge>{model.kind}</Badge>
        <Text size="sm" fw={500}>
          {model.name}
        </Text>
      </Group>
      {model.spice_model && (
        <Stack gap="xs" data-testid="spice-model-details">
          <Text size="sm" fw={500}>
            SPICE Model
          </Text>
          <Text size="xs" c="dimmed">
            Kind: {model.spice_model.kind}
          </Text>
          <Text size="xs" c="dimmed">
            Parameters: {model.spice_model.parameters.length}
          </Text>
        </Stack>
      )}
      {model.spice_subcircuit && (
        <Stack gap="xs">
          <Text size="sm" fw={500}>
            Subcircuit
          </Text>
          <Text size="xs" c="dimmed">
            Pins: {model.spice_subcircuit.pins.join(", ")}
          </Text>
        </Stack>
      )}
      {model.touchstone_summary && (
        <Stack gap="xs">
          <Text size="sm" fw={500}>
            Touchstone Network
          </Text>
          <Text size="xs" c="dimmed">
            {model.touchstone_summary.port_count}-port | {model.touchstone_summary.point_count}{" "}
            points
          </Text>
        </Stack>
      )}
    </Paper>
  );
}
