import { Title } from '@mantine/core';
import { FormulaPanel } from '../components/FormulaPanel';

export function FormulaLibraryScreen() {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Formula Library</Title>
        <FormulaPanel />
      </div>
    </section>
  );
}
