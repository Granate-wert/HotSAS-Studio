import { Title } from "@mantine/core";
import { EngineeringNotebook } from "../components/notebook/EngineeringNotebook";

export function CalculatorScreen() {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Engineering Notebook</Title>
        <EngineeringNotebook />
      </div>
    </section>
  );
}
