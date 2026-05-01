import { Title } from '@mantine/core';
import { FormulaPanel } from '../components/FormulaPanel';

export function CalculatorScreen() {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Engineering Notebook</Title>
        <FormulaPanel />
      </div>
    </section>
  );
}
