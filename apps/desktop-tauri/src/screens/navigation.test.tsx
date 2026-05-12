import { describe, expect, it } from "vitest";
import { navigationItems } from "./navigation";

describe("navigationItems", () => {
  it("uses the documented Component Library label", () => {
    const componentLibrary = navigationItems.find((item) => item.id === "components");

    expect(componentLibrary?.label).toBe("Component Library");
  });
});
