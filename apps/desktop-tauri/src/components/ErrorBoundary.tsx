import { Alert, Button, Stack, Text, Title } from "@mantine/core";
import { Component, type ErrorInfo, type ReactNode } from "react";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null, errorInfo: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null };
  }

  override componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("ErrorBoundary caught an error:", error, errorInfo);
    this.setState({ error, errorInfo });
  }

  private handleReset = () => {
    this.setState({ hasError: false, error: null, errorInfo: null });
  };

  override render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <Stack gap="md" p="xl">
          <Title order={3} c="red">
            Something went wrong
          </Title>
          <Alert color="red" variant="light">
            <Text size="sm" style={{ whiteSpace: "pre-wrap" }}>
              {this.state.error?.message ?? "Unknown error"}
            </Text>
          </Alert>
          {this.state.errorInfo ? (
            <Text size="xs" c="dimmed" style={{ whiteSpace: "pre-wrap" }}>
              {this.state.errorInfo.componentStack}
            </Text>
          ) : null}
          <Button onClick={this.handleReset} variant="light">
            Try again
          </Button>
        </Stack>
      );
    }

    return this.props.children;
  }
}
