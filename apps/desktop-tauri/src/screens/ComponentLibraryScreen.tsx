import { useEffect, useState } from "react";
import { Alert, Grid, LoadingOverlay, Stack, Text, Title } from "@mantine/core";
import { backend } from "../api";
import { AssignComponentPanel } from "../components/component-library/AssignComponentPanel";
import { ComponentDetailsPanel } from "../components/component-library/ComponentDetailsPanel";
import { ComponentSearchPanel } from "../components/component-library/ComponentSearchPanel";
import { ComponentTable } from "../components/component-library/ComponentTable";
import { useHotSasStore } from "../store";
import type { ComponentLibraryDto, ComponentSearchRequestDto, TypedComponentParametersDto } from "../types";

export function ComponentLibraryScreen() {
  const [library, setLibrary] = useState<ComponentLibraryDto | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [typedParams, setTypedParams] = useState<TypedComponentParametersDto | null>(null);

  const selectedLibraryComponentId = useHotSasStore((s) => s.selectedLibraryComponentId);
  const selectedLibraryComponent = useHotSasStore((s) => s.selectedLibraryComponent);
  const componentSearchResult = useHotSasStore((s) => s.componentSearchResult);
  const selectedSchematicComponent = useHotSasStore((s) => s.selectedComponent);

  const setSelectedLibraryComponentId = useHotSasStore((s) => s.setSelectedLibraryComponentId);
  const setSelectedLibraryComponent = useHotSasStore((s) => s.setSelectedLibraryComponent);
  const setComponentSearchResult = useHotSasStore((s) => s.setComponentSearchResult);
  const setProject = useHotSasStore((s) => s.setProject);

  useEffect(() => {
    loadLibrary();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function loadLibrary() {
    setLoading(true);
    setError(null);
    try {
      const lib = await backend.loadBuiltinComponentLibrary();
      setLibrary(lib);
      setComponentSearchResult({
        components: lib.components,
        total_count: lib.components.length,
        categories: lib.categories,
        tags: lib.tags,
      });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleSearch(request: ComponentSearchRequestDto) {
    setLoading(true);
    setError(null);
    try {
      const result = await backend.searchComponents(request);
      setComponentSearchResult(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleReset() {
    if (library) {
      setComponentSearchResult({
        components: library.components,
        total_count: library.components.length,
        categories: library.categories,
        tags: library.tags,
      });
    }
    setSelectedLibraryComponentId(null);
    setSelectedLibraryComponent(null);
  }

  async function handleSelectComponent(id: string) {
    setSelectedLibraryComponentId(id);
    setTypedParams(null);
    setLoading(true);
    try {
      const [details, typed] = await Promise.all([
        backend.getComponentDetails(id),
        backend.getTypedComponentParameters(id).catch(() => null),
      ]);
      setSelectedLibraryComponent(details);
      setTypedParams(typed);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleAssign() {
    if (!selectedSchematicComponent || !selectedLibraryComponent) return;
    setLoading(true);
    try {
      const result = await backend.assignComponentToSelectedInstance({
        instance_id: selectedSchematicComponent.instance_id,
        component_definition_id: selectedLibraryComponent.id,
        selected_symbol_id: selectedLibraryComponent.symbol_ids[0] ?? null,
        selected_footprint_id: selectedLibraryComponent.footprint_ids[0] ?? null,
        selected_simulation_model_id: selectedLibraryComponent.simulation_models[0]?.id ?? null,
      });
      setProject(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  const displayComponents = componentSearchResult?.components ?? library?.components ?? [];

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Stack gap="md" pos="relative">
          <LoadingOverlay visible={loading} />
          <Title order={2}>Component Library</Title>
          {library && (
            <Text size="sm" c="dimmed">
              {library.title} v{library.version} — {library.components.length} components
            </Text>
          )}
          {error && (
            <Alert color="red" onClose={() => setError(null)} withCloseButton>
              {error}
            </Alert>
          )}
          <ComponentSearchPanel
            categories={library?.categories ?? []}
            onSearch={handleSearch}
            onReset={handleReset}
          />
          <Grid>
            <Grid.Col span={7}>
              <ComponentTable
                components={displayComponents}
                selectedId={selectedLibraryComponentId}
                onSelect={handleSelectComponent}
              />
            </Grid.Col>
            <Grid.Col span={5}>
              <Stack gap="md">
                {selectedLibraryComponent && (
                  <ComponentDetailsPanel component={selectedLibraryComponent} typedParams={typedParams} />
                )}
                <AssignComponentPanel
                  selectedComponent={selectedSchematicComponent}
                  libraryComponent={selectedLibraryComponent}
                  onAssign={handleAssign}
                />
              </Stack>
            </Grid.Col>
          </Grid>
        </Stack>
      </div>
    </section>
  );
}
