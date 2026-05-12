import { describe, expect, it } from "vitest";
const { readFileSync } = await import(
  // @ts-expect-error Vitest runs in Node, but the app tsconfig intentionally omits Node types.
  "node:fs"
);

const styles = readFileSync("src/styles.css", "utf8");

describe("global layout styles", () => {
  it("do not force a desktop-only body width", () => {
    expect(styles).not.toMatch(/body\s*{[^}]*min-width:\s*1100px/s);
  });

  it("allow the top toolbar to wrap within narrow desktop windows", () => {
    expect(styles).toMatch(/\.toolbar\s*{[^}]*flex-wrap:\s*wrap/s);
  });
});
