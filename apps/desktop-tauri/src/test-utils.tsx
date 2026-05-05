import { MantineProvider } from "@mantine/core";
import { render as rtlRender, type RenderOptions, screen, fireEvent } from "@testing-library/react";
import type { ReactElement, ReactNode } from "react";

function AllTheProviders({ children }: { children: ReactNode }) {
  return <MantineProvider defaultColorScheme="dark">{children}</MantineProvider>;
}

export function render(ui: ReactElement, options?: Omit<RenderOptions, "wrapper">) {
  return rtlRender(ui, { wrapper: AllTheProviders, ...options });
}

export { screen, fireEvent };
