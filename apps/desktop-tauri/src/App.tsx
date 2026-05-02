import {
  AppShell,
  Divider,
  Group,
  MantineProvider,
  NavLink,
  Stack,
  Text,
  Title,
} from "@mantine/core";
import { useState } from "react";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { DebugLogPanel } from "./components/DebugLogPanel";
import { Workbench } from "./components/Workbench";
import { navigationItems, type ScreenId } from "./screens/navigation";

export default function App() {
  const [activeScreen, setActiveScreen] = useState<ScreenId>("start");

  return (
    <MantineProvider defaultColorScheme="dark">
      <AppShell navbar={{ width: 250, breakpoint: "sm" }} padding={0}>
        <AppShell.Navbar className="navbar">
          <Stack gap="xs" p="md">
            <Group justify="space-between" align="center">
              <Title order={3}>HotSAS Studio</Title>
              <DebugLogPanel />
            </Group>
            <Text size="xs" c="dimmed">
              Hardware-Oriented Schematic Analysis & Simulation Studio
            </Text>
          </Stack>
          <Divider />
          {navigationItems.map((item) => (
            <NavLink
              key={item.id}
              label={item.label}
              leftSection={item.icon}
              active={activeScreen === item.id}
              onClick={() => setActiveScreen(item.id)}
            />
          ))}
        </AppShell.Navbar>
        <AppShell.Main className="main">
          <ErrorBoundary>
            <Workbench activeScreen={activeScreen} />
          </ErrorBoundary>
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}
