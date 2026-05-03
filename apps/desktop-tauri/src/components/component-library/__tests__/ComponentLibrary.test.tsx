import { screen, waitFor } from "@testing-library/react";
import { render } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ComponentLibraryScreen } from "../../../screens/ComponentLibraryScreen";
import * as api from "../../../api";

const mockLibrary = {
  id: "hotsas_builtin",
  title: "HotSAS Built-in Library",
  version: "1.5.0",
  components: [
    {
      id: "generic_resistor",
      name: "Generic Resistor",
      category: "resistor",
      manufacturer: null,
      part_number: null,
      description: "Generic resistor",
      tags: ["passive", "resistor"],
      has_symbol: true,
      has_footprint: true,
      has_simulation_model: false,
    },
    {
      id: "generic_capacitor",
      name: "Generic Capacitor",
      category: "capacitor",
      manufacturer: null,
      part_number: null,
      description: "Generic capacitor",
      tags: ["passive", "capacitor"],
      has_symbol: true,
      has_footprint: true,
      has_simulation_model: false,
    },
  ],
  categories: ["resistor", "capacitor"],
  tags: ["passive", "resistor", "capacitor"],
};

const mockDetails = {
  id: "generic_resistor",
  name: "Generic Resistor",
  category: "resistor",
  manufacturer: null,
  part_number: null,
  description: "Generic resistor",
  parameters: [{ name: "resistance", value: "10k", unit: "Ohm" }],
  ratings: [{ name: "power", value: "0.25", unit: "W" }],
  symbol_ids: ["resistor"],
  footprint_ids: ["axial_resistor_placeholder"],
  simulation_models: [],
  datasheets: [],
  tags: ["passive", "resistor"],
  metadata: [],
  symbol_preview: {
    id: "resistor",
    title: "Resistor",
    component_kind: "resistor",
    width: 80,
    height: 30,
    pins: [
      { id: "1", name: "1", number: "1", electrical_type: "passive", x: -40, y: 0, side: "left" },
      { id: "2", name: "2", number: "2", electrical_type: "passive", x: 40, y: 0, side: "right" },
    ],
  },
  footprint_previews: [
    {
      id: "axial_resistor_placeholder",
      name: "Axial Resistor",
      package_name: "AXIAL-0.4",
      pad_count: 1,
      metadata: [],
    },
  ],
};

describe("ComponentLibraryScreen", () => {
  it("renders and loads built-in library on mount", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue(mockLibrary);
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: mockLibrary.components,
      total_count: mockLibrary.components.length,
      categories: mockLibrary.categories,
      tags: mockLibrary.tags,
    });

    render(<ComponentLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("Generic Resistor")).toBeInTheDocument();
    });
    expect(api.backend.loadBuiltinComponentLibrary).toHaveBeenCalled();
  });

  it("search input calls searchComponents", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue(mockLibrary);
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: [mockLibrary.components[0]],
      total_count: 1,
      categories: mockLibrary.categories,
      tags: mockLibrary.tags,
    });

    render(<ComponentLibraryScreen />);
    await waitFor(() => screen.getByText("Generic Resistor"));

    const searchInput = screen.getByPlaceholderText("Search components...");
    await userEvent.type(searchInput, "resistor");
    const searchButton = screen.getByRole("button", { name: "Search" });
    await userEvent.click(searchButton);

    await waitFor(() => {
      expect(api.backend.searchComponents).toHaveBeenCalled();
    });
  });

  it("component table displays components", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue(mockLibrary);
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: mockLibrary.components,
      total_count: mockLibrary.components.length,
      categories: mockLibrary.categories,
      tags: mockLibrary.tags,
    });

    render(<ComponentLibraryScreen />);
    await waitFor(() => {
      expect(screen.getByText("Generic Resistor")).toBeInTheDocument();
      expect(screen.getByText("Generic Capacitor")).toBeInTheDocument();
    });
  });

  it("selecting component calls getComponentDetails", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue(mockLibrary);
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: mockLibrary.components,
      total_count: mockLibrary.components.length,
      categories: mockLibrary.categories,
      tags: mockLibrary.tags,
    });
    vi.spyOn(api.backend, "getComponentDetails").mockResolvedValue(mockDetails);

    render(<ComponentLibraryScreen />);
    await waitFor(() => screen.getByRole("cell", { name: "Generic Resistor" }));

    const row = screen.getByRole("cell", { name: "Generic Resistor" }).closest("tr");
    if (row) await userEvent.click(row);

    await waitFor(() => {
      expect(api.backend.getComponentDetails).toHaveBeenCalledWith("generic_resistor");
    });
  });

  it("details panel displays parameters", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue(mockLibrary);
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: mockLibrary.components,
      total_count: mockLibrary.components.length,
      categories: mockLibrary.categories,
      tags: mockLibrary.tags,
    });
    vi.spyOn(api.backend, "getComponentDetails").mockResolvedValue(mockDetails);

    render(<ComponentLibraryScreen />);
    await waitFor(() => screen.getByRole("cell", { name: "Generic Resistor" }));

    const row = screen.getByRole("cell", { name: "Generic Resistor" }).closest("tr");
    if (row) await userEvent.click(row);

    await waitFor(() => {
      expect(screen.getByText("Parameters")).toBeInTheDocument();
      expect(screen.getByText("resistance")).toBeInTheDocument();
    });
  });

  it("empty state renders without crash", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockResolvedValue({
      ...mockLibrary,
      components: [],
    });
    vi.spyOn(api.backend, "searchComponents").mockResolvedValue({
      components: [],
      total_count: 0,
      categories: [],
      tags: [],
    });

    render(<ComponentLibraryScreen />);
    await waitFor(() => {
      expect(screen.getByText("No components found.")).toBeInTheDocument();
    });
  });

  it("error state renders readable message", async () => {
    vi.spyOn(api.backend, "loadBuiltinComponentLibrary").mockRejectedValue(
      new Error("Network error"),
    );

    render(<ComponentLibraryScreen />);
    await waitFor(() => {
      expect(screen.getByText("Network error")).toBeInTheDocument();
    });
  });
});
