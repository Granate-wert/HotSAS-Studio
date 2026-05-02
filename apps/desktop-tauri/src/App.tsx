import { AppShell, Divider, MantineProvider, NavLink, Stack, Text, Title } from "@mantine/core";
import { useState } from "react";
import { Workbench } from "./components/Workbench";
import { navigationItems, type ScreenId } from "./screens/navigation";

export default function App() {
  const [activeScreen, setActiveScreen] = useState<ScreenId>("start");

  return (
    <MantineProvider defaultColorScheme="dark">
      <AppShell navbar={{ width: 250, breakpoint: "sm" }} padding={0}>
        <AppShell.Navbar className="navbar">
          <Stack gap="xs" p="md">
            <Title order={3}>HotSAS Studio</Title>
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
          <Workbench activeScreen={activeScreen} />
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}
