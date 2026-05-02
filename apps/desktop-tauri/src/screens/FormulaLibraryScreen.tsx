import {
  Alert,
  Badge,
  Button,
  Code,
  Group,
  SimpleGrid,
  Stack,
  Table,
  Text,
  Title,
} from "@mantine/core";
import { RefreshCw } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { backend } from "../api";
import type { FormulaDetailsDto, FormulaPackDto, FormulaSummaryDto } from "../types";

export function FormulaLibraryScreen() {
  const [packs, setPacks] = useState<FormulaPackDto[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [formulas, setFormulas] = useState<FormulaSummaryDto[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [details, setDetails] = useState<FormulaDetailsDto | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectedFormula = useMemo(
    () => formulas.find((formula) => formula.id === selectedId) ?? formulas[0] ?? null,
    [formulas, selectedId],
  );

  useEffect(() => {
    void loadRegistry();
  }, []);

  useEffect(() => {
    if (!selectedFormula) {
      setDetails(null);
      return;
    }

    let active = true;
    backend
      .getFormula(selectedFormula.id)
      .then((formulaDetails) => {
        if (active) {
          setDetails(formulaDetails);
          setSelectedId(formulaDetails.id);
        }
      })
      .catch((loadError: unknown) => {
        if (active) {
          setError(loadError instanceof Error ? loadError.message : String(loadError));
        }
      });

    return () => {
      active = false;
    };
  }, [selectedFormula]);

  async function loadRegistry() {
    setLoading(true);
    setError(null);
    try {
      const loadedPacks = await backend.loadFormulaPacks();
      const [loadedCategories, loadedFormulas] = await Promise.all([
        backend.listFormulaCategories(),
        backend.listFormulas(),
      ]);
      setPacks(loadedPacks);
      setCategories(loadedCategories);
      setFormulas(loadedFormulas);
      setSelectedId(loadedFormulas[0]?.id ?? null);
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : String(loadError));
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Group justify="space-between" align="center">
          <Title order={2}>Formula Library</Title>
          <Button
            leftSection={<RefreshCw size={16} />}
            loading={loading}
            variant="light"
            onClick={() => void loadRegistry()}
          >
            Reload
          </Button>
        </Group>

        {error ? (
          <Alert color="red" variant="light">
            {error}
          </Alert>
        ) : null}

        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="md">
          <Stack gap="sm" className="sub-panel">
            <Title order={4}>Packs</Title>
            {packs.map((pack) => (
              <Group key={pack.pack_id} gap="xs">
                <Badge variant="light">{pack.pack_id}</Badge>
                <Text size="sm">
                  {pack.title} {pack.version} · {pack.formula_count}
                </Text>
              </Group>
            ))}
            <Title order={4}>Categories</Title>
            <Group gap="xs">
              {categories.map((category) => (
                <Badge key={category} color="gray" variant="outline">
                  {category}
                </Badge>
              ))}
            </Group>
          </Stack>

          <Stack gap="sm" className="sub-panel">
            <Title order={4}>Formulas</Title>
            {formulas.map((formula) => (
              <Button
                key={formula.id}
                justify="flex-start"
                variant={formula.id === selectedFormula?.id ? "filled" : "subtle"}
                onClick={() => setSelectedId(formula.id)}
              >
                {formula.title}
              </Button>
            ))}
          </Stack>

          <Stack gap="sm" className="sub-panel">
            <Title order={4}>{details?.title ?? "Details"}</Title>
            {details ? (
              <>
                <Group gap="xs">
                  <Badge>{details.category}</Badge>
                  {details.linked_circuit_template_id ? (
                    <Badge variant="outline">{details.linked_circuit_template_id}</Badge>
                  ) : null}
                </Group>
                <Text size="sm">{details.description}</Text>
                <Code block>
                  {details.equations.map((equation) => equation.expression).join("\n")}
                </Code>

                <Table>
                  <Table.Thead>
                    <Table.Tr>
                      <Table.Th>Name</Table.Th>
                      <Table.Th>Unit</Table.Th>
                      <Table.Th>Default</Table.Th>
                    </Table.Tr>
                  </Table.Thead>
                  <Table.Tbody>
                    {details.variables.map((variable) => (
                      <Table.Tr key={variable.name}>
                        <Table.Td>{variable.name}</Table.Td>
                        <Table.Td>{variable.unit || "-"}</Table.Td>
                        <Table.Td>{variable.default?.original ?? "-"}</Table.Td>
                      </Table.Tr>
                    ))}
                  </Table.Tbody>
                </Table>

                <Group gap="xs">
                  {details.outputs.map((output) => (
                    <Badge key={output.name} variant="light">
                      {output.name} {output.unit}
                    </Badge>
                  ))}
                </Group>
              </>
            ) : (
              <Text size="sm">-</Text>
            )}
          </Stack>
        </SimpleGrid>
      </div>
    </section>
  );
}
