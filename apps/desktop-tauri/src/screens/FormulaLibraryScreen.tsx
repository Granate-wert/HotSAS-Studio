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
  TextInput,
  Title,
} from "@mantine/core";
import { RefreshCw } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { backend } from "../api";
import { logger } from "../utils/logger";
import type {
  FormulaDetailsDto,
  FormulaEvaluationResultDto,
  FormulaPackDto,
  FormulaSummaryDto,
} from "../types";

function isNonEmptyArray<T>(value: unknown): value is T[] {
  return Array.isArray(value) && value.length > 0;
}

export function FormulaLibraryScreen() {
  const [packs, setPacks] = useState<FormulaPackDto[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [formulas, setFormulas] = useState<FormulaSummaryDto[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [details, setDetails] = useState<FormulaDetailsDto | null>(null);
  const [variableInputs, setVariableInputs] = useState<Record<string, string>>({});
  const [calculationResult, setCalculationResult] = useState<FormulaEvaluationResultDto | null>(
    null,
  );
  const [loading, setLoading] = useState(false);
  const [calculating, setCalculating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

  const filteredFormulas = useMemo(() => {
    let result = formulas;
    if (selectedCategory) {
      result = result.filter((f) => f.category === selectedCategory);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      result = result.filter(
        (f) =>
          f.title.toLowerCase().includes(q) ||
          f.id.toLowerCase().includes(q) ||
          f.description.toLowerCase().includes(q),
      );
    }
    return result;
  }, [formulas, selectedCategory, searchQuery]);

  const selectedFormula = useMemo(
    () =>
      filteredFormulas.find((formula) => formula.id === selectedId) ?? filteredFormulas[0] ?? null,
    [filteredFormulas, selectedId],
  );

  useEffect(() => {
    logger.info("FormulaLibraryScreen mounted");
    void loadRegistry();
  }, []);

  useEffect(() => {
    if (!selectedFormula) {
      setDetails(null);
      return;
    }

    logger.info(`Selected formula: ${selectedFormula.id}`);
    let active = true;
    backend
      .getFormula(selectedFormula.id)
      .then((formulaDetails) => {
        if (!active) return;
        logger.info(`Loaded formula details: ${formulaDetails.id}`);
        setDetails(formulaDetails);
        setSelectedId(formulaDetails.id);
        setCalculationResult(null);
        setVariableInputs(
          Object.fromEntries(
            (formulaDetails.variables ?? []).map((variable) => [
              variable.name,
              variable.default?.original ?? "",
            ]),
          ),
        );
      })
      .catch((loadError: unknown) => {
        if (!active) return;
        const msg = loadError instanceof Error ? loadError.message : String(loadError);
        logger.error(`getFormula failed: ${msg}`);
        setError(msg);
      });

    return () => {
      active = false;
    };
  }, [selectedFormula]);

  async function loadRegistry() {
    logger.info("loadRegistry started");
    setLoading(true);
    setError(null);
    try {
      const loadedPacks = await backend.loadFormulaPacks();
      logger.info(`loadRegistry: loaded ${loadedPacks.length} pack(s)`);
      const [loadedCategories, loadedFormulas] = await Promise.all([
        backend.listFormulaCategories(),
        backend.listFormulas(),
      ]);
      logger.info(
        `loadRegistry: ${loadedFormulas.length} formula(s), ${loadedCategories.length} categorie(s)`,
      );
      setPacks(loadedPacks);
      setCategories(loadedCategories);
      setFormulas(loadedFormulas);
      setSelectedId(loadedFormulas[0]?.id ?? null);
    } catch (loadError) {
      const msg = loadError instanceof Error ? loadError.message : String(loadError);
      logger.error(`loadRegistry failed: ${msg}`);
      setError(msg);
    } finally {
      setLoading(false);
    }
  }

  async function calculateSelectedFormula() {
    if (!details) {
      return;
    }

    logger.info(`Calculate started for formula: ${details.id}`);
    for (const variable of details.variables ?? []) {
      logger.info(`  Input ${variable.name} = ${variableInputs[variable.name] ?? ""}`);
    }

    setCalculating(true);
    setError(null);
    try {
      const result = await backend.calculateFormula({
        formula_id: details.id,
        variables: (details.variables ?? []).map((variable) => ({
          name: variable.name,
          value: variableInputs[variable.name] ?? "",
          unit: variable.unit || null,
        })),
      });
      logger.info(
        `Calculate succeeded. Outputs: ${result.outputs.map((o) => `${o.name}=${o.value.display}`).join(", ")}`,
      );
      setCalculationResult(result);
    } catch (calculationError) {
      const msg =
        calculationError instanceof Error ? calculationError.message : String(calculationError);
      logger.error(`Calculate failed: ${msg}`);
      setCalculationResult(null);
      setError(msg);
    } finally {
      setCalculating(false);
    }
  }

  const safeOutputs = isNonEmptyArray(calculationResult?.outputs) ? calculationResult.outputs : [];
  const safeWarnings = Array.isArray(calculationResult?.warnings) ? calculationResult.warnings : [];
  const safeVariables = Array.isArray(details?.variables) ? details.variables : [];
  const safeEquations = Array.isArray(details?.equations) ? details.equations : [];
  const safeDetailsOutputs = Array.isArray(details?.outputs) ? details.outputs : [];

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
                  {pack.title} {pack.version} - {pack.formula_count}
                </Text>
              </Group>
            ))}
            <Title order={4}>Categories</Title>
            <Group gap="xs">
              <Badge
                color={selectedCategory === null ? "blue" : "gray"}
                variant={selectedCategory === null ? "filled" : "outline"}
                style={{ cursor: "pointer" }}
                onClick={() => setSelectedCategory(null)}
              >
                All
              </Badge>
              {categories.map((category) => (
                <Badge
                  key={category}
                  color={selectedCategory === category ? "blue" : "gray"}
                  variant={selectedCategory === category ? "filled" : "outline"}
                  style={{ cursor: "pointer" }}
                  onClick={() => setSelectedCategory(category)}
                >
                  {category}
                </Badge>
              ))}
            </Group>
          </Stack>

          <Stack gap="sm" className="sub-panel">
            <Title order={4}>Formulas</Title>
            <TextInput
              placeholder="Search formulas..."
              value={searchQuery}
              onChange={(event) => setSearchQuery(event.currentTarget.value)}
            />
            {filteredFormulas.map((formula) => (
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
                {safeEquations.length > 0 ? (
                  <Code block>
                    {safeEquations.map((equation) => equation.expression).join("\n")}
                  </Code>
                ) : null}

                {safeVariables.length > 0 ? (
                  <Table>
                    <Table.Thead>
                      <Table.Tr>
                        <Table.Th>Name</Table.Th>
                        <Table.Th>Unit</Table.Th>
                        <Table.Th>Default</Table.Th>
                      </Table.Tr>
                    </Table.Thead>
                    <Table.Tbody>
                      {safeVariables.map((variable) => (
                        <Table.Tr key={variable.name}>
                          <Table.Td>{variable.name}</Table.Td>
                          <Table.Td>{variable.unit || "-"}</Table.Td>
                          <Table.Td>
                            <TextInput
                              aria-label={`${variable.name} value`}
                              value={variableInputs[variable.name] ?? ""}
                              placeholder={variable.default?.original ?? "value"}
                              onChange={(event) => {
                                const value = event.currentTarget?.value ?? "";
                                logger.debug(`Input ${variable.name} changed to: ${value}`);
                                setVariableInputs((current) => ({
                                  ...current,
                                  [variable.name]: value,
                                }));
                              }}
                            />
                          </Table.Td>
                        </Table.Tr>
                      ))}
                    </Table.Tbody>
                  </Table>
                ) : (
                  <Text size="sm" c="dimmed">
                    No variables defined.
                  </Text>
                )}

                <Button loading={calculating} onClick={() => void calculateSelectedFormula()}>
                  Calculate
                </Button>

                {details.assumptions.length > 0 ? (
                  <Alert color="blue" variant="light">
                    <Text size="sm" fw={500}>
                      Assumptions
                    </Text>
                    {details.assumptions.map((a) => (
                      <Text key={a} size="sm">
                        • {a}
                      </Text>
                    ))}
                  </Alert>
                ) : null}

                {details.limitations.length > 0 ? (
                  <Alert color="orange" variant="light">
                    <Text size="sm" fw={500}>
                      Limitations
                    </Text>
                    {details.limitations.map((l) => (
                      <Text key={l} size="sm">
                        • {l}
                      </Text>
                    ))}
                  </Alert>
                ) : null}

                {details.examples.length > 0 ? (
                  <Stack gap="xs">
                    <Text size="sm" fw={500}>
                      Examples
                    </Text>
                    {details.examples.map((example, index) => (
                      <Button
                        key={index}
                        variant="light"
                        size="xs"
                        onClick={() => {
                          const inputs: Record<string, string> = {};
                          for (const input of example.inputs) {
                            inputs[input.name] = input.value;
                          }
                          setVariableInputs(inputs);
                          setCalculationResult(null);
                        }}
                      >
                        {example.title}
                      </Button>
                    ))}
                  </Stack>
                ) : null}

                {safeDetailsOutputs.length > 0 ? (
                  <Group gap="xs">
                    {safeDetailsOutputs.map((output) => (
                      <Badge key={output.name} variant="light">
                        {output.name} {output.unit}
                      </Badge>
                    ))}
                  </Group>
                ) : null}

                {calculationResult ? (
                  <Stack gap="xs">
                    <Title order={5}>Result</Title>
                    {safeOutputs.length > 0 ? (
                      safeOutputs.map((output) => (
                        <Text key={output.name} size="sm">
                          {output.name}: {output.value?.display ?? "—"}
                        </Text>
                      ))
                    ) : (
                      <Text size="sm" c="dimmed">
                        No outputs returned.
                      </Text>
                    )}
                    {safeWarnings.map((warning) => (
                      <Alert key={warning} color="yellow" variant="light">
                        {warning}
                      </Alert>
                    ))}
                  </Stack>
                ) : null}
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
