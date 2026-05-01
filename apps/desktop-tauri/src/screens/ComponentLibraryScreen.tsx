import { Title } from '@mantine/core';
import { LibraryPanel } from '../components/LibraryPanel';

export function ComponentLibraryScreen() {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>E Component Library</Title>
        <LibraryPanel />
      </div>
    </section>
  );
}
