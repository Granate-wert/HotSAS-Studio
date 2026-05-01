import { ScrollArea } from '@mantine/core';

export function PreBlock({ text }: { text: string }) {
  return (
    <ScrollArea className="pre-scroll">
      <pre>{text}</pre>
    </ScrollArea>
  );
}
