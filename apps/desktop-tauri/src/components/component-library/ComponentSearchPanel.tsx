import { Button, Checkbox, Group, Select, Stack, TextInput } from "@mantine/core";
import { useState } from "react";
import type { ComponentSearchRequestDto } from "../../types";

type Props = {
  categories: string[];
  onSearch: (request: ComponentSearchRequestDto) => void;
  onReset: () => void;
};

export function ComponentSearchPanel({ categories, onSearch, onReset }: Props) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState<string | null>(null);
  const [hasSymbol, setHasSymbol] = useState(false);
  const [hasFootprint, setHasFootprint] = useState(false);
  const [hasSimulationModel, setHasSimulationModel] = useState(false);

  function handleSearch() {
    onSearch({
      search: search.trim() || null,
      category,
      tags: [],
      manufacturer: null,
      has_symbol: hasSymbol || null,
      has_footprint: hasFootprint || null,
      has_simulation_model: hasSimulationModel || null,
    });
  }

  function handleReset() {
    setSearch("");
    setCategory(null);
    setHasSymbol(false);
    setHasFootprint(false);
    setHasSimulationModel(false);
    onReset();
  }

  return (
    <Stack gap="xs">
      <Group grow>
        <TextInput
          placeholder="Search components..."
          value={search}
          onChange={(e) => setSearch(e.currentTarget.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSearch()}
        />
        <Select
          placeholder="Category"
          data={categories}
          value={category}
          onChange={setCategory}
          clearable
        />
      </Group>
      <Group>
        <Checkbox
          label="Has symbol"
          checked={hasSymbol}
          onChange={(e) => setHasSymbol(e.currentTarget.checked)}
        />
        <Checkbox
          label="Has footprint"
          checked={hasFootprint}
          onChange={(e) => setHasFootprint(e.currentTarget.checked)}
        />
        <Checkbox
          label="Has model"
          checked={hasSimulationModel}
          onChange={(e) => setHasSimulationModel(e.currentTarget.checked)}
        />
      </Group>
      <Group>
        <Button onClick={handleSearch}>Search</Button>
        <Button variant="light" onClick={handleReset}>
          Reset
        </Button>
      </Group>
    </Stack>
  );
}
