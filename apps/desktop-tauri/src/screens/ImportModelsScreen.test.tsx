import { screen, fireEvent, waitFor } from "@testing-library/react";
import { render } from "../test-utils";
import { ImportModelsScreen } from "./ImportModelsScreen";
import { useHotSasStore } from "../store";
import { backend } from "../api";

vi.mock("../api", async () => {
  const actual = await import("../api/__mocks__");
  return { backend: actual.backend };
});

describe("ImportModelsScreen", () => {
  beforeEach(() => {
    useHotSasStore.setState({
      spiceImportReport: null,
      touchstoneImportReport: null,
      importedModels: [],
      selectedImportedModel: null,
      busy: false,
      error: null,
    });
  });

  it("renders tabs for SPICE, Touchstone, and Library", () => {
    const { container } = render(<ImportModelsScreen />);
    expect(container.querySelector(".screen-panel")).toBeInTheDocument();
    expect(container.querySelector(".screen-content")).toBeInTheDocument();
    expect(screen.getByText("SPICE Model / Subcircuit")).toBeInTheDocument();
    expect(screen.getByText("Touchstone S-Parameters")).toBeInTheDocument();
    expect(screen.getByText("Imported Library")).toBeInTheDocument();
  });

  it("shows empty library message when no models imported", () => {
    render(<ImportModelsScreen />);
    fireEvent.click(screen.getByText("Imported Library"));
    expect(screen.getByText(/No imported models yet/i)).toBeInTheDocument();
  });

  it("imports SPICE content and shows report", async () => {
    render(<ImportModelsScreen />);
    const textarea = screen.getByPlaceholderText(/\.model/);
    fireEvent.change(textarea, { target: { value: ".model D1 D()" } });
    fireEvent.click(screen.getByText("Import SPICE"));

    await waitFor(() => {
      expect(screen.getByText(/Import Result/)).toBeInTheDocument();
    });
  });

  it("imports Touchstone content and shows report", async () => {
    render(<ImportModelsScreen />);
    fireEvent.click(screen.getByText("Touchstone S-Parameters"));
    const textarea = screen.getByPlaceholderText(/# GHz/);
    fireEvent.change(textarea, { target: { value: "# GHz S MA R 50\n1.0 0.9 -0.1" } });
    fireEvent.click(screen.getByText("Import Touchstone"));

    await waitFor(() => {
      expect(screen.getByText(/Import Result/)).toBeInTheDocument();
    });
  });

  it("refresh list loads imported models", async () => {
    vi.spyOn(backend, "listImportedModels").mockResolvedValue([
      {
        id: "model-1",
        kind: "SpiceModel",
        name: "1N4148",
        source_format: "spice",
      },
    ]);

    render(<ImportModelsScreen />);
    fireEvent.click(screen.getByText("Imported Library"));
    fireEvent.click(screen.getByText("Refresh List"));

    expect(await screen.findByText("1N4148")).toBeInTheDocument();
  });

  it("selecting a model from list shows details", async () => {
    vi.spyOn(backend, "listImportedModels").mockResolvedValue([
      {
        id: "model-1",
        kind: "SpiceModel",
        name: "1N4148",
        source_format: "spice",
      },
    ]);
    vi.spyOn(backend, "getImportedModel").mockResolvedValue({
      id: "model-1",
      kind: "SpiceModel",
      name: "1N4148",
      source_format: "spice",
      spice_model: {
        id: "sm-1",
        name: "1N4148",
        kind: "Diode",
        parameters: [],
        warnings: [],
      },
      spice_subcircuit: null,
      touchstone_summary: null,
    });

    render(<ImportModelsScreen />);
    fireEvent.click(screen.getByText("Imported Library"));
    fireEvent.click(screen.getByText("Refresh List"));

    const row = await screen.findByTestId("model-row-model-1");
    fireEvent.click(row);

    expect(await screen.findByText(/Model Details/)).toBeInTheDocument();
    expect(screen.getByTestId("spice-model-details")).toBeInTheDocument();
  });
});
