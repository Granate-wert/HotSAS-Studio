import { SimulationDashboard } from "../components/simulation/SimulationDashboard";

export function SimulationScreen() {
  return (
    <section className="screen-panel">
      <div className="screen-content wide">
        <SimulationDashboard />
      </div>
    </section>
  );
}
